use std::ops::RangeInclusive;

use enumset::EnumSet;

use self::{
	allowed_range::AllowedTemperatureRangeSafety,
	temperature_change::{modes::*, *},
};
use crate::utils::measurement::temperature::Temperature;

pub mod allowed_range;
pub mod temperature_change;

pub struct TemperatureSafety
{
	allowed_temperature_range: AllowedTemperatureRangeSafety,
	keep_target_temperature: TemperatureChangeSafety<KeepMode>,
	rise_to_target_temperature: TemperatureChangeSafety<RisingMode>,
}

impl TemperatureSafety
{
	pub fn new(
		allowed_temperature_range: RangeInclusive<Temperature>,
		keep_target_temperature_config: TemperatureChangeConfig,
		rise_to_target_temperature_config: TemperatureChangeConfig, rise_to_target_temperature_samples_count: usize,
	) -> Self
	{
		Self {
			allowed_temperature_range: AllowedTemperatureRangeSafety::new(allowed_temperature_range),
			keep_target_temperature: TemperatureChangeSafety::new(KeepMode::default(), keep_target_temperature_config),
			rise_to_target_temperature: TemperatureChangeSafety::new(
				RisingMode::new(rise_to_target_temperature_samples_count),
				rise_to_target_temperature_config,
			),
		}
	}

	/// Returns a set of all the errors that happened. If no error has happened the set is empty.
	pub fn is_temperature_safe(
		&mut self, current_temperature: Temperature, target_temperature: Temperature, delta_time: f32,
	) -> EnumSet<TemperatureError>
	{
		let mut errors = EnumSet::empty();

		if !self.allowed_temperature_range.is_temperature_safe(current_temperature)
		{
			errors.insert(TemperatureError::CurrentTemperatureOutsideAllowedRange);
		}

		if !self.allowed_temperature_range.is_temperature_safe(target_temperature)
		{
			errors.insert(TemperatureError::TargetTemperatureOutsideAllowedRange);
		}

		if !self
			.keep_target_temperature
			.is_temperature_safe(current_temperature, target_temperature, delta_time)
		{
			errors.insert(TemperatureError::CantKeepTargetTemperature);
		}

		if !self
			.rise_to_target_temperature
			.is_temperature_safe(current_temperature, target_temperature, delta_time)
		{
			errors.insert(TemperatureError::CantRiseFastEnoughToTargetTemperature);
		}

		errors
	}
}

#[derive(enumset::EnumSetType, Debug, Hash)]
pub enum TemperatureError
{
	/// The `current_temperature` is outside the allowable range you provided to [`Safety::new`].
	CurrentTemperatureOutsideAllowedRange,

	/// The `target_temperature` is outside the allowable range you provided to [`Safety::new`].
	TargetTemperatureOutsideAllowedRange,

	/// After `current_temperature` reached the `target_temperature` in a previous call to the function,
	/// the `current_temperature` wasn't kept in the range `target_temperature ± keep_target_temperature_config.hysteresis`
	/// for more than the `keep_target_temperature_config.period_in_seconds` you provided to [`Safety::new`].
	///
	/// Check [`this`] for more info.
	///
	/// [`this`]: temperature_change::modes::KeepMode
	CantKeepTargetTemperature,

	/// While `current_temperature` is trying to reach the `target_temperature`, the `current_temperature`
	/// wasn't rising fast enough.
	///
	/// Check [`this`] for more info.
	///
	/// [`this`]: temperature_change::modes::RisingMode
	CantRiseFastEnoughToTargetTemperature,
}
