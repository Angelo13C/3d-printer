use ringbuffer::{AllocRingBuffer, RingBuffer};

use super::{config::TemperatureChangeConfig, ProtectionModeTrait};
use crate::utils::measurement::temperature::Temperature;

/// While the current temperature is trying to reach the target temperature, check that it is always rising at least
/// [`TemperatureChangeConfig::hysteresis`] degrees for [`TemperatureChangeConfig::period_in_seconds`] seconds in a row.
///
/// # Examples
/// Here the temperature rises fast enough (because is is always rising for more than `10°C` within `20` seconds):
/// ```
/// # use firmware_core::
/// # {
/// # 	printer::components::temperature::safety::temperature_change::{*, modes::*},
/// # 	utils::measurement::temperature::Temperature
/// # };
/// #
/// let mut rising_temperature_safety = TemperatureChangeSafety::new(RisingMode::new(5),
/// TemperatureChangeConfig
/// {
/// 	period_in_seconds: 20.,
/// 	hysteresis: 10.
/// });
/// let target_temperature = Temperature::from_celsius(200.);
///
/// assert!(rising_temperature_safety.is_temperature_safe(Temperature::from_celsius(30.), target_temperature, 4.));
/// assert!(rising_temperature_safety.is_temperature_safe(Temperature::from_celsius(40.), target_temperature, 4.));
/// assert!(rising_temperature_safety.is_temperature_safe(Temperature::from_celsius(50.), target_temperature, 4.));
/// assert!(rising_temperature_safety.is_temperature_safe(Temperature::from_celsius(60.), target_temperature, 4.));
/// ```
///
/// Here the temperature doesn't rise fast enough (it rises by only `5°C` in `3` seconds which is below the config):
/// ```
/// # use firmware_core::
/// # {
/// # 	printer::components::temperature::safety::temperature_change::{*, modes::*},
/// # 	utils::measurement::temperature::Temperature
/// # };
/// #
/// let mut rising_temperature_safety = TemperatureChangeSafety::new(RisingMode::new(5),
/// TemperatureChangeConfig
/// {
/// 	period_in_seconds: 2.,
/// 	hysteresis: 10.
/// });
/// let target_temperature = Temperature::from_celsius(200.);
///
/// assert!(rising_temperature_safety.is_temperature_safe(Temperature::from_celsius(30.), target_temperature, 1.));
/// assert!(!rising_temperature_safety.is_temperature_safe(Temperature::from_celsius(35.), target_temperature, 3.));
/// ```
pub struct RisingMode
{
	samples: AllocRingBuffer<Temperature>,
	remaining_seconds_for_new_sample: f32,
}

impl RisingMode
{
	pub fn new(samples_count: usize) -> Self
	{
		Self {
			samples: AllocRingBuffer::new(samples_count),
			remaining_seconds_for_new_sample: 0.,
		}
	}

	fn reset(&mut self)
	{
		self.samples.clear();
		self.remaining_seconds_for_new_sample = 0.;
	}
}

impl ProtectionModeTrait for RisingMode
{
	fn should_start_timer(&self, current_temperature: Temperature, target_temperature: Temperature) -> bool
	{
		current_temperature < target_temperature
	}

	fn should_continue_timer(
		&mut self, current_temperature: Temperature, target_temperature: Temperature, config: TemperatureChangeConfig,
		delta_time: f32,
	) -> bool
	{
		let mut should_continue = self.should_start_timer(current_temperature, target_temperature);

		if should_continue
		{
			if let Some(oldest_sample) = self.samples.is_full().then_some(self.samples.front()).flatten()
			{
				let temperature_difference = current_temperature - *oldest_sample;
				should_continue = temperature_difference.as_kelvin() < config.hysteresis;
			}

			if should_continue
			{
				self.remaining_seconds_for_new_sample -= delta_time;

				if self.remaining_seconds_for_new_sample <= 0.
				{
					let seconds_to_take_sample = config.period_in_seconds / self.samples.len() as f32;
					self.remaining_seconds_for_new_sample += seconds_to_take_sample;

					self.samples.push(current_temperature);
				}
			}
		}

		if !should_continue
		{
			self.reset();
		}

		should_continue
	}
}
