use std::convert::Infallible;

use embedded_hal::spi::{ErrorType, SpiDevice, SpiDeviceRead, SpiDeviceWrite};

pub struct MockSpi;
impl SpiDevice<u8> for MockSpi
{
	fn transaction(&mut self, _: &mut [embedded_hal::spi::Operation<'_, u8>]) -> Result<(), Self::Error>
	{
		todo!()
	}
}

impl SpiDeviceWrite for MockSpi
{
	fn write_transaction(&mut self, _: &[&[u8]]) -> Result<(), Self::Error>
	{
		todo!()
	}
}

impl SpiDeviceRead<u8> for MockSpi
{
	fn read_transaction(&mut self, _: &mut [&mut [u8]]) -> Result<(), Self::Error>
	{
		todo!()
	}
}

impl ErrorType for MockSpi
{
	type Error = Infallible;
}
