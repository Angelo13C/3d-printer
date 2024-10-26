use std::sync::atomic::{AtomicBool, Ordering};

use embedded_io::{ErrorType, Write};
use embedded_svc::ota::{Ota as OtaTrait, OtaUpdate};

use crate::utils::math::Percentage;

static HAS_COMPLETED_UPDATE: AtomicBool = AtomicBool::new(false);

/// A firmware update [`started`] by the `OverTheAirUpdater`.
///
/// [`started`]: super::OverTheAirUpdater
pub struct OverTheAirUpdate<'a, Ota: OtaTrait + 'a>
{
	pub(super) update: Ota::Update<'a>,
	pub(super) update_size_in_bytes: usize,
	pub(super) current_written_bytes: usize,
}

impl<'a, Ota: OtaTrait> OverTheAirUpdate<'a, Ota>
{
	/// Writes a new portion of data to the OTA partition. After all the data has been written call [`Self::complete`].
	pub fn write(&mut self, data: &[u8]) -> Result<Percentage, OverTheAirUpdateWriteError<Ota>>
	{
		self.update
			.write_all(data)
			.map_err(OverTheAirUpdateWriteError::OtaWrite)?;
		self.current_written_bytes += data.len();
		Percentage::from_0_to_1(self.current_written_bytes as f32 / self.update_size_in_bytes as f32)
			.map_err(|_| OverTheAirUpdateWriteError::WrittenMoreBytesThanUpdateSize)
	}

	/// States to the firmware that the OTA update you started before has been aborted.
	pub fn abort(self) -> Result<(), <Ota as ErrorType>::Error>
	{
		self.update.abort()
	}

	/// States to the firmware that the OTA update you started before has been succesfully [`written`], so the
	/// microcontroller can be [`rebooted`] to actually use the updated version of the firmware.
	///
	/// [`written`]: Self::write
	/// [`rebooted`]: super::OverTheAirUpdater::reboot
	pub fn complete(self) -> Result<(), <Ota as ErrorType>::Error>
	{
		HAS_COMPLETED_UPDATE.store(true, Ordering::Relaxed);
		self.update.complete()
	}
}

/// Errors that can occur during the OTA update writing process.
pub enum OverTheAirUpdateWriteError<Ota: OtaTrait>
{
	/// Error occurred during writing to the OTA.
	OtaWrite(<Ota as ErrorType>::Error),
	/// More bytes were written than the update size.
	WrittenMoreBytesThanUpdateSize,
}

impl<Ota: OtaTrait> std::fmt::Debug for OverTheAirUpdateWriteError<Ota>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Self::OtaWrite(arg0) => f.debug_tuple("OtaWrite").field(arg0).finish(),
			Self::WrittenMoreBytesThanUpdateSize => write!(f, "WrittenMoreBytesThanUpdateSize"),
		}
	}
}

/// Checks if the OTA update has been completed.
pub(super) fn has_completed_update() -> bool
{
	HAS_COMPLETED_UPDATE.load(Ordering::Relaxed)
}
