//! This module contains functionalities used to make the `Communication` thread and the `Components` thread talk with each other.

mod sender_and_receiver;

pub use sender_and_receiver::*;

use crate::printer::components::{g_code::GCodeCommand, Peripherals, Printer3DComponents};

/// A command sent from the `Communication` thread to the `Components` thread to be [`executed`].
///
/// [`executed`]: `Self::execute`
pub enum Command<P: Peripherals>
{
	AddGCodeCommandsToBuffer(Vec<Box<dyn GCodeCommand<P>>>),
}

impl<P: Peripherals> Command<P>
{
	/// Executes the command on the provided components.
	pub fn execute(self, components: &mut Printer3DComponents<P>)
	{
		match self
		{
			Command::AddGCodeCommandsToBuffer(commands) =>
			{
				for command in commands
				{
					if let Some(g_code_executer) = components.g_code_executer.as_mut()
					{
						g_code_executer.add_command_to_buffer(command);
					}
				}
			},
		}
	}
}
