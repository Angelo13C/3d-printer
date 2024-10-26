//! Analog-to-Digital Converter (ADC) module.
//!
//! This module defines traits and structures for interacting with ADC peripherals.
//! It provides a way to read analog values and convert them into percentage-based
//! representations. The ADC trait outlines the necessary functions for any ADC
//! implementation, while the ADC pin traits provide the interface for reading
//! values from specific ADC pins.

use std::{fmt::Debug, ops::Div};

use crate::utils::math::Percentage;

/// Trait representing an Analog-to-Digital Converter (ADC).
pub trait Adc
{
	/// The type of value that can be read from the ADC.
	type ReadableValue: Div<Self::ReadableValue, Output = Result<Percentage, ()>>;

	/// Returns the maximum readable value from the ADC.
	fn max_readable_value(&self) -> Self::ReadableValue;
}

/// Trait representing an ADC pin for a specific [`Adc`] type.
pub trait AdcPin<A: Adc>
{
	/// The type of error that can occur when reading from the pin.
	type Error: Debug;

	/// Reads a value from the ADC pin.
	///
	/// Returns the raw value read from the pin.
	fn read(&mut self, adc: &mut A) -> Result<A::ReadableValue, Self::Error>;
}

/// Extension trait for ADC pins that provides additional functionalities (it is automatically implemented).
pub trait AdcPinExt<A: Adc>: AdcPin<A>
{
	/// Reads a value from the ADC pin and returns it as a percentage.
	///
	/// This method uses the ADC's maximum readable value to convert the raw
	/// reading into a percentage. It returns an error if the reading cannot
	/// be performed or if the result is not a valid percentage.
	fn read_percentage(&mut self, adc: &mut A) -> Result<Percentage, ReadPercentageError<A, Self>>
	where Self: Sized;
}

/// Implementation of the `AdcPinExt` trait for all types that implement [`AdcPin`].
impl<P: AdcPin<A>, A: Adc> AdcPinExt<A> for P
{
	fn read_percentage(&mut self, adc: &mut A) -> Result<Percentage, ReadPercentageError<A, Self>>
	where Self: Sized
	{
		let read = self.read(adc).map_err(|err| ReadPercentageError::CantRead(err))?;
		(read / adc.max_readable_value()).map_err(|_| ReadPercentageError::InvalidPercentage)
	}
}

/// Enum representing errors that can occur when reading a percentage from an ADC pin.
pub enum ReadPercentageError<A: Adc, P: AdcPin<A>>
{
	/// Error indicating that the reading from the pin failed.
	CantRead(P::Error),
	/// Error indicating that the resulting percentage is invalid.
	InvalidPercentage,
}

impl<A: Adc, P: AdcPin<A>> Debug for ReadPercentageError<A, P>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Self::CantRead(arg0) => f.debug_tuple("Can't read").field(arg0).finish(),
			Self::InvalidPercentage => write!(f, "Invalid percentage"),
		}
	}
}
