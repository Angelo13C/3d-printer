//! Defines safety mechanisms to prevent overheating and ensure safe operation.
//!
//! This module implements safety checks to ensure that the temperatures of the 3D printer's hotend and
//! heated bed remain within safe limits. It provides mechanisms to verify if the current and target
//! temperatures are within specified allowable ranges and handles various temperature-related errors.
//!
//! # Key struct
//! - [`TemperatureSafety`]: A struct that manages safety checks for temperature control.

use std::ops::RangeInclusive;

use enumset::EnumSet;

use self::{
	allowed_range::AllowedTemperatureRangeSafety,
	temperature_change::{modes::*, *},
};
use crate::utils::measurement::temperature::Temperature;

pub mod allowed_range;
pub mod temperature_change;

/// A struct that manages safety checks for temperature control.
pub struct TemperatureSafety
{
	allowed_temperature_range: AllowedTemperatureRangeSafety,
	keep_target_temperature: TemperatureChangeSafety<KeepMode>,
	rise_to_target_temperature: TemperatureChangeSafety<RisingMode>,
}

impl TemperatureSafety
{
	/// Creates a new `TemperatureSafety` instance with specified allowable temperature ranges and configurations.
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

	/// Returns a set of all the errors that occurred during the temperature safety check.
	/// If no errors occurred, the set is empty.
	pub fn is_temperature_safe(
		&mut self, current_temperature: Temperature, target_temperature: Option<Temperature>, delta_time: f32,
	) -> EnumSet<TemperatureError>
	{
		let mut errors = EnumSet::empty();

		if !self.allowed_temperature_range.is_temperature_safe(current_temperature)
		{
			errors.insert(TemperatureError::CurrentTemperatureOutsideAllowedRange);
		}

		if let Some(target_temperature) = target_temperature
		{
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
		}

		errors
	}
}

/// An enumeration of possible temperature-related errors that can occur during safety checks.
#[derive(enumset::EnumSetType, Debug, Hash)]
pub enum TemperatureError
{
	/// The `current_temperature` is outside the allowable range provided to [`TemperatureSafety::new`].
	CurrentTemperatureOutsideAllowedRange,

	/// The `target_temperature` is outside the allowable range provided to [`TemperatureSafety::new`].
	TargetTemperatureOutsideAllowedRange,

	/// After `current_temperature` reached the `target_temperature` in a previous call to the function,
	/// the `current_temperature` wasn't kept within the range `target_temperature ± keep_target_temperature_config.hysteresis`
	/// for more than the `keep_target_temperature_config.period_in_seconds` provided to [`TemperatureSafety::new`].
	/// Check [`this`] for more info.
	///
	/// [`this`]: temperature_change::modes::KeepMode
	CantKeepTargetTemperature,

	/// While `current_temperature` is trying to reach the `target_temperature`, the `current_temperature`
	/// wasn't rising fast enough.
	/// Check [`this`] for more info.
	///
	/// [`this`]: temperature_change::modes::RisingMode
	CantRiseFastEnoughToTargetTemperature,
}
