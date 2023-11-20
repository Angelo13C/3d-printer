use std::fmt::Debug;

/// A type that can instantiate a [`Watchdog`] on the current thread calling the [`Self::watch_current_thread`] method.
///
/// After you watch a thread you must regularly feed the returned watchdog, otherwise the firmware will go in an error state.
pub trait WatchdogCreator
{
	type Watchdog: Watchdog;

	/// Check the [`trait's`] documentation.
	///
	/// [`trait's`]: Self
	fn watch_current_thread(self) -> Option<Self::Watchdog>;
}

/// A type that can restarts the [`watchdog timer`] in the microcontroller calling [`Self::feed`].
///
/// [`watchdog timer`]: https://en.wikipedia.org/wiki/Watchdog_timer
pub trait Watchdog
{
	type Error: Debug;

	/// Check the [`trait's`] documentation.
	///
	/// [`trait's`]: Self
	fn feed(&mut self) -> Result<(), Self::Error>;
}
