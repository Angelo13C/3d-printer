use std::{fmt::Debug, time::Duration};

use crate::utils::measurement::frequency::Frequency;

/// A timer that can be used to get the current time it is keeping and also to [`call some callback you provide when a certain
/// time is reached`].
///
/// [`call some callback you provide when a certain time is reached`]: `Self::on_alarm`
pub trait Timer
{
	type Error: Debug;
	type AdditionalFunctionality: TimerAdditionalFunctionality;

	/// Returns a [`Self::AdditionalFunctionality`] instance that can be used to get some additional timer
	/// functionality (like setting the alarm time or getting the time kept by the timer).
	///
	/// The reason all of this is separated in 2 traits is because it may be useful in the case
	/// you want to call some of these methods within the `callback` you provide to `Timer::on_alarm`.
	/// This would be impossible if they were part of the same trait (calling this method borrows mutably, so
	/// you can't borrow it anymore in the `callback`).
	fn get_additional_functionality(&self) -> Self::AdditionalFunctionality;

	/// Returns the frequency at which the clock of the timer is running.
	fn get_clock_frequency(&self) -> Frequency;

	/// Calls the provided `callback` every time the alarm time set using [`TimerAdditionalFunctionality::set_alarm`] is reached.
	///
	/// # Safety
	/// The `callback` will be called in an ISR context.
	unsafe fn on_alarm(&mut self, callback: impl FnMut() + 'static) -> Result<(), Self::Error>;

	/// Enable or disable the timer based on the provided `enable` variable.
	///
	/// When the timer is disabled it won't increase the time it is keeping (which means that it also won't fire alarms).
	fn enable_alarm(&mut self, enable: bool) -> Result<(), Self::Error>;
}

/// Check [`Timer::get_additional_functionality`].
pub trait TimerAdditionalFunctionality: 'static
{
	type Error: Debug;

	/// Calls the `callback` you provided to [`Timer::on_alarm`] when the [`current time`] reaches the specified `time`.
	///
	/// [`current time`]: `Self::get_time`
	fn set_alarm(&mut self, time: Duration) -> Result<(), Self::Error>;

	/// Get the current time kept by the timer.
	fn get_time(&self) -> Result<Duration, Self::Error>;
}
