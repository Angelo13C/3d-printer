use std::collections::VecDeque;

use super::{GCodeCommand, Status};
use crate::{
	printer::components::{print_process, Peripherals, Printer3DComponents},
	utils::{
		math::vectors::*,
		measurement::distance::{Distance, Units},
	},
};

/// Number of savable positions in `GCodeExecuter` (check [`GCodeExecuter::save_position`] and [`GCodeExecuter::get_saved_position`]).
pub const SAVED_POSITIONS_COUNT: usize = 1;

pub struct GCodeExecuter<P: Peripherals>
{
	/// All the commands added to the executer first go in this queue to be prepared.
	commands_to_prepare: VecDeque<Box<dyn GCodeCommand<P>>>,
	/// When a command is successfully prepared (it returns [`Status::Finished`]), it goes in this queue.
	command_buffer: VecDeque<Box<dyn GCodeCommand<P>>>,
	/// This is the first command taken from the `command_buffer` queue and it's constantly executed. When the execution is finished
	/// (it returns [`Status::Finished`]) this field becomes `None` and a new command is taken (if possible) from the `command_buffer`.
	current_command: Option<Box<dyn GCodeCommand<P>>>,

	current_command_being_executed_index: u32,

	position_mode: PositionMode,
	extruder_position_mode: PositionMode,

	saved_positions: [VectorN<4>; SAVED_POSITIONS_COUNT],
}

impl<P: Peripherals> GCodeExecuter<P>
{
	pub fn tick(&mut self, printer_components: &mut Printer3DComponents<P>) -> Result<(), TickError>
	{
		print_process::set_commands_in_buffer_count(
			(self.commands_to_prepare.len() + self.command_buffer.len()) as u16,
		);

		let mut prepare_another_command = true;
		while prepare_another_command && (!self.commands_to_prepare.is_empty() || !self.command_buffer.is_empty())
		{
			if let Some(mut command) = self.commands_to_prepare.pop_front()
			{
				match command.prepare(printer_components, self)
				{
					Status::Working =>
					{
						self.commands_to_prepare.push_front(command);

						prepare_another_command = false;
					},
					Status::Finished => self.command_buffer.push_back(command),
					Status::Error(error) => return Err(TickError::PreparingCommand { error }),
				}
			}

			if self.current_command.is_none()
			{
				self.current_command = self.command_buffer.pop_front();
			}

			if let Some(mut command) = self.current_command.take()
			{
				let status = command.execute(printer_components, self);

				match status
				{
					Status::Working =>
					{
						self.current_command = Some(command);

						if self.commands_to_prepare.is_empty()
						{
							prepare_another_command = false;
						}
					},
					Status::Finished =>
					{
						self.current_command_being_executed_index += 1;
					},
					Status::Error(error) => return Err(TickError::ExecutingCommand { error }),
				}
			}
		}

		Ok(())
	}

	/// Returns `true` if it is currently executing a command or it has at least 1 command in its command buffer.
	/// Otherwise returns `false`.
	pub fn has_command_to_execute(&self) -> bool
	{
		self.current_command.is_some() || !self.command_buffer.is_empty() || !self.commands_to_prepare.is_empty()
	}

	pub fn set_units(&mut self, units: Units)
	{
		todo!("{:#?}", units)
	}

	pub fn set_position_mode(&mut self, mode: PositionMode)
	{
		self.position_mode = mode;
		self.extruder_position_mode = mode;
	}

	pub fn set_extruder_position_mode(&mut self, mode: PositionMode)
	{
		self.extruder_position_mode = mode;
	}

	pub fn calculate_position_based_on_mode(
		&self, printer_components: &Printer3DComponents<P>, x: &mut Option<Distance>, y: &mut Option<Distance>,
		z: &mut Option<Distance>, e: &mut Option<Distance>,
	)
	{
		if self.position_mode == PositionMode::Relative
		{
			let position = printer_components
				.motion_controller
				.get_last_planned_move_end_position()
				.unwrap_or_default();
			*x = x.map(|x| x + position[0]);
			*y = y.map(|y| y + position[1]);
			*z = z.map(|z| z + position[2]);
		}
		if self.extruder_position_mode == PositionMode::Relative
		{
			let position = printer_components
				.motion_controller
				.get_last_planned_move_end_position()
				.unwrap_or_default();
			*e = e.map(|e| e + position[3]);
		}
	}

	pub fn add_command_to_buffer(&mut self, command: Box<dyn GCodeCommand<P>>)
	{
		self.commands_to_prepare.push_back(command);
	}

	pub fn add_commands_to_buffer(&mut self, commands: Vec<Box<dyn GCodeCommand<P>>>)
	{
		self.commands_to_prepare.extend(commands);
	}

	/// Save the provided `position` at the specified `slot`, so that you can later retrieve it using [`Self::get_position(slot)`].
	///
	/// Returns `Err(InvalidPositionSlot)` if `slot >= SAVED_POSITIONS_COUNT`.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::{printer::components::{g_code::execute::*, mock::*}, utils::math::vectors::*};
	/// #
	/// let mut g_code_executer = GCodeExecuter::<MockPeripherals>::default();
	///
	/// assert!(g_code_executer.save_position(VectorN::default(), 0).is_ok());
	/// assert!(g_code_executer.save_position(VectorN::default(), 1000).is_err());
	/// ```
	pub fn save_position(&mut self, position: VectorN<4>, slot: usize) -> Result<(), InvalidPositionSlot>
	{
		self.validate_position_slot(slot)?;

		self.saved_positions[slot] = position;

		Ok(())
	}

	/// Get the position you provided to [`Self::save_position`] at the same slot as the one you provided here (or [`VectorN::default`]
	/// if you never saved a position in that slot).
	///
	/// Returns `Err(InvalidPositionSlot)` if `slot >= SAVED_POSITIONS_COUNT`.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::{printer::components::{g_code::execute::*, mock::*}, utils::{measurement::distance::Distance, math::vectors::*}};
	/// #
	/// let mut g_code_executer = GCodeExecuter::<MockPeripherals>::default();
	///
	/// assert_eq!(g_code_executer.get_saved_position(0), Ok(VectorN::default()));
	///
	/// g_code_executer.save_position(VectorN::new([Distance::from_centimeters(3); 4]), 0).unwrap();
	/// assert_eq!(g_code_executer.get_saved_position(0), Ok(VectorN::new([Distance::from_centimeters(3); 4])));
	///
	/// assert_eq!(g_code_executer.get_saved_position(100000), Err(InvalidPositionSlot));
	/// ```
	pub fn get_saved_position(&self, slot: usize) -> Result<VectorN<4>, InvalidPositionSlot>
	{
		self.validate_position_slot(slot)?;

		Ok(self.saved_positions[slot].clone())
	}

	fn validate_position_slot(&self, slot: usize) -> Result<(), InvalidPositionSlot>
	{
		match slot >= self.saved_positions.len()
		{
			true => Err(InvalidPositionSlot),
			false => Ok(()),
		}
	}
}

#[derive(Debug)]
pub enum TickError
{
	/// Calling [`GCodeCommand::prepare`] resulted in the [`Status::Error`] being returned.
	PreparingCommand
	{
		error: String
	},
	/// Calling [`GCodeCommand::execute`] resulted in the [`Status::Error`] being returned.
	ExecutingCommand
	{
		error: String
	},
}

impl<P: Peripherals> Default for GCodeExecuter<P>
{
	fn default() -> Self
	{
		Self {
			commands_to_prepare: VecDeque::with_capacity(120),
			command_buffer: VecDeque::with_capacity(100),
			current_command: Default::default(),
			current_command_being_executed_index: 0,
			position_mode: Default::default(),
			extruder_position_mode: Default::default(),
			saved_positions: Default::default(),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InvalidPositionSlot;

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum PositionMode
{
	#[default]
	Absolute,
	Relative,
}
