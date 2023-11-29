pub mod bresenham;
pub mod log_in_isr;
pub mod math;
pub mod measurement;
pub mod mutex;

/// Converts the provided `slice` to an array of the same type and with the length `N`.
///
/// # Panics
/// Panics if the length of the slice is less than `N`.
///
/// # Examples
/// ```
/// # use firmware_core::utils::*;
/// #
/// let array = [5, 10, 20];
/// assert_eq!(slice_to_array(&array), array);
/// ```
///
/// This instead will panic because the array has a length of 3 which is less than 4:
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
