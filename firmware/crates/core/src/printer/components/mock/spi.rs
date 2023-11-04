use embedded_hal::spi::{ErrorType, SpiDevice};

use super::MockError;

pub struct MockSpi;
impl SpiDevice<u8> for MockSpi
{
	fn transaction(&mut self, _: &mut [embedded_hal::spi::Operation<'_, u8>]) -> Result<(), Self::Error>
	{
		todo!()
	}
}

impl ErrorType for MockSpi
{
	type Error = MockError;
}
