use std::{ops::Div, fmt::Debug};

use crate::utils::math::Percentage;

pub trait Adc
{
	type ReadableValue: Div<Self::ReadableValue, Output = Result<Percentage, ()>>;

	fn max_readable_value(&self) -> Self::ReadableValue;
}

pub trait AdcPin<A: Adc>
{
	type Error: Debug;

	fn read(&mut self, adc: &mut A) -> Result<A::ReadableValue, Self::Error>;
}

pub trait AdcPinExt<A: Adc>: AdcPin<A>
{
	fn read_percentage(&mut self, adc: &mut A) -> Result<Percentage, ReadPercentageError<A, Self>>
	where Self: Sized;
}

impl<P: AdcPin<A>, A: Adc> AdcPinExt<A> for P
{
	fn read_percentage(&mut self, adc: &mut A) -> Result<Percentage, ReadPercentageError<A, Self>>
	where Self: Sized
	{
		let read = self.read(adc).map_err(|err| ReadPercentageError::CantRead(err))?;
		(read / adc.max_readable_value()).map_err(|_| ReadPercentageError::InvalidPercentage)
	}
}

pub enum ReadPercentageError<A: Adc, P: AdcPin<A>>
{
	CantRead(P::Error),
	InvalidPercentage,
}

impl<A: Adc, P: AdcPin<A>> Debug for ReadPercentageError<A, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CantRead(arg0) => f.debug_tuple("Can't read").field(arg0).finish(),
            Self::InvalidPercentage => write!(f, "Invalid percentage"),
        }
    }
}
