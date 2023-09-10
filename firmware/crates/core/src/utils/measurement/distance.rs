use std::{
	default::Default,
	num::ParseIntError,
	ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A distance with a `1 femtometer` sensitivity (10^-15 meters) and a range of values that goes from `-2^63 m`
/// to `2^63-1 m` (which is from -9223 to 9223 meters).
///
/// The reason the sensitivity is so high is because in the case of a [`LinearStepperMotor`] connected to a lead
/// screw with an `8mm` lead, with `200` steps per revolution and with the support for `256` microstepping (thanks to the [`TMC2209`])
/// each microstep will take `8mm / (200 * 256) = 156.25nm`. If the sensitivity was only `1 nanometer` it would mean that for each
/// microstep there would be an error of `0.25nm`, and in the case of large distances (i.e. `>30cm`) this would adds up and create
/// errors of almost a millimeter. And also another problem is that using an `i32` to store the
/// distance in nanometers instead of an `i64` to store it femtometers (which is what I'm doing) would mean that the range
/// of values storable in this struct would be from `-2 to 2 meters`, which is too tight for measuring the distance
/// of the filament extruded by the extruder's stepper motor (a 1Kg 1.75mm PLA filament is around 333 meters long, and while
/// you would never use a whole filament to do a single print, it's possible that you would overshoot the 2 meters limit).
///
/// [`LinearStepperMotor`]: `crate::printer::components::motion::LinearStepperMotor`
/// [`TMC2209`]: `crate::printer::components::drivers::tmc2209::TMC2209`
pub struct Distance
{
	femtometers: i64,
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
	/// assert_eq!(Distance::NANOMETER, Distance::from_nanometers(1));
	/// ```
	pub const NANOMETER: Self = Self::from_nanometers(1);

	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// assert_eq!(Distance::INCH, Distance::from_inches(1.));
	/// ```
	pub const INCH: Self = Self::from_micrometers(25_400);

	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// assert_eq!(Distance::ZERO, Distance::from_femtometers(0));
	/// ```
	pub const ZERO: Self = Self::from_femtometers(0);

	/// Returns a [`Distance`] from the provided femtometers (`10^-15 meters`).
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// #
	/// assert_eq!(Distance::from_femtometers(30).as_femtometers(), 30);
	/// assert_eq!(Distance::from_femtometers(1_400).as_femtometers(), 1_400);
	/// assert_eq!(Distance::from_femtometers(5_100).as_picometers(), 5);
	/// ```
	pub const fn from_femtometers(femtometers: i64) -> Self
	{
		Self { femtometers }
	}

	/// Returns a [`Distance`] from the provided picometers (`10^-12 meters`).
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// #
	/// assert_eq!(Distance::from_picometers(30).as_picometers(), 30);
	/// assert_eq!(Distance::from_picometers(1_400).as_picometers(), 1_400);
	/// assert_eq!(Distance::from_picometers(5_100).as_nanometers(), 5);
	/// ```
	pub const fn from_picometers(picometers: i64) -> Self
	{
		Self::from_femtometers(picometers * 1_000)
	}

	/// Returns a [`Distance`] from the provided nanometers (`10^-9 meters`).
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// #
	/// assert_eq!(Distance::from_nanometers(30).as_nanometers(), 30);
	/// assert_eq!(Distance::from_nanometers(1_400).as_nanometers(), 1_400);
	/// assert_eq!(Distance::from_nanometers(5_100).as_micrometers(), 5);
	/// ```
	pub const fn from_nanometers(nanometers: i64) -> Self
	{
		Self::from_picometers(nanometers * 1_000)
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
	pub const fn from_micrometers(micrometers: i64) -> Self
	{
		Self::from_nanometers(micrometers * 1_000)
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
		Self::from_micrometers(millimeters as i64 * 1_000)
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
		let inches_to_femtometers = Distance::INCH.as_femtometers();
		let mut femtometers = (inches.trunc() as i64) * inches_to_femtometers;
		femtometers += (inches.fract() * inches_to_femtometers as f32) as i64;
		Self::from_femtometers(femtometers)
	}

	/// Returns the number of femtometers (`10^-15 meters`) this distance represents.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// #
	/// assert_eq!(Distance::from_femtometers(5_900).as_femtometers(), 5_900);
	/// ```
	pub const fn as_femtometers(&self) -> i64
	{
		self.femtometers
	}

	/// Returns the number of femtometers (`10^-15 meters`) this distance represents.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// #
	/// assert_eq!(Distance::from_femtometers(5_900).as_femtometers(), 5_900);
	/// ```
	pub const fn as_picometers(&self) -> i64
	{
		self.as_femtometers() / 1_000
	}

	/// Returns the number of nanometers (`10^-9 meters`) this distance represents.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// #
	/// assert_eq!(Distance::from_nanometers(5_900).as_nanometers(), 5_900);
	/// ```
	pub const fn as_nanometers(&self) -> i64
	{
		self.as_picometers() / 1_000
	}

	/// Returns the number of micrometers (`10^-6 meters`) this distance represents (the nanometers part is trunked).
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::distance::Distance;
	/// #
	/// // The .9 is trunked
	/// assert_eq!(Distance::from_nanometers(5_900).as_micrometers(), 5);
	/// ```
	pub const fn as_micrometers(&self) -> i64
	{
		self.as_nanometers() / 1_000
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

impl Mul<i32> for Distance
{
	type Output = Self;

	fn mul(self, rhs: i32) -> Self::Output
	{
		Self::from_femtometers(self.femtometers * rhs as i64)
	}
}
impl Mul<f32> for Distance
{
	type Output = Self;

	fn mul(self, rhs: f32) -> Self::Output
	{
		Self::from_femtometers((self.femtometers as f64 * rhs as f64) as i64)
	}
}
impl Div<i32> for Distance
{
	type Output = Self;

	fn div(self, rhs: i32) -> Self::Output
	{
		Self::from_femtometers(self.femtometers / rhs as i64)
	}
}
impl Div<Self> for Distance
{
	type Output = f32;

	fn div(self, rhs: Self) -> Self::Output
	{
		(self.as_nanometers() as f64 / rhs.as_nanometers() as f64) as f32
	}
}
impl Add<Distance> for Distance
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output
	{
		Self::from_femtometers(self.femtometers + rhs.femtometers)
	}
}
impl Sub<Distance> for Distance
{
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output
	{
		Self::from_femtometers(self.femtometers - rhs.femtometers)
	}
}

impl Neg for Distance
{
	type Output = Self;

	fn neg(self) -> Self::Output
	{
		Self::from_femtometers(-self.as_femtometers())
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
	/// assert_eq!(Units::Millimeters.create_distance("123.569402").unwrap().as_millimeters(), 123);
	/// assert_eq!(Units::Millimeters.create_distance("123.569402").unwrap().as_nanometers(), 123_569_402);
	/// // 5.5 inches are 139.7 millimeters
	/// assert_eq!(Units::Inches.create_distance("-5.5").unwrap().as_millimeters(), -139);
	/// assert_eq!(Units::Inches.create_distance("-5.5").unwrap().as_nanometers(), -139_700_000);
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
		let value_to_femtometers = match self
		{
			Units::Millimeters => Distance::MILLIMETER,
			Units::Inches => Distance::INCH,
		}
		.as_femtometers();

		// This conversion of the value to femtometers is separated in two parts
		// (one which applies the whole part of the number and the other one only for the decimal part)
		// because of precision errors with float numbers.
		const DECIMAL_SEPARATOR: char = '.';
		let femtometers = if let Some(decimal_separator_index) = value.find(DECIMAL_SEPARATOR)
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
				value[..decimal_separator_index].parse::<i64>()?
			};
			integer_part_of_value *= value_to_femtometers;

			let mut fractional_part_of_value = value[(1 + decimal_separator_index)..].parse::<i64>()?;
			fractional_part_of_value *= signum;
			fractional_part_of_value *= value_to_femtometers;
			fractional_part_of_value /= 10_i64.pow((value.len() - decimal_separator_index - 1) as u32);

			integer_part_of_value + fractional_part_of_value
		}
		else
		{
			value_to_femtometers * value.parse::<i64>()?
		};

		Ok(Distance::from_femtometers(femtometers))
	}
}
