use embedded_svc::ota::Ota as OtaTrait;
pub use update::*;

mod update;

/// Makes [`OTA updates`] possible for this firmware.
///
/// [`OTA updates`]: https://en.wikipedia.org/wiki/Over-the-air_update
pub struct OverTheAirUpdater<Ota: OtaTrait>
{
	ota: Ota,
	reboot_fn: fn(),
}

impl<Ota: OtaTrait> OverTheAirUpdater<Ota>
{
	/// Returns a new handler for OTA updates that can reboot the microcontroller using the provided `reboot_fn`.
	///
	/// Check [`Self`] for more details.
	pub fn new(ota: Ota, reboot_fn: fn()) -> Self
	{
		Self { ota, reboot_fn }
	}

	/// Starts an [`OverTheAirUpdate`].
	pub fn initiate_update(&mut self, update_size_in_bytes: usize) -> Result<OverTheAirUpdate<'_, Ota>, Ota::Error>
	{
		self.ota.initiate_update().map(|ota| OverTheAirUpdate {
			update: ota,
			update_size_in_bytes,
			current_written_bytes: 0,
		})
	}

	/// Returns `true` if you [`initiated an update`] before and it has [`completed`], otherwise
	/// returns `false`.
	///
	/// [`initiated an update`]: Self::initiate_update
	/// [`completed`]: OverTheAirUpdate::complete
	pub fn needs_reboot(&self) -> bool
	{
		update::has_completed_update()
	}

	/// Reboots the microcontroller. It may be useful after an OTA update is completed to run the newly
	/// installed firmware.
	///
	/// # Warning
	/// Naturally rebooting the microcontroller means the value of all the variables is lost. So save the state
	/// of everything you need before rebooting.
	pub fn reboot(&self)
	{
		(self.reboot_fn)()
	}
}
