use std::sync::Arc;

use super::{command::CommandsSender, other::GCodeHistory};
use crate::{
	printer::{
		communication::{ota::OverTheAirUpdater, security::Security},
		components::{file_system::FileSystem, print_process::PrintProcess, Peripherals},
	},
	utils::mutex::{Mutex, MutexGuard},
};

/// A container of resources that can be used by the [`callbacks`] of the http requests and also by the
/// [`Communication`] struct. Internally it's simply a [`ResourcesImpl`] wrapped in an `Arc<Mutex>`.
///
/// [`callbacks`]: super::request
/// [`Communication`]: super::super::Communication
pub struct Resources<P: Peripherals>(Arc<Mutex<ResourcesImpl<P>>>);

/// Check [`Resources`].
pub struct ResourcesImpl<P: Peripherals>
{
	pub system_time: Option<P::SystemTime>,
	pub file_system: FileSystem<P::FlashChip, P::FlashSpi>,
	pub ota_updater: OverTheAirUpdater<P::Ota>,
	pub security: Option<Security>,
	pub command_sender: CommandsSender<P>,
	pub print_process: PrintProcess<P>,

	pub g_code_history: GCodeHistory,
}

impl<P: Peripherals> Resources<P>
{
	/// Wraps the provided resources in an `Arc<Mutex>>` and returns the resulting [`Resources`].
	pub fn new(
		system_time: Option<P::SystemTime>, file_system: FileSystem<P::FlashChip, P::FlashSpi>,
		ota_updater: OverTheAirUpdater<P::Ota>, security: Security, command_sender: CommandsSender<P>,
		print_process: PrintProcess<P>,
	) -> Self
	{
		log::info!("Create the resources used in the HTTP uri handlers");

		Self(Arc::new(Mutex::new(ResourcesImpl {
			system_time,
			file_system,
			ota_updater,
			security: Some(security),
			command_sender,
			print_process,
			g_code_history: GCodeHistory::new(),
		})))
	}

	/// Try to get the internal resources, returning `None` if they already currently being used, or `Some(...)`
	/// if they can be used.
	pub fn try_lock(&self) -> Option<MutexGuard<'_, ResourcesImpl<P>>>
	{
		self.0.try_lock()
	}

	/// Blocks the thread until the mutex is unlocked and returns a [`MutexGuard`] with the protected value
	/// (ResourcesImpl).
	pub fn lock(&self) -> MutexGuard<'_, ResourcesImpl<P>>
	{
		self.0.lock()
	}
}

impl<P: Peripherals> ResourcesImpl<P>
{
	pub fn get_file_system_and_print_process(
		&mut self,
	) -> (&mut FileSystem<P::FlashChip, P::FlashSpi>, &mut PrintProcess<P>)
	{
		(&mut self.file_system, &mut self.print_process)
	}
}

impl<P: Peripherals> Clone for Resources<P>
{
	/// Makes a clone of the [`Arc`] pointer.
	fn clone(&self) -> Self
	{
		Self(Arc::clone(&self.0))
	}
}
