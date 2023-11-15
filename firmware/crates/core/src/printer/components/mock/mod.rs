#![allow(unused_variables)]

mod adc;
mod connection;
mod input;
mod output;
mod peripherals;
mod pwm;
mod spi;
mod time;
mod timer;
mod uart;
mod z_axis_probe;

pub use adc::*;
pub use connection::*;
pub use input::*;
pub use output::*;
pub use peripherals::*;
pub use pwm::*;
pub use spi::*;
pub use time::*;
pub use timer::*;
pub use uart::*;
pub use z_axis_probe::*;

#[derive(Debug)]
pub struct MockError;
impl embedded_hal::spi::Error for MockError
{
	fn kind(&self) -> embedded_hal::spi::ErrorKind
	{
		todo!()
	}
}
impl embedded_hal::digital::Error for MockError
{
	fn kind(&self) -> embedded_hal::digital::ErrorKind
	{
		todo!()
	}
}
impl embedded_svc::io::Error for MockError
{
	fn kind(&self) -> embedded_svc::io::ErrorKind
	{
		todo!()
	}
}
