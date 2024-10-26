//! This module provides the implementation for a cartridge heater interface
//! that can be used to control the heating element in a 3D printer.
//!
//! The `CartridgeHeater` struct allows for PWM control of the heater's temperature
//! through a specified pin.

use crate::{printer::components::hal::pwm::PwmPin, utils::math::Percentage};

/// A cartridge heater connected to the microcontroller that can be controlled using the `P` pin.
pub struct CartridgeHeater<P: PwmPin>
{
	pin: P,
}

impl<P: PwmPin> CartridgeHeater<P>
{
	/// Returns a new [`CartridgeHeater`] that can control its heat percentage through the provided `pin`.
	///
	/// # Parameters
	/// - `pin`: The PWM pin connected to the cartridge heater.
	pub fn new(pin: P) -> Self
	{
		Self { pin }
	}

	/// Sets the `percentage` of current to give to the heater.
	pub fn set_heat_percentage(&mut self, percentage: Percentage) -> Result<(), <P as PwmPin>::Error>
	{
		self.pin.set_duty_cycle(percentage)
	}
}
