use crate::printer::components::hal::uart::Uart;

pub struct MockUart;
impl Uart for MockUart
{
	type Error = ();

	fn read(
		&mut self, buf: &mut [u8], timeout: crate::utils::measurement::duration::SmallDuration,
	) -> Result<usize, Self::Error>
	{
		todo!()
	}

	fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error>
	{
		todo!()
	}

	fn flush_read(&mut self) -> Result<(), Self::Error>
	{
		todo!()
	}
}
