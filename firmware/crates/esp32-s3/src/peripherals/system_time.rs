use std::time::Duration;

use esp_idf_hal::delay::Delay;
use esp_idf_svc::timer::EspTaskTimerService;
use esp_idf_sys::EspError;
use firmware_core::printer::components::time::SystemTime as SystemTimeTrait;

pub struct SystemTime(EspTaskTimerService);

impl SystemTime
{
	pub fn new() -> Result<Self, EspError>
	{
		Ok(Self(EspTaskTimerService::new()?))
	}
}

impl SystemTimeTrait for SystemTime
{
	fn now(&self) -> Duration
	{
		self.0.now()
	}

	fn delay(&self, duration: Duration)
	{
		// Delay at least 1us even if the duration is only 10ns
		Delay::new(u32::MAX).delay_us(duration.subsec_micros().max(1))
	}
}
