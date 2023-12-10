use std::convert::Infallible;

use crate::printer::components::hal::watchdog::{Watchdog, WatchdogCreator};

pub struct MockWatchdogCreator;
impl WatchdogCreator for MockWatchdogCreator
{
	type Watchdog = MockWatchdog;

	fn watch_current_thread(self) -> Option<Self::Watchdog>
	{
		Some(MockWatchdog)
	}
}

pub struct MockWatchdog;
impl Watchdog for MockWatchdog
{
	type Error = Infallible;

	fn feed(&mut self) -> Result<(), Self::Error>
	{
		Ok(())
	}
}
