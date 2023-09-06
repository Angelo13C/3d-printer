use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::frequency::Frequency;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A duration of time with a `10 nanoseconds` sensitivity and a range of values that goes up to `10*2^32ns` (which is almost 43 seconds).
pub struct SmallDuration
{
	tens_nano_seconds: u32,
}

impl SmallDuration
{
	/// A [`SmallDuration`] with a duration 1 second.
	pub const SECOND: Self = Self::from_seconds(1);
	/// A [`SmallDuration`] with a duration 1 millisecond.
	pub const MILLI_SECOND: Self = Self::from_millis(1);
	/// A [`SmallDuration`] with a duration 1 microsecond.
	pub const MICRO_SECOND: Self = Self::from_micros(1);
	/// A [`SmallDuration`] with a duration 0 seconds.
	pub const ZERO: Self = Self::from_micros(0);
	/// A [`SmallDuration`] with the highest value storable in this struct (`10*2^32` nanoseconds, which is almost 43 seconds).
	pub const MAX_VALUE: Self = Self::from_tens_of_nanos(u32::MAX);

	/// Returns a [`SmallDuration`] with a duration of the provided tens of nanoseconds (`10^-8 seconds`).
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::duration::SmallDuration;
	/// #
	/// assert_eq!(SmallDuration::from_tens_of_nanos(1).as_nanos(), 10);
	/// assert_eq!(SmallDuration::from_tens_of_nanos(100).as_nanos(), 1_000);
	/// assert_eq!(SmallDuration::from_tens_of_nanos(100).as_tens_of_nanos(), 100);
	/// ```
	pub const fn from_tens_of_nanos(tens_of_nanos: u32) -> Self
	{
		Self {
			tens_nano_seconds: tens_of_nanos,
		}
	}

	/// Returns a [`SmallDuration`] with a duration of the provided microseconds (`10^-6 seconds`).
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::duration::SmallDuration;
	/// #
	/// assert_eq!(SmallDuration::from_micros(1).as_nanos(), 1_000);
	/// assert_eq!(SmallDuration::from_micros(1_000_000).as_seconds(), 1);
	/// assert_eq!(SmallDuration::from_micros(100).as_micros(), 100);
	/// ```
	pub const fn from_micros(micros: u32) -> Self
	{
		Self::from_tens_of_nanos(micros * 100)
	}

	/// Returns a [`SmallDuration`] with a duration of the provided milliseconds (`10^-3 seconds`).
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::duration::SmallDuration;
	/// #
	/// assert_eq!(SmallDuration::from_millis(1).as_nanos(), 1_000_000);
	/// assert_eq!(SmallDuration::from_millis(1_000).as_seconds(), 1);
	/// assert_eq!(SmallDuration::from_millis(100).as_millis(), 100);
	/// ```
	pub const fn from_millis(millis: u16) -> Self
	{
		Self::from_micros(millis as u32 * 1_000)
	}

	/// Returns a [`SmallDuration`] with a duration of the provided seconds.
	///
	/// Keep in mind the [`max value`] this struct can represent.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::duration::SmallDuration;
	/// #
	/// assert_eq!(SmallDuration::from_seconds(1).as_nanos(), 1_000_000_000);
	/// assert_eq!(SmallDuration::from_seconds(30).as_millis(), 30_000);
	/// assert_eq!(SmallDuration::from_seconds(20).as_seconds(), 20);
	/// ```
	///
	/// [`max value`]: `Self::MAX_VALUE`
	pub const fn from_seconds(seconds: u8) -> Self
	{
		Self::from_millis(seconds as u16 * 1_000)
	}

	/// Returns the number of nanoseconds (`10^-9 seconds`) this duration represents.
	pub const fn as_nanos(&self) -> u64
	{
		self.as_tens_of_nanos() as u64 * 10
	}

	/// Returns the number of tens of nanoseconds (`10^-8 seconds`) this duration represents.
	pub const fn as_tens_of_nanos(&self) -> u32
	{
		self.tens_nano_seconds
	}

	/// Returns the number of microseconds (`10^-6 seconds`) this duration represents.
	pub const fn as_micros(&self) -> u32
	{
		self.as_tens_of_nanos() / 100
	}

	/// Returns the number of milliseconds (`10^-3 seconds`) this duration represents.
	pub const fn as_millis(&self) -> u32
	{
		self.as_micros() / 1_000
	}

	/// Returns the number of seconds this duration represents.
	pub const fn as_seconds(&self) -> u32
	{
		self.as_millis() / 1_000
	}
}

impl From<Frequency> for SmallDuration
{
	/// Converts the provided [`Frequency`] into a [`SmallDuration`].
	///
	/// ```
	/// # use firmware_core::utils::measurement::{frequency::Frequency, duration::SmallDuration};
	/// #
	/// assert_eq!(Into::<SmallDuration>::into(Frequency::from_hertz(1)), SmallDuration::SECOND);
	/// assert_eq!(Into::<SmallDuration>::into(Frequency::from_hertz(5)), SmallDuration::from_micros(200_000));
	/// ```
	fn from(value: Frequency) -> Self
	{
		Self::from_tens_of_nanos(Self::SECOND.as_tens_of_nanos() / value.as_hertz())
	}
}

impl Add for SmallDuration
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output
	{
		Self::from_tens_of_nanos(self.as_tens_of_nanos() + rhs.as_tens_of_nanos())
	}
}
impl AddAssign for SmallDuration
{
	fn add_assign(&mut self, rhs: Self)
	{
		*self = *self + rhs
	}
}

impl Sub for SmallDuration
{
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output
	{
		Self::from_tens_of_nanos(self.as_tens_of_nanos() - rhs.as_tens_of_nanos())
	}
}
impl SubAssign for SmallDuration
{
	fn sub_assign(&mut self, rhs: Self)
	{
		*self = *self - rhs
	}
}

impl Mul<f32> for SmallDuration
{
	type Output = Self;

	fn mul(self, rhs: f32) -> Self::Output
	{
		Self::from_tens_of_nanos((self.as_tens_of_nanos() as f32 * rhs) as u32)
	}
}
impl MulAssign<f32> for SmallDuration
{
	fn mul_assign(&mut self, rhs: f32)
	{
		*self = *self * rhs
	}
}

impl Div<f32> for SmallDuration
{
	type Output = Self;

	fn div(self, rhs: f32) -> Self::Output
	{
		Self::from_tens_of_nanos((self.as_tens_of_nanos() as f32 / rhs) as u32)
	}
}
impl DivAssign<f32> for SmallDuration
{
	fn div_assign(&mut self, rhs: f32)
	{
		*self = *self / rhs
	}
}
