use super::{config::TemperatureChangeConfig, ProtectionModeTrait};
use crate::utils::measurement::temperature::Temperature;

#[derive(Default)]
/// Once the current temperature reaches the target temperature, check that it doesn't drift away for more than
/// [`TemperatureChangeConfig::hysteresis`] degrees for [`TemperatureChangeConfig::period_in_seconds`] seconds in a row.
///
/// # Examples
/// Here the temperature drifts away for more than [`TemperatureChangeConfig::hysteresis`] degrees, but it goes back
/// to normal values before [`TemperatureChangeConfig::period_in_seconds`] seconds.
/// ```
/// # use firmware_core::
/// # {
/// # 	printer::components::temperature::safety::temperature_change::{*, modes::*},
/// # 	utils::measurement::temperature::Temperature
/// # };
/// #
/// let mut keep_temperature_safety = TemperatureChangeSafety::new(KeepMode::default(),
/// TemperatureChangeConfig
/// {
/// 	period_in_seconds: 20.,
/// 	hysteresis: 10.
/// });
/// let target_temperature = Temperature::from_celsius(200.);
///
/// assert!(keep_temperature_safety.is_temperature_safe(target_temperature, target_temperature, 0.));
/// assert!(keep_temperature_safety.is_temperature_safe(Temperature::from_celsius(215.), target_temperature, 5.));
/// assert!(keep_temperature_safety.is_temperature_safe(Temperature::from_celsius(220.), target_temperature, 5.));
/// assert!(keep_temperature_safety.is_temperature_safe(Temperature::from_celsius(230.), target_temperature, 5.));
/// assert!(keep_temperature_safety.is_temperature_safe(target_temperature, target_temperature, 2.));
///
/// assert!(keep_temperature_safety.is_temperature_safe(Temperature::from_celsius(215.), target_temperature, 5.));
/// assert!(keep_temperature_safety.is_temperature_safe(Temperature::from_celsius(220.), target_temperature, 5.));
/// assert!(keep_temperature_safety.is_temperature_safe(Temperature::from_celsius(230.), target_temperature, 5.));
/// assert!(keep_temperature_safety.is_temperature_safe(target_temperature, target_temperature, 2.));
/// ```
///
/// Here the temperature drifts away for more than [`TemperatureChangeConfig::hysteresis`] degrees for more
/// than [`TemperatureChangeConfig::period_in_seconds`] seconds.
/// ```
/// # use firmware_core::
/// # {
/// # 	printer::components::temperature::safety::temperature_change::{*, modes::*},
/// # 	utils::measurement::temperature::Temperature
/// # };
/// #
/// let mut keep_temperature_safety = TemperatureChangeSafety::new(KeepMode::default(),
/// TemperatureChangeConfig
/// {
/// 	period_in_seconds: 4.,
/// 	hysteresis: 10.
/// });
/// let target_temperature = Temperature::from_celsius(200.);
///
/// assert!(keep_temperature_safety.is_temperature_safe(target_temperature, target_temperature, 0.));
/// assert!(keep_temperature_safety.is_temperature_safe(Temperature::from_celsius(225.), target_temperature, 3.));
/// assert!(!keep_temperature_safety.is_temperature_safe(Temperature::from_celsius(225.), target_temperature, 3.));
///
/// assert!(keep_temperature_safety.is_temperature_safe(target_temperature, target_temperature, 0.));
/// assert!(keep_temperature_safety.is_temperature_safe(Temperature::from_celsius(215.), target_temperature, 2.));
/// assert!(!keep_temperature_safety.is_temperature_safe(Temperature::from_celsius(215.), target_temperature, 3.));
/// ```
pub struct KeepMode;

impl ProtectionModeTrait for KeepMode
{
	fn should_start_timer(&self, current_temperature: Temperature, target_temperature: Temperature) -> bool
	{
		current_temperature >= target_temperature
	}

	fn should_continue_timer(
		&mut self, current_temperature: Temperature, target_temperature: Temperature, config: TemperatureChangeConfig,
		_: f32,
	) -> bool
	{
		!((target_temperature - Temperature::from_kelvin(config.hysteresis))
			..=(target_temperature + Temperature::from_kelvin(config.hysteresis)))
			.contains(&current_temperature)
	}
}
