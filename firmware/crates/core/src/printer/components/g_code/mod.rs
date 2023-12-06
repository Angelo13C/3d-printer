//! **G-code** is the language understood by 3D printers, and it's used to make the 3D printer execute
//! commands (like move the tool to a specific location, set the bed's temperature...).
//!
//! A 3D printer slicer software receives a 3D model and converts it to a long list of G-code commands,
//! which are then sent to the printer so that it can print the model.
//!
//! This firmware (for now) only supports the most important G-code commands.
//!
//! For an extensive list of all the existing G-code commands, check the [RepRap's documentation](https://reprap.org/wiki/G-code).

pub mod commands;
pub mod execute;
pub mod parameters;
pub mod parser;

use std::{any::Any, fmt::Debug};

use self::execute::GCodeExecuter;
use super::Peripherals;
use crate::printer::components::Printer3DComponents;

pub trait GCodeCommand<P: Peripherals>: Send + Sync + Debug + AsAny<P>
{
	/// This method is called as soon as the G-code command is parsed from a string and before
	/// it is put in the command buffer of the [`GCodeExecuter`].
	/// When this method returns [`Status::Finished`], the command is inserted in the buffer.
	fn prepare(&mut self, _: &mut Printer3DComponents<P>, _: &mut GCodeExecuter<P>) -> Status
	{
		Status::Finished
	}

	/// This method is called when the G-code command becomes the first command in the command
	/// buffer of the [`GCodeExecuter`].
	/// When this method returns [`Status::Finished`], the command is removed from the buffer.
	fn execute(&mut self, _: &mut Printer3DComponents<P>, _: &mut GCodeExecuter<P>) -> Status
	{
		Status::Finished
	}
}

/// This trait is only used for testing purposes, and it's automatically implemented by every [`GCodeCommand`].
pub trait AsAny<P: Peripherals>
{
	fn as_any(&self) -> &dyn Any;
}
impl<C: GCodeCommand<P> + 'static, P: Peripherals> AsAny<P> for C
{
	fn as_any(&self) -> &dyn Any
	{
		self
	}
}

#[derive(Clone, Debug)]
/// Status of execution (or preparation) of a [`G-code command`].
///
/// [`G-code command`]: GCodeCommand
pub enum Status
{
	/// The command still needs to be processed.
	Working,
	/// The command has finished processing successfully.
	Finished,
	/// There has been an error in the execution of the command.
	Error(String),
}

impl PartialEq for Status
{
	fn eq(&self, other: &Self) -> bool
	{
		core::mem::discriminant(self) == core::mem::discriminant(other)
	}
}

impl Eq for Status {}
