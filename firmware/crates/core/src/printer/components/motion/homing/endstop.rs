/// Type that can be used as an [`endstop`].
///
/// [`endstop`]: <https://reprap.org/wiki/Endstop>
pub trait Endstop
{
	type IsEndReachedError;
	type OnEndReachedError;
	type HomingError;

	/// Check if the endstop is currently triggered or not.
	///
	/// Returns `Err(Self::IsEndReachedError)` if there has been an error in reading the endstop's state,
	/// otherwise returns `Ok(is_currently_triggered)`.
	fn is_end_reached(&self) -> Result<bool, Self::IsEndReachedError>;

	/// Execute the provided `callback` when the endstop is triggered.
	///
	/// # Safety
	/// The `callback` will be called in an ISR context.
	unsafe fn on_end_reached(&mut self, callback: impl FnMut() + 'static) -> Result<(), Self::OnEndReachedError>;

	/// Prepare the endstop for homing. Some types of endstop may not require a preparation so there's a
	/// default blank implementation, but others like a probe with the `BLTouch` sensor do.
	fn prepare_for_homing(&mut self) -> Result<(), Self::HomingError>
	{
		Ok(())
	}

	/// The endstop finished homing. Some types of endstop may not need to do anything so there's a
	/// default blank implementation, but others like a probe with the `BLTouch` sensor do.
	fn finish_homing(&mut self) -> Result<(), Self::HomingError>
	{
		Ok(())
	}
}

