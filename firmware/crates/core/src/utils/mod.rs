//! This module provides various utility functions and types that assist in the firmware's
//! functionality. It includes operations for mathematical calculations, measurements, and
//! data conversions. The utilities are designed to simplify common tasks and improve code
//! reusability across different components of the firmware.

pub mod bresenham;
pub mod math;
pub mod measurement;

/// Converts the provided `slice` to an array of the same type and with the length `N`.
///
/// # Panics
/// This function will panic if the length of the provided slice is less than `N`.
///
/// # Examples
/// ```
/// # use firmware_core::utils::*;
/// #
/// let array = [5, 10, 20];
/// assert_eq!(slice_to_array(&array), array);
/// ```
///
/// This example will panic because the array has a length of 3, which is less than 4:
/// ```should_panic
/// # use firmware_core::utils::*;
/// #
/// let array = [5, 10, 20];
/// slice_to_array::<_, 4>(&array);
/// ```
pub fn slice_to_array<T, const N: usize>(slice: &[T]) -> [T; N]
where T: Clone + Copy
{
	core::array::from_fn(|i| slice[i])
}
