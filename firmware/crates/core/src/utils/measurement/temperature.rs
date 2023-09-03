use std::{
	fmt::Debug,
	ops::{Add, Sub},
};

/// A temperature value.
#[derive(Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Temperature
{
	kelvin: f32,
}

impl Temperature
{
	/// Equivalent of `0°C` in the Kelvin scale.
	pub const ZERO_CELSIUS_IN_KELVIN: f32 = 273.15;

	/// Returns a [`Temperature`] of the provided `degrees` celsius.
	pub fn from_celsius(degrees: f32) -> Self
	{
		Self::from_kelvin(degrees + Self::ZERO_CELSIUS_IN_KELVIN)
	}

	/// Returns a [`Temperature`] of the provided `kelvin`.
	pub const fn from_kelvin(kelvin: f32) -> Self
	{
		Self { kelvin }
	}

	/// Returns the value of [`Self`] in degrees Celsius.
	///
	/// # Example
	/// ```
	/// # use firmware_core::utils::measurement::temperature::Temperature;
	/// #
	/// assert_eq!(Temperature::from_kelvin(0.).as_celsius(), -Temperature::ZERO_CELSIUS_IN_KELVIN);
	/// assert_eq!(Temperature::from_celsius(0.).as_celsius(), 0.);
	/// ```
	pub fn as_celsius(&self) -> f32
	{
		self.kelvin - Self::ZERO_CELSIUS_IN_KELVIN
	}

	/// Returns the value of [`Self`] in Kelvin.
	///
	/// # Example
	/// ```
	/// # use firmware_core::utils::measurement::temperature::Temperature;
	/// #
	/// assert_eq!(Temperature::from_kelvin(0.).as_kelvin(), 0.);
	/// assert_eq!(Temperature::from_celsius(0.).as_kelvin(), Temperature::ZERO_CELSIUS_IN_KELVIN);
	/// ```
	pub const fn as_kelvin(&self) -> f32
	{
		self.kelvin
	}
}

impl Add<Self> for Temperature
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output
	{
		Self::from_kelvin(self.kelvin + rhs.kelvin)
	}
}
impl Sub<Self> for Temperature
{
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output
	{
		Self::from_kelvin(self.kelvin - rhs.kelvin)
	}
}

impl Debug for Temperature
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "{} °C", self.as_celsius())
	}
}
