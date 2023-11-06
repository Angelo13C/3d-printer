//! This module contains functionalities used to make the `Communication` thread and the `Components` thread talk with each other.

mod sender_and_receiver;

pub use sender_and_receiver::*;

use crate::printer::components::{g_code::GCodeCommand, Peripherals, Printer3DComponents};

/// A command sent from the `Communication` thread to the `Components` thread to be [`executed`].
///
/// [`executed`]: `Self::execute`
pub enum Command<P: Peripherals>
{
	AddGCodeCommandToBuffer(Box<dyn GCodeCommand<P>>),
}

impl<P: Peripherals> Command<P>
{
	pub fn execute(self, components: &mut Printer3DComponents<P>)
	{
		match self
		{
			Command::AddGCodeCommandToBuffer(command) =>
			{
				todo!()
			},
		}
	}
}
