#![allow(unused_variables)]
//! This module provides mock implementations for each trait in the HAL (Hardware Abstraction Layer) module.
//! These mock structs are useful for testing and simulation purposes, allowing developers to test functionality
//! without requiring actual hardware.
//!
//! Each mock struct corresponds to a specific trait defined in the HAL module, providing a way to simulate
//! the behavior of the hardware without needing the physical components.
//!
//! The [`MockError`] type is defined to represent errors that may occur in the mock implementations, and it
//! implements various error traits for compatibility with HAL and other embedded systems libraries.

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
mod watchdog;
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
pub use watchdog::*;
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
