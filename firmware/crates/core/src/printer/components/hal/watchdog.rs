//! Watchdog module.
//!
//! This module provides traits for creating and managing a [`Watchdog`] on the current thread.
//!
//! [`Watchdog`]: <https://en.wikipedia.org/wiki/Watchdog_timer>

use std::fmt::Debug;

/// A type that can instantiate a [`Watchdog`] on the current thread by calling the [`Self::watch_current_thread`] method.
///
/// After you watch a thread, you must regularly feed the returned watchdog; otherwise, the firmware will enter an error state.
pub trait WatchdogCreator
{
	/// The type of watchdog that this creator produces.
	type Watchdog: Watchdog;

	/// Creates a new watchdog for the current thread.
	///
	/// Check the [`trait's`] documentation for more information.
	///
	/// [`trait's`]: Self
	fn watch_current_thread(self) -> Option<Self::Watchdog>;
}

/// A type that can restart the [`watchdog timer`] in the microcontroller by calling [`Self::feed`].
///
/// [`watchdog timer`]: https://en.wikipedia.org/wiki/Watchdog_timer
pub trait Watchdog
{
	/// The type of error that can occur while feeding the watchdog.
	type Error: Debug;

	/// Feeds the watchdog to reset the timer and prevent the firmware from entering an error state.
	///
	/// Check the [`trait's`] documentation for more information.
	///
	/// [`trait's`]: Self
	fn feed(&mut self) -> Result<(), Self::Error>;
}
