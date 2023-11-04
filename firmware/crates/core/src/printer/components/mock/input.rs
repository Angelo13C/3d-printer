use embedded_hal::digital::{ErrorType, InputPin};

use super::MockError;

pub struct MockInputPin;
impl InputPin for MockInputPin
{
	fn is_high(&self) -> Result<bool, Self::Error>
	{
		todo!()
	}

	fn is_low(&self) -> Result<bool, Self::Error>
	{
		todo!()
	}
}

impl ErrorType for MockInputPin
{
	type Error = MockError;
}
