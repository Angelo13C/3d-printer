//! Defines mechanisms for checking if a temperature is within an allowed range.
//!
//! This module provides the [`AllowedTemperatureRangeSafety`] struct, which ensures that a given
//! temperature is within a specified safe range. It is useful for enforcing safety constraints
//! in temperature control systems.

use std::ops::RangeInclusive;

use crate::utils::measurement::temperature::Temperature;

/// Makes sure a [`Temperature`] is in the allowed range you passed when constructing an instance of this
/// struct.
///
/// # Examples
/// ```
/// # use firmware_core::{printer::components::temperature::safety::allowed_range::*, utils::measurement::temperature::Temperature};
/// #
/// let allowed_temperature_range_safety = AllowedTemperatureRangeSafety::new(
/// 	Temperature::from_celsius(0.)..=Temperature::from_celsius(250.));
///
/// assert!(allowed_temperature_range_safety.is_temperature_safe(Temperature::from_celsius(100.)));
/// assert!(allowed_temperature_range_safety.is_temperature_safe(Temperature::from_celsius(250.)));
/// assert!(allowed_temperature_range_safety.is_temperature_safe(Temperature::from_celsius(0.)));
///
/// assert!(!allowed_temperature_range_safety.is_temperature_safe(Temperature::from_celsius(-20.)));
/// assert!(!allowed_temperature_range_safety.is_temperature_safe(Temperature::from_celsius(300.)));
/// assert!(!allowed_temperature_range_safety.is_temperature_safe(Temperature::from_celsius(-1.)));
/// ```
pub struct AllowedTemperatureRangeSafety
{
	allowed_range: RangeInclusive<Temperature>,
}

impl AllowedTemperatureRangeSafety
{
	/// Creates a new instance of `AllowedTemperatureRangeSafety` with the specified allowed range.
	///
	/// Check [`struct's documentation`](Self).
	pub const fn new(allowed_range: RangeInclusive<Temperature>) -> Self
	{
		Self { allowed_range }
	}

	/// Checks if the provided temperature is within the allowed range.
	///
	/// Check [`struct's documentation`](Self).
	pub fn is_temperature_safe(&self, temperature: Temperature) -> bool
	{
		self.allowed_range.contains(&temperature)
	}
}
