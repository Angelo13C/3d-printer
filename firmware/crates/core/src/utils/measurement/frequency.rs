use std::ops::Div;

use super::duration::SmallDuration;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A frequency value with a `1Hz` sensitivity and a range of values that goes up to `2^32Hz` (which is almost 4.3GHz).
pub struct Frequency
{
	hertz: u32,
}

impl Frequency
{
	/// Returns a [`Frequency`] that represents the provided `hertz`.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::frequency::Frequency;
	/// #
	/// assert_eq!(Frequency::from_hertz(1).as_hertz(), 1);
	/// assert_eq!(Frequency::from_hertz(100).as_hertz(), 100);
	/// assert_eq!(Frequency::from_hertz(10_000).as_hertz(), 10_000);
	/// ```
	pub const fn from_hertz(hertz: u32) -> Self
	{
		Self { hertz }
	}

	/// Returns the number of hertz this frequency represents.
	pub const fn as_hertz(&self) -> u32
	{
		self.hertz
	}
}

impl From<SmallDuration> for Frequency
{
	/// Converts the provided [`SmallDuration`] into a [`Frequency`].
	///
	/// ```
	/// # use firmware_core::utils::measurement::{frequency::Frequency, duration::SmallDuration};
	/// #
	/// assert_eq!(Into::<Frequency>::into(SmallDuration::from_millis(50)), Frequency::from_hertz(20));
	/// assert_eq!(Into::<Frequency>::into(SmallDuration::from_micros(1)), Frequency::from_hertz(1_000_000));
	/// ```
	fn from(value: SmallDuration) -> Self
	{
		Self::from_hertz(SmallDuration::SECOND.as_tens_of_nanos() / value.as_tens_of_nanos())
	}
}

impl Div<u32> for Frequency
{
	type Output = Self;

	fn div(self, rhs: u32) -> Self::Output
	{
		Self::from_hertz(self.as_hertz() / rhs)
	}
}
