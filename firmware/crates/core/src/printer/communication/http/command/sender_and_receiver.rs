use std::sync::mpsc::{Receiver, SendError, Sender, TryIter};

use super::Command;
use crate::printer::components::Peripherals;

/// The communication thread can send [`commands`] to the [`main thread`] through this struct.
///
/// [`commands`]: Command
/// [`main thread`]: CommandsReceiver
/// [`HttpHandler`]: crate::printer::communication::http::HttpHandler
pub struct CommandsSender<P: Peripherals>
{
	sender: Sender<Command<P>>,
}

impl<P: Peripherals> CommandsSender<P>
{
	/// Returns a [`CommandsSender`] and a [`CommandsReceiver`] linked together.
	///
	/// All the [`commands`] [`sent`] through the `CommandsSender` will be [`received`] by the `CommandsReceiver`.
	///
	/// [`commands`]: Command
	/// [`sent`]: CommandsSender::send_command
	/// [`received`]: CommandsReceiver::iterate_received_commands
	pub fn new() -> (Self, CommandsReceiver<P>)
	{
		let (sender, receiver) = std::sync::mpsc::channel();
		(Self { sender }, CommandsReceiver::<P> { receiver })
	}

	/// Send the provided `command` to the linked [`CommandsReceiver`].
	///
	/// Check [`Self::new`] for more info.
	pub fn send_command(&mut self, command: Command<P>) -> Result<(), SendError<Command<P>>>
	{
		self.sender.send(command)
	}
}

/// List of [`commands`] received from the [`communication thread`] that have to be [`executed`] on the main thread.
///
/// [`commands`]: Command
/// [`executed`]: Command::execute
/// [`communication thread`]: CommandsSender
pub struct CommandsReceiver<P: Peripherals>
{
	receiver: Receiver<Command<P>>,
}

impl<P: Peripherals> CommandsReceiver<P>
{
	/// Returns an iterator that will consume all the commands received from the [`communication thread`]
	///
	/// Check [`Self`] for more info.
	///
	/// [`communication thread`]: CommandsSender
	pub fn iterate_received_commands(&self) -> TryIter<'_, Command<P>>
	{
		self.receiver.try_iter()
	}
}
