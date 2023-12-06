use std::fmt::Debug;

use super::super::homing::endstop::Endstop;
use crate::utils::math::vectors::Vector3;

/// A probe used on the Z axis to home/do bed levelling with an offset from the nozzle.
pub struct Probe<P: ZAxisProbe>
{
	inner_probe: P,
	offset_from_nozzle: Vector3,
}

impl<P: ZAxisProbe> Probe<P>
{
	pub fn new(inner_probe: P, offset_from_nozzle: Vector3) -> Self
	{
		Self {
			inner_probe,
			offset_from_nozzle,
		}
	}

	/// Returns the value of `offset_from_nozzle` you passed to [`Self::new`].
	pub fn get_offset_from_nozzle(&self) -> Vector3
	{
		self.offset_from_nozzle
	}
}

impl<P: ZAxisProbe> Endstop for Probe<P>
{
	type IsEndReachedError = P::IsEndReachedError;
	type OnEndReachedError = P::OnEndReachedError;
	type HomingError = P::HomingError;

	fn is_end_reached(&self) -> Result<bool, Self::IsEndReachedError>
	{
		self.inner_probe.is_end_reached()
	}

	unsafe fn on_end_reached(&mut self, callback: impl FnMut() + Send + 'static)
		-> Result<(), Self::OnEndReachedError>
	{
		self.inner_probe.on_end_reached(callback)
	}

	fn prepare_for_homing(&mut self) -> Result<(), Self::HomingError>
	{
		self.inner_probe.prepare_for_homing()
	}

	fn finish_homing(&mut self) -> Result<(), Self::HomingError>
	{
		self.inner_probe.finish_homing()
	}
}

pub trait ZAxisProbe
{
	type IsEndReachedError: Debug;
	type OnEndReachedError: Debug;
	type HomingError: Debug;

	fn is_end_reached(&self) -> Result<bool, Self::IsEndReachedError>;

	/// # Safety
	/// The `callback` will be called in an ISR context.
	unsafe fn on_end_reached(&mut self, callback: impl FnMut() + Send + 'static)
		-> Result<(), Self::OnEndReachedError>;

	/// Prepare the probe for homing.
	fn prepare_for_homing(&mut self) -> Result<(), Self::HomingError>;

	/// The probe finished homing.
	fn finish_homing(&mut self) -> Result<(), Self::HomingError>;
}
