use std::convert::Infallible;

use embedded_hal::digital::{ErrorType, OutputPin};

pub struct MockOutputPin;
impl OutputPin for MockOutputPin
{
	fn set_low(&mut self) -> Result<(), Self::Error>
	{
		todo!()
	}

	fn set_high(&mut self) -> Result<(), Self::Error>
	{
		todo!()
	}
}

impl ErrorType for MockOutputPin
{
	type Error = Infallible;
}
