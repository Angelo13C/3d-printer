use std::sync::Arc;

use super::command::CommandsSender;
use crate::{
	printer::{
		communication::security::Security,
		components::{file_system::FileSystem, Peripherals},
	},
	utils::mutex::{Mutex, MutexGuard},
};

pub struct Resources<P: Peripherals>(Arc<Mutex<ResourcesImpl<P>>>);

pub struct ResourcesImpl<P: Peripherals>
{
	pub file_system: FileSystem<P::FlashChip, P::FlashSpi>,
	pub security: Security,
	pub command_sender: CommandsSender<P>,
}

impl<P: Peripherals> Resources<P>
{
	pub fn new(
		file_system: FileSystem<P::FlashChip, P::FlashSpi>, security: Security, command_sender: CommandsSender<P>,
	) -> Self
	{
		Self(Arc::new(Mutex::new(ResourcesImpl {
			file_system,
			security,
			command_sender,
		})))
	}

	pub fn try_lock(&self) -> Option<MutexGuard<'_, ResourcesImpl<P>>>
	{
		self.0.try_lock()
	}
}

impl<P: Peripherals> Clone for Resources<P>
{
	fn clone(&self) -> Self
	{
		Self(Arc::clone(&self.0))
	}
}
