//! Timer module.
//!
//! This module provides a trait for timers that can keep track of elapsed time
//! and trigger callbacks when specified alarms are reached. It defines the main
//! `Timer` trait and an additional trait for extended functionality.

use std::{fmt::Debug, time::Duration};

use crate::utils::measurement::frequency::Frequency;

/// A timer that can keep track of elapsed time and call a provided callback when a certain
/// time is reached.
pub trait Timer
{
	/// The type of error that can occur when using the timer.
	type Error: Debug;

	/// Type for additional timer functionality.
	type AdditionalFunctionality: TimerAdditionalFunctionality + Send;

	/// Returns an instance of [`Self::AdditionalFunctionality`] for accessing extended timer
	/// functionalities, like setting alarm times or retrieving the current time.
	///
	/// The separation into two traits allows for calling these methods within the callback
	/// provided to [`Timer::on_alarm`], which would be impossible if they were in the same trait.
	fn get_additional_functionality(&self) -> Self::AdditionalFunctionality;

	/// Returns the frequency at which the clock of the timer is running.
	fn get_clock_frequency(&self) -> Frequency;

	/// Calls the provided `callback` every time the alarm time set using
	/// [`TimerAdditionalFunctionality::set_alarm`] is reached.
	///
	/// # Safety
	/// The `callback` will be called in an ISR context.
	unsafe fn on_alarm(&mut self, callback: impl FnMut() + Send + 'static) -> Result<(), Self::Error>;

	/// Enable or disable the timer based on the provided `enable` variable.
	///
	/// When the timer is disabled, it will not keep track of time, and alarms will not fire.
	fn enable_alarm(&mut self, enable: bool) -> Result<(), Self::Error>;

	/// Retrieves the current alarm value in ticks.
	fn get_alarm_in_ticks(&self) -> Result<u64, Self::Error>;

	/// Retrieves the current alarm as a [`Duration`].
	fn get_alarm(&self) -> Result<Duration, Self::Error>
	{
		let alarm_in_ticks = self.get_alarm_in_ticks()?;
		Ok(ticks_to_duration(alarm_in_ticks, self.get_clock_frequency()))
	}
}

/// Trait for additional timer functionality.
pub trait TimerAdditionalFunctionality: 'static
{
	/// The type of error that can occur in the additional functionality.
	type Error: Debug;

	/// Sets an alarm that calls the provided callback when the current time reaches the specified `time`.
	///
	/// [`current time`]: `Self::get_time`
	fn set_alarm(&mut self, time: Duration) -> Result<(), Self::Error>;

	/// Sets an alarm in ticks.
	fn set_alarm_in_ticks(&mut self, ticks: u64) -> Result<(), Self::Error>;

	/// Gets the current time kept by the timer.
	fn get_time(&self) -> Result<Duration, Self::Error>;

	/// Gets the current time in ticks.
	fn get_time_in_ticks(&self) -> Result<u64, Self::Error>;
}

/// Converts ticks to a [`Duration`] based on the clock frequency.
pub const fn ticks_to_duration(ticks: u64, clock_frequency: Frequency) -> Duration
{
	let clock_frequency = clock_frequency.as_hertz() as u64;
	let whole_seconds = ticks / clock_frequency;
	let subsec_counter = ticks - (whole_seconds * clock_frequency);
	let nanoseconds = subsec_counter * Duration::from_secs(1).as_nanos() as u64 / clock_frequency;
	Duration::new(whole_seconds, nanoseconds as u32)
}

/// Converts a [`Duration`] to a tick counter based on the clock frequency.
pub const fn duration_to_counter(duration: Duration, clock_frequency: Frequency) -> u64
{
	let clock_frequency = clock_frequency.as_hertz() as u64;
	let mut counter = duration.as_secs() * clock_frequency;
	counter += (duration.subsec_nanos() as u64 * clock_frequency) / Duration::from_secs(1).as_nanos() as u64;
	counter
}
