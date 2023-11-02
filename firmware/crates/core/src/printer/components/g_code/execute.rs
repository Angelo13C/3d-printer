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
	current_command: Option<Box<dyn GCodeCommand<P>>>,
	/// The commands are stored in reverse order (the first one to be executed is the last one present in this Vec).
	command_buffer_reversed: Vec<Box<dyn GCodeCommand<P>>>,

	position_mode: PositionMode,
	extruder_position_mode: PositionMode,

	saved_positions: [VectorN<4>; SAVED_POSITIONS_COUNT],
}

impl<P: Peripherals> GCodeExecuter<P>
{
	pub fn tick(&mut self, printer_components: &mut Printer3DComponents<P>)
	{
		if self.current_command.is_none() && !self.command_buffer_reversed.is_empty()
		{
			self.current_command = Some(
				self.command_buffer_reversed
					.remove(self.command_buffer_reversed.len() - 1),
			);
		}

		if let Some(mut command) = self.current_command.take()
		{
			let status = command.execute(printer_components, self);

			if status == Status::Working
			{
				self.current_command = Some(command);
			}
		}
	}

	/// Returns `true` if it is currently executing a command or it has at least 1 command in its command buffer.
	/// Otherwise returns `false`.
	pub fn has_command_to_execute(&self) -> bool
	{
		self.current_command.is_some() || !self.command_buffer_reversed.is_empty()
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
		self.command_buffer_reversed.insert(0, command);
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

impl<P: Peripherals> Default for GCodeExecuter<P>
{
	fn default() -> Self
	{
		Self {
			current_command: Default::default(),
			command_buffer_reversed: Vec::with_capacity(50),
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
