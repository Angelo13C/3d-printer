use esp_idf_hal::task::watchdog::*;
use esp_idf_sys::EspError;
use firmware_core::printer::components::hal::watchdog::{
	Watchdog as WatchdogTrait, WatchdogCreator as WatchdogCreatorTrait,
};

#[derive(Clone)]
pub struct WatchdogCreator(pub TWDTDriver<'static>);

impl WatchdogCreatorTrait for WatchdogCreator
{
	type Watchdog = Watchdog;

	fn watch_current_thread(self) -> Option<Self::Watchdog>
	{
		let driver = Box::leak(Box::new(self.0));
		if let Ok(watchdog) = driver.watch_current_task()
		{
			Some(Watchdog(watchdog))
		}
		else
		{
			None
		}
	}
}

pub struct Watchdog(WatchdogSubscription<'static>);

impl WatchdogTrait for Watchdog
{
	type Error = EspError;

	fn feed(&mut self) -> Result<(), Self::Error>
	{
		self.0.feed()
	}
}
