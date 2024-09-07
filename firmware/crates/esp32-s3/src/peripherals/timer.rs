use std::{fmt::Debug, time::Duration};

use esp_idf_hal::{
	peripheral::Peripheral,
	timer::{config, Timer as EspTimer, TimerDriver},
};
use esp_idf_sys::{esp, EspError};
use firmware_core::{
	printer::components::hal::timer::{Timer as TimerTrait, TimerAdditionalFunctionality as TimerInInterruptTrait},
	utils::measurement::frequency::Frequency,
};

/// Maximum supported frequency of the [`base clock`].
///
/// ```
/// # use esp32-s3::peripherals::timer::*;
/// #
/// assert_eq!(BASE_CLOCK_FREQUENCY, Frequency::from_hertz(80_000_000));
/// ```
///
/// [`base clock`]: <https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/constant.TIMER_BASE_CLK.html>
pub const BASE_CLOCK_FREQUENCY: Frequency = Frequency::from_hertz(80_000_000);

pub struct Timer
{
	driver: TimerDriver<'static>,
	clock_divider: u32,
}

impl Timer
{
	pub fn new<TIMER: EspTimer>(
		timer: impl Peripheral<P = TIMER> + 'static, config: &config::Config,
	) -> Result<Self, EspError>
	{
		Ok(Self {
			driver: TimerDriver::new(timer, config)?,
			clock_divider: config.divider,
		})
	}
}

impl TimerTrait for Timer
{
	type Error = EspError;
	type AdditionalFunctionality = TimerInInterrupt;

	unsafe fn on_alarm(&mut self, callback: impl FnMut() + Send + 'static) -> Result<(), Self::Error>
	{
		self.driver.enable_alarm(true)?;
		self.driver.subscribe(callback)?;
		self.driver.enable_interrupt()?;

		Ok(())
	}

	fn enable_alarm(&mut self, enable: bool) -> Result<(), Self::Error>
	{
		self.driver.enable(enable)
	}

	fn get_alarm_in_ticks(&self) -> Result<u64, Self::Error>
	{
		self.driver.alarm()
	}

	fn get_additional_functionality(&self) -> Self::AdditionalFunctionality
	{
		TimerInInterrupt {
			group: self.driver.group(),
			index: self.driver.index(),
			clock_divider: self.clock_divider,
		}
	}

	fn get_clock_frequency(&self) -> Frequency
	{
		BASE_CLOCK_FREQUENCY / self.clock_divider
	}
}

impl Debug for Timer
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		f.debug_struct("Timer")
			.field("clock_divider", &self.clock_divider)
			.finish()
	}
}

pub struct TimerInInterrupt
{
	group: esp_idf_sys::timer_group_t,
	index: esp_idf_sys::timer_idx_t,
	clock_divider: u32,
}

impl TimerInInterruptTrait for TimerInInterrupt
{
	type Error = EspError;

	fn set_alarm(&mut self, alarm_time: Duration) -> Result<(), Self::Error>
	{
		let value = self.duration_to_counter(alarm_time);
		self.set_alarm_in_ticks(value)
	}

	fn set_alarm_in_ticks(&mut self, ticks: u64) -> Result<(), Self::Error>
	{
		if esp_idf_hal::interrupt::active()
		{
			unsafe {
				esp_idf_sys::timer_group_set_alarm_value_in_isr(self.group, self.index, ticks);
			}
		}
		else
		{
			esp!(unsafe { esp_idf_sys::timer_set_alarm_value(self.group, self.index, ticks) })?;
		}

		Ok(())
	}

	fn get_time(&self) -> Result<Duration, Self::Error>
	{
		Ok(self.counter_to_duration(self.get_time_in_ticks()?))
	}

	fn get_time_in_ticks(&self) -> Result<u64, Self::Error>
	{
		let value = if esp_idf_hal::interrupt::active()
		{
			unsafe { esp_idf_sys::timer_group_get_counter_value_in_isr(self.group, self.index) }
		}
		else
		{
			let mut value = 0_u64;

			esp!(unsafe { esp_idf_sys::timer_get_counter_value(self.group, self.index, &mut value) })?;

			value
		};

		Ok(value)
	}
}

impl TimerInInterrupt
{
	pub fn counter_to_duration(&self, counter: u64) -> Duration
	{
		let clock_frequency = (BASE_CLOCK_FREQUENCY.as_hertz() / self.clock_divider) as u64;
		let whole_seconds = counter / clock_frequency;
		let subsec_counter = counter - (whole_seconds * clock_frequency);
		let nanoseconds = subsec_counter * Duration::from_secs(1).as_nanos() as u64 / clock_frequency;
		Duration::new(whole_seconds, nanoseconds as u32)
	}

	pub fn duration_to_counter(&self, duration: Duration) -> u64
	{
		let clock_frequency = (BASE_CLOCK_FREQUENCY.as_hertz() / self.clock_divider) as u64;
		let mut counter = duration.as_secs() * clock_frequency;
		counter += (duration.subsec_nanos() as u64 * clock_frequency) / Duration::from_secs(1).as_nanos() as u64;
		counter
	}
}
