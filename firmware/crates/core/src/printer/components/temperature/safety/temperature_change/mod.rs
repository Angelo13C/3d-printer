//! There are 2 modes of protection for temperature change:
//! - [`RisingMode`](modes::RisingMode): Makes sure that before the current temperature reaches the target temperature,
//! the current temperature is rising fast enough.
//! - [`KeepMode`](modes::KeepMode): Makes sure that after the current temperature reaches the target temperature,
//! the current temperature is kept within a range near the target temperature.

mod config;
mod keep_mode;
mod rising_mode;

pub use config::*;

pub mod modes
{
	pub use super::{keep_mode::*, rising_mode::*};
}

use crate::utils::measurement::temperature::Temperature;

/// Makes sure the temperature change is "normal". What "normal" means depends on the `ProtectionMode`
/// parameter of this struct.
///
/// Check [`module's documentation for more info`](self).
pub struct TemperatureChangeSafety<ProtectionMode: ProtectionModeTrait>
{
	config: TemperatureChangeConfig,
	last_target_temperature: Option<Temperature>,
	protection_mode: ProtectionMode,
	current_timer_in_seconds: Option<f32>,
}

impl<ProtectionMode: ProtectionModeTrait> TemperatureChangeSafety<ProtectionMode>
{
	pub fn new(protection_mode: ProtectionMode, config: TemperatureChangeConfig) -> Self
	{
		Self {
			config,
			last_target_temperature: None,
			protection_mode,
			current_timer_in_seconds: None,
		}
	}

	pub fn is_temperature_safe(
		&mut self, current_temperature: Temperature, target_temperature: Temperature, delta_time: f32,
	) -> bool
	{
		// Stop the timer if the target temperature has changed
		if Some(target_temperature) != self.last_target_temperature
		{
			self.last_target_temperature = Some(target_temperature);

			self.stop_timer();
		}

		if let Some(mut current_timer_in_seconds) = self.current_timer_in_seconds.as_mut()
		{
			if self.protection_mode.should_continue_timer(
				current_temperature,
				target_temperature,
				self.config,
				delta_time,
			)
			{
				*current_timer_in_seconds -= delta_time;
				if *current_timer_in_seconds <= 0.
				{
					self.stop_timer();

					return false;
				}
			}
			else
			{
				self.restart_timer();
			}
		}
		else
		{
			if self
				.protection_mode
				.should_start_timer(current_temperature, target_temperature)
			{
				self.restart_timer();
			}
		}

		true
	}

	fn stop_timer(&mut self)
	{
		self.current_timer_in_seconds = None;
	}

	fn restart_timer(&mut self)
	{
		self.current_timer_in_seconds = Some(self.config.period_in_seconds);
	}
}

pub trait ProtectionModeTrait
{
	fn should_start_timer(&self, current_temperature: Temperature, target_temperature: Temperature) -> bool;
	fn should_continue_timer(
		&mut self, current_temperature: Temperature, target_temperature: Temperature, config: TemperatureChangeConfig,
		delta_time: f32,
	) -> bool;
}
