use std::collections::VecDeque;

use super::{GCodeCommand, Status};
use crate::{
	printer::components::{Peripherals, Printer3DComponents},
	utils::{
		math::vectors::*,
		measurement::distance::{Distance, Units},
	},
};

pub const SAVED_POSITIONS_COUNT: usize = 1;

pub struct GCodeExecuter<P: Peripherals>
{
	commands_to_prepare: VecDeque<Box<dyn GCodeCommand<P>>>,
	command_buffer: VecDeque<Box<dyn GCodeCommand<P>>>,
	current_command: Option<Box<dyn GCodeCommand<P>>>,

	position_mode: PositionMode,
	extruder_position_mode: PositionMode,

	saved_positions: [VectorN<4>; SAVED_POSITIONS_COUNT],
}

impl<P: Peripherals> GCodeExecuter<P>
{
	pub fn tick(&mut self, printer_components: &mut Printer3DComponents<P>) -> Result<(), TickError>
	{
		if let Some(mut command) = self.commands_to_prepare.pop_front()
		{
			match command.prepare(printer_components, self)
			{
				Status::Working => self.commands_to_prepare.push_front(command),
				Status::Finished => self.command_buffer.push_back(command),
				Status::Error(error) => return Err(TickError::PreparingCommand(error)),
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
				Status::Working => self.current_command = Some(command),
				Status::Finished => (),
				Status::Error(error) => return Err(TickError::ExecutingCommand(error)),
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

	pub fn save_position(&mut self, position: VectorN<4>, slot: usize) -> Result<(), InvalidPositionSlot>
	{
		self.validate_position_slot(slot)?;

		self.saved_positions[slot] = position;

		Ok(())
	}

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
	PreparingCommand(String),
	ExecutingCommand(String),
}

impl<P: Peripherals> Default for GCodeExecuter<P>
{
	fn default() -> Self
	{
		Self {
			commands_to_prepare: VecDeque::with_capacity(120),
			command_buffer: VecDeque::with_capacity(100),
			current_command: Default::default(),
			position_mode: Default::default(),
			extruder_position_mode: Default::default(),
			saved_positions: Default::default(),
		}
	}
}

pub struct InvalidPositionSlot;

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum PositionMode
{
	#[default]
	Absolute,
	Relative,
}
