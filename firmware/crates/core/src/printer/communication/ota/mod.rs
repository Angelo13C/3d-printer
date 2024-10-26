//! This module provides functionality for Over-The-Air (OTA) updates for the firmware.
//!
//! OTA updates allow for the remote updating of the firmware without requiring physical access to the device.
//! For more information on OTA updates, see [`Over-the-air update`](https://en.wikipedia.org/wiki/Over-the-air_update).

use embedded_svc::ota::Ota as OtaTrait;
pub use update::*;

mod update;

/// Struct that manages OTA updates for the firmware.
pub struct OverTheAirUpdater<Ota: OtaTrait>
{
	ota: Ota,
	reboot_fn: fn(),
}

impl<Ota: OtaTrait> OverTheAirUpdater<Ota>
{
	/// Creates a new instance of `OverTheAirUpdater`.
	///
	/// This function initializes the OTA update handler with the provided OTA trait and reboot function.
	///
	/// # Parameters
	/// - `ota`: An implementation of the `Ota` trait to manage the OTA process.
	/// - `reboot_fn`: A function pointer that will be called to reboot the microcontroller after an update.
	///
	/// # Returns
	/// Returns a new instance of `OverTheAirUpdater`.
	pub fn new(ota: Ota, reboot_fn: fn()) -> Self
	{
		Self { ota, reboot_fn }
	}

	/// Initiates an OTA update.
	///
	/// This function prepares the OTA update process, indicating the size of the update in bytes.
	///
	/// # Parameters
	/// - `update_size_in_bytes`: The size of the update to be downloaded.
	///
	/// # Returns
	/// Returns an instance of `OverTheAirUpdate` if the initiation is successful, or an error if it fails.
	pub fn initiate_update(&mut self, update_size_in_bytes: usize) -> Result<OverTheAirUpdate<'_, Ota>, Ota::Error>
	{
		self.ota.initiate_update().map(|ota| OverTheAirUpdate {
			update: ota,
			update_size_in_bytes,
			current_written_bytes: 0,
		})
	}

	/// Checks if an OTA update has been initiated and completed.
	///
	/// This function returns `true` if an update has been started and completed; otherwise, it returns `false`.
	///
	/// # Returns
	/// - `true` if the update has completed, `false` otherwise.
	pub fn needs_reboot(&self) -> bool
	{
		update::has_completed_update()
	}

	/// Reboots the microcontroller.
	///
	/// This function is typically called after a successful OTA update to run the new firmware.
	///
	/// # Warning
	/// Rebooting will result in the loss of all variable states. Ensure that any necessary state is saved
	/// before calling this function.
	pub fn reboot(&self)
	{
		(self.reboot_fn)()
	}
}
