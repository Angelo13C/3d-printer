use std::sync::atomic::{AtomicBool, Ordering};

use embedded_io::{ErrorType, Write};
use embedded_svc::ota::{Ota as OtaTrait, OtaUpdate};

static HAS_COMPLETED_UPDATE: AtomicBool = AtomicBool::new(false);

/// A firmware update [`started`] by the `OverTheAirUpdater`.
///
/// [`started`]: super::OverTheAirUpdater
pub struct OverTheAirUpdate<'a, Ota: OtaTrait + 'a>(pub(super) Ota::Update<'a>);

impl<'a, Ota: OtaTrait> OverTheAirUpdate<'a, Ota>
{
	/// Writes a new portion of data to the OTA partition. After all the data has been written call [`Self::complete`].
	pub fn write(&mut self, data: &[u8]) -> Result<(), <Ota as ErrorType>::Error>
	{
		self.0.write_all(data)
	}

	/// States to the firmware that the OTA update you started before has been aborted.
	pub fn abort(self) -> Result<(), <Ota as ErrorType>::Error>
	{
		self.0.abort()
	}

	/// States to the firmware that the OTA update you started before has been succesfully [`written`], so the
	/// microcontroller can be [`rebooted`] to actually use the updated version of the firmware.
	///
	/// [`written`]: Self::write
	/// [`rebooted`]: super::OverTheAirUpdater::reboot
	pub fn complete(self) -> Result<(), <Ota as ErrorType>::Error>
	{
		HAS_COMPLETED_UPDATE.store(true, Ordering::Relaxed);
		self.0.complete()
	}
}

pub(super) fn has_completed_update() -> bool
{
	HAS_COMPLETED_UPDATE.load(Ordering::Relaxed)
}
