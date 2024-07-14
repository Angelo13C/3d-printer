use std::{
	default::Default,
	fmt::Debug,
	num::ParseIntError,
	ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// A distance with a `10 nanometer` sensitivity (10^-8 meters) and a range of values that goes from `10 * (-2^31) nm`
/// to `10 * (2^31-1) nm` (which is from -21 to 21 meters).
pub struct Distance
{
	tens_of_nanometers: i32,
}

impl Distance
{
	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// assert_eq!(Distance::CENTIMETER, Distance::from_centimeters(1));
	/// ```
	pub const CENTIMETER: Self = Self::from_centimeters(1);

	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// assert_eq!(Distance::MILLIMETER, Distance::from_millimeters(1));
	/// ```
	pub const MILLIMETER: Self = Self::from_millimeters(1);

	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// assert_eq!(Distance::MICROMETER, Distance::from_micrometers(1));
	/// ```
	pub const MICROMETER: Self = Self::from_micrometers(1);

	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// assert_eq!(Distance::TENS_OF_NANOMETERS, Distance::from_tens_of_nanometers(1));
	/// ```
	pub const TENS_OF_NANOMETERS: Self = Self::from_tens_of_nanometers(1);

	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// assert_eq!(Distance::INCH, Distance::from_inches(1.));
	/// ```
	pub const INCH: Self = Self::from_micrometers(25_400);

	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// assert_eq!(Distance::ZERO, Distance::from_tens_of_nanometers(0));
	/// ```
	pub const ZERO: Self = Self::from_tens_of_nanometers(0);

	/// Returns a [`Distance`] from the provided tens of nanometers (`10^-8 meters`).
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// #
	/// assert_eq!(Distance::from_tens_of_nanometers(30).as_tens_of_nanometers(), 30);
	/// assert_eq!(Distance::from_tens_of_nanometers(1_400).as_tens_of_nanometers(), 1_400);
	/// assert_eq!(Distance::from_tens_of_nanometers(510).as_micrometers(), 5);
	/// ```
	pub const fn from_tens_of_nanometers(tens_of_nanometers: i32) -> Self
	{
		Self { tens_of_nanometers }
	}

	/// Returns a [`Distance`] from the provided micrometers (`10^-6 meters`).
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// #
	/// assert_eq!(Distance::from_micrometers(30).as_micrometers(), 30);
	/// assert_eq!(Distance::from_micrometers(1_400).as_micrometers(), 1_400);
	/// assert_eq!(Distance::from_micrometers(5_100).as_millimeters(), 5);
	/// ```
	pub const fn from_micrometers(micrometers: i32) -> Self
	{
		Self::from_tens_of_nanometers(micrometers * 100)
	}

	/// Returns a [`Distance`] from the provided millimeters (`10^-3 meters`).
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// #
	/// assert_eq!(Distance::from_millimeters(30).as_millimeters(), 30);
	/// assert_eq!(Distance::from_millimeters(1_400).as_millimeters(), 1_400);
	/// assert_eq!(Distance::from_millimeters(2_000).as_centimeters(), 200);
	/// ```
	pub const fn from_millimeters(millimeters: i32) -> Self
	{
		Self::from_micrometers(millimeters * 1_000)
	}

	/// Returns a [`Distance`] from the provided centimeters (`10^-2 meters`).
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// #
	/// assert_eq!(Distance::from_centimeters(30).as_centimeters(), 30);
	/// assert_eq!(Distance::from_centimeters(140).as_centimeters(), 140);
	/// assert_eq!(Distance::from_centimeters(200).as_meters(), 2);
	/// ```
	pub const fn from_centimeters(centimeters: i32) -> Self
	{
		Self::from_millimeters(centimeters * 10)
	}

	/// Returns a [`Distance`] from the provided inches (`25.4 * 10^-3 meters`).
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// #
	/// assert_eq!(Distance::from_inches(3.).as_micrometers(), 76_200);
	/// assert_eq!(Distance::from_inches(1.).as_centimeters(), 2);
	/// assert_eq!(Distance::from_inches(60.).as_meters(), 1);
	/// ```
	pub fn from_inches(inches: f32) -> Self
	{
		// This conversion of inches to nanometers is separated in two parts
		// (one which applies the whole part of the number and the other one only for the decimal part)
		// because of precision errors with float numbers.
		let inches_to_tens_of_nanometers = Distance::INCH.as_tens_of_nanometers();
		let mut tens_of_nanometers = (inches.trunc() as i32) * inches_to_tens_of_nanometers;
		tens_of_nanometers += (inches.fract() * inches_to_tens_of_nanometers as f32) as i32;
		Self::from_tens_of_nanometers(tens_of_nanometers)
	}

	/// Returns the number of tens of nanometers (`10^-8 meters`) this distance represents.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// #
	/// assert_eq!(Distance::from_tens_of_nanometers(5_900).as_tens_of_nanometers(), 5_900);
	/// ```
	pub const fn as_tens_of_nanometers(&self) -> i32
	{
		self.tens_of_nanometers
	}

	/// Returns the number of micrometers (`10^-6 meters`) this distance represents (the nanometers part is trunked).
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// #
	/// // The .9 is trunked
	/// assert_eq!(Distance::from_tens_of_nanometers(590).as_micrometers(), 5);
	/// ```
	pub const fn as_micrometers(&self) -> i32
	{
		self.as_tens_of_nanometers() / 100
	}

	/// Returns the number of millimeters (`10^-3 meters`) this distance represents (the micrometers part is trunked).
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// #
	/// // The .9 is trunked
	/// assert_eq!(Distance::from_micrometers(5_900).as_millimeters(), 5);
	/// ```
	pub const fn as_millimeters(&self) -> i32
	{
		self.as_micrometers() as i32 / 1_000
	}

	/// Returns the number of millimeters (`10^-3 meters`) this distance represents as a f32.
	///
	/// It differs from [`Self::as_millimeters`] due to the fact that the millimeters are of type `f32` (the
	/// micrometers part is not trunked).
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// #
	/// assert_eq!(Distance::from_micrometers(5_900).as_millimeters_f32(), 5.9);
	/// assert_eq!(Distance::from_micrometers(3_210).as_millimeters_f32(), 3.21);
	/// ```
	pub fn as_millimeters_f32(&self) -> f32
	{
		self.as_tens_of_nanometers() as f32 / Self::MILLIMETER.as_tens_of_nanometers() as f32
	}

	/// Returns the number of centimeters (`10^-2 meters`) this distance represents (the millimeters part is trunked).
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// #
	/// // The .9 is trunked
	/// assert_eq!(Distance::from_millimeters(59).as_centimeters(), 5);
	/// ```
	pub const fn as_centimeters(&self) -> i32
	{
		self.as_millimeters() / 10
	}

	/// Returns the number of meters this distance represents (the decimeters part is trunked).
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// #
	/// // The .42 is trunked
	/// assert_eq!(Distance::from_centimeters(142).as_meters(), 1);
	/// ```
	pub const fn as_meters(&self) -> i32
	{
		self.as_centimeters() / 100
	}
}

impl Debug for Distance
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "Distance: {}mm", self.as_millimeters_f32())
	}
}

impl Mul<u8> for Distance
{
	type Output = Self;

	fn mul(self, rhs: u8) -> Self::Output
	{
		self * rhs as i32
	}
}
impl Mul<i32> for Distance
{
	type Output = Self;

	fn mul(self, rhs: i32) -> Self::Output
	{
		Self::from_tens_of_nanometers(self.tens_of_nanometers * rhs)
	}
}
impl Mul<f32> for Distance
{
	type Output = Self;

	fn mul(self, rhs: f32) -> Self::Output
	{
		Self::from_tens_of_nanometers((self.tens_of_nanometers as f64 * rhs as f64) as i32)
	}
}
impl MulAssign<f32> for Distance
{
	fn mul_assign(&mut self, rhs: f32)
	{
		*self = *self * rhs;
	}
}
impl Div<u8> for Distance
{
	type Output = Self;

	fn div(self, rhs: u8) -> Self::Output
	{
		self / rhs as i32
	}
}
impl Div<i32> for Distance
{
	type Output = Self;

	fn div(self, rhs: i32) -> Self::Output
	{
		Self::from_tens_of_nanometers(self.tens_of_nanometers / rhs)
	}
}
impl Div<Self> for Distance
{
	type Output = f32;

	fn div(self, rhs: Self) -> Self::Output
	{
		(self.as_tens_of_nanometers() as f64 / rhs.as_tens_of_nanometers() as f64) as f32
	}
}
impl Add<Distance> for Distance
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output
	{
		Self::from_tens_of_nanometers(self.tens_of_nanometers + rhs.tens_of_nanometers)
	}
}
impl AddAssign for Distance
{
	fn add_assign(&mut self, rhs: Self)
	{
		*self = *self + rhs;
	}
}
impl Sub<Distance> for Distance
{
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output
	{
		Self::from_tens_of_nanometers(self.tens_of_nanometers - rhs.tens_of_nanometers)
	}
}
impl SubAssign for Distance
{
	fn sub_assign(&mut self, rhs: Self)
	{
		*self = *self - rhs;
	}
}

impl Neg for Distance
{
	type Output = Self;

	fn neg(self) -> Self::Output
	{
		Self::from_tens_of_nanometers(-self.as_tens_of_nanometers())
	}
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// An enum to represent the 2 units of measurement mainly used in 3D printing.
pub enum Units
{
	#[default]
	Millimeters,
	Inches,
}

impl Units
{
	/// Create a [`Distance`] that represents the provided `value` in this unit of measurement.
	///
	/// Returns `Ok(Distance)` if there was no error parsing the `value` string into a number, otherwise `Err(ParseIntError)`.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::distance::Units;
	/// #
	/// assert_eq!(Units::Millimeters.create_distance("123.56940").unwrap().as_millimeters(), 123);
	/// assert_eq!(Units::Millimeters.create_distance("123.56940").unwrap().as_tens_of_nanometers(), 12_356_940);
	/// // 5.5 inches are 139.7 millimeters
	/// assert_eq!(Units::Inches.create_distance("-5.5").unwrap().as_millimeters(), -139);
	/// assert_eq!(Units::Inches.create_distance("-5.5").unwrap().as_tens_of_nanometers(), -13_970_000);
	///
	/// assert_eq!(Units::Inches.create_distance("-.1").unwrap().as_micrometers(), -2_540);
	/// ```
	///
	/// # Note
	/// The `value` string can hold a number with a fractional part, but the decimal separator must be the dot (`.`).
	/// ```should_panic
	/// # use firmware_core::utils::measurement::distance::Units;
	/// #
	/// Units::Millimeters.create_distance("123,1").unwrap();
	/// ```
	pub fn create_distance(&self, value: &str) -> Result<Distance, ParseIntError>
	{
		let value_to_tens_of_nanometers = match self
		{
			Units::Millimeters => Distance::MILLIMETER,
			Units::Inches => Distance::INCH,
		}
		.as_tens_of_nanometers();

		// This conversion of the value to tens of nanometers is separated in two parts
		// (one which applies the whole part of the number and the other one only for the decimal part)
		// because of precision errors with float numbers.
		const DECIMAL_SEPARATOR: char = '.';
		let tens_of_nanometers = if let Some(decimal_separator_index) = value.find(DECIMAL_SEPARATOR)
		{
			let signum = match value.starts_with('-')
			{
				true => -1,
				false => 1,
			};

			let mut integer_part_of_value = if decimal_separator_index == 0 || &value[..decimal_separator_index] == "-"
			{
				0
			}
			else
			{
				value[..decimal_separator_index].parse::<i32>()?
			};
			integer_part_of_value *= value_to_tens_of_nanometers;

			let mut fractional_part_of_value = value[(1 + decimal_separator_index)..].parse::<i64>()?;
			fractional_part_of_value *= signum;
			fractional_part_of_value *= value_to_tens_of_nanometers as i64;
			fractional_part_of_value /= 10_i64.pow((value.len() - decimal_separator_index - 1) as u32);

			integer_part_of_value + fractional_part_of_value as i32
		}
		else
		{
			value_to_tens_of_nanometers * value.parse::<i32>()?
		};

		Ok(Distance::from_tens_of_nanometers(tens_of_nanometers))
	}
}
