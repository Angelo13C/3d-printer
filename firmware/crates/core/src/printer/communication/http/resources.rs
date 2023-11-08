use std::sync::Arc;

use super::{command::CommandsSender, other::GCodeHistory};
use crate::{
	printer::{
		communication::security::Security,
		components::{file_system::FileSystem, print_process::PrintProcess, Peripherals},
	},
	utils::mutex::{Mutex, MutexGuard},
};

pub struct Resources<P: Peripherals>(Arc<Mutex<ResourcesImpl<P>>>);

pub struct ResourcesImpl<P: Peripherals>
{
	pub system_time: Option<P::SystemTime>,
	pub file_system: FileSystem<P::FlashChip, P::FlashSpi>,
	pub security: Security,
	pub command_sender: CommandsSender<P>,
	pub print_process: PrintProcess<P>,

	pub g_code_history: GCodeHistory,
}

impl<P: Peripherals> Resources<P>
{
	pub fn new(
		system_time: Option<P::SystemTime>, file_system: FileSystem<P::FlashChip, P::FlashSpi>, security: Security,
		command_sender: CommandsSender<P>, print_process: PrintProcess<P>,
	) -> Self
	{
		Self(Arc::new(Mutex::new(ResourcesImpl {
			system_time,
			file_system,
			security,
			command_sender,
			print_process,
			g_code_history: GCodeHistory::new(),
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
