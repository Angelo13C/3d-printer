use std::fmt::Debug;

/// Type that can be used as an [`endstop`].
///
/// [`endstop`]: <https://reprap.org/wiki/Endstop>
pub trait Endstop
{
	type IsEndReachedError;
	type OnEndReachedError;
	type HomingError: Debug;

	/// Check if the endstop is currently triggered or not.
	///
	/// Returns `Err(Self::IsEndReachedError)` if there has been an error in reading the endstop's state,
	/// otherwise returns `Ok(is_currently_triggered)`.
	fn is_end_reached(&self) -> Result<bool, Self::IsEndReachedError>;

	/// Execute the provided `callback` when the endstop is triggered.
	///
	/// # Safety
	/// The `callback` will be called in an ISR context.
	unsafe fn on_end_reached(&mut self, callback: impl FnMut() + Send + 'static) -> Result<(), Self::OnEndReachedError>;

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

/// A "fake" [`Endstop`] that is not physically present in the machine, but it has to be triggered via software
/// calling [`ManualEndstop::trigger`].
///
/// This could be useful for testing purposes.
///
/// # Examples
/// ```
/// # use firmware_core::printer::components::motion::homing::endstop::*;
/// # use std::sync::{atomic::{Ordering, AtomicBool}, Arc};
/// #
/// let mut endstop = ManualEndstop::new();
/// let triggered = Arc::new(AtomicBool::new(false));
/// let triggered_clone = Arc::clone(&triggered);
/// endstop.trigger();
/// // Nothing happens because no callback has been registered yet
/// assert_eq!(triggered.load(Ordering::Relaxed), false);
///
/// unsafe { endstop.on_end_reached(move || triggered_clone.store(true, Ordering::Relaxed)) };
/// endstop.trigger();
///
/// assert_eq!(triggered.load(Ordering::Relaxed), true);
/// ```
#[derive(Default)]
pub struct ManualEndstop
{
	callback: Option<Box<dyn FnMut()>>,
	is_triggered: bool,
}
impl ManualEndstop
{
	/// Returns a [`ManualEndstop`] without any callback registered.
	///
	/// To set the callback function you should use [`ManualEndstop::on_end_reached`], and to trigger
	/// the callback use [`ManualEndstop::trigger`].
	///
	/// Check the [`struct's doc`] for an example.
	///
	/// [`struct's doc`]: `ManualEndstop`
	pub fn new() -> Self
	{
		Self::default()
	}

	/// Sets the value returned by [`Self::is_end_reached`] to `true`.
	///
	/// If a callback has been registered, it will be called.
	///
	/// Check the [`struct's doc`] for an example.
	///
	/// [`struct's doc`]: `ManualEndstop`
	pub fn trigger(&mut self)
	{
		self.is_triggered = true;
		if let Some(callback) = self.callback.as_mut()
		{
			(callback)()
		}
	}

	/// Sets the value returned by [`Self::is_end_reached`] to `false`.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::printer::components::motion::homing::endstop::*;
	/// #
	/// let mut endstop = ManualEndstop::new();
	///
	/// assert_eq!(endstop.is_end_reached(), Ok(false));
	///
	/// // This won't call any callback since there's no callback registered but it will set the `is_end_reached` return value to true.
	/// endstop.trigger();
	/// assert_eq!(endstop.is_end_reached(), Ok(true));
	///
	/// endstop.remove_trigger();
	/// assert_eq!(endstop.is_end_reached(), Ok(false));
	/// ```
	pub fn remove_trigger(&mut self)
	{
		self.is_triggered = false;
	}
}

// Safety: the callback is actually not called in an ISR context, but it's triggered manually by the user
// (by calling [`ManualEndstop::trigger`]). So it's safe.
impl Endstop for ManualEndstop
{
	type IsEndReachedError = ();
	type OnEndReachedError = ();
	type HomingError = ();

	/// # Safety
	/// The callback is actually not called in an ISR context, but it's triggered manually by the user
	/// (by calling [`ManualEndstop::trigger`]). So calling this function is always safe.
	unsafe fn on_end_reached(&mut self, callback: impl FnMut() + 'static) -> Result<(), Self::OnEndReachedError>
	{
		self.callback = Some(Box::new(callback));

		Ok(())
	}

	fn is_end_reached(&self) -> Result<bool, Self::IsEndReachedError>
	{
		Ok(self.is_triggered)
	}
}
