//! This module contains mathematical utilities and extensions for various calculations
//! used in the firmware. It provides functions for mapping values between ranges,
//! linear interpolation, constraining values to a specified range...

mod number_ext;
mod percentage;
pub mod vectors;

use std::ops::{Add, Div, Mul, RangeInclusive, Sub};

pub use number_ext::*;
pub use percentage::*;

/// Map a value from a specified range to another range.
///
/// This function takes a value and linearly maps it from the `from` range to the `to` range.
///
/// # Examples
/// ```
/// # use firmware_core::utils::math::map;
/// # use std::ops::RangeInclusive;
/// #
/// assert_eq!(map(5, 0..=10, 0..=100), 50);
/// assert_eq!(map(0, 0..=10, 0..=100), 0);
/// assert_eq!(map(10, 0..=10, 0..=100), 100);
/// ```
pub fn map<T>(value: T, from: RangeInclusive<T>, to: RangeInclusive<T>) -> T
where T: Clone + Copy + Sub<T, Output = T> + Mul<T, Output = T> + Add<T, Output = T> + Div<T, Output = T>
{
	(value - *from.start()) * (*to.end() - *to.start()) / (*from.end() - *from.start()) + *to.start()
}

/// [`Linearly interpolates`] between `range.start()` and `range.end()`.
///
/// # Examples
/// ```
/// # use firmware_core::utils::math;
/// #
/// assert_eq!(math::lerp(0.0, 0_f32..=100.), 0.);
/// assert_eq!(math::lerp(0.5, 0_f32..=100.), 50.);
/// assert_eq!(math::lerp(1.0, 0_f32..=100.), 100.);
/// ```
///
/// [`Linearly interpolates`]: <https://en.wikipedia.org/wiki/Linear_interpolation>
pub fn lerp<T>(t: f32, range: RangeInclusive<T>) -> T
where T: Clone + Copy + Sub<T, Output = T> + Mul<f32, Output = T> + Add<T, Output = T>
{
	*range.start() + (*range.end() - *range.start()) * t
}

/// Constrains a value within a specified range.
///
/// Returns `value` if it is within the provided `range`, `range.start()` if `value` is smaller
/// than `range.start()`, and `range.end()` if `value` is greater than `range.end()`.
///
/// # Examples
/// ```
/// # use firmware_core::utils::math::constrain;
/// assert_eq!(constrain(10, 0..=20), 10);
/// assert_eq!(constrain(-5, 0..=20), 0);
/// assert_eq!(constrain(40, 0..=20), 20);
/// ```
pub fn constrain<T>(value: T, range: RangeInclusive<T>) -> T
where T: Copy + PartialOrd
{
	if value < *range.start()
	{
		*range.start()
	}
	else if value > *range.end()
	{
		*range.end()
	}
	else
	{
		value
	}
}
