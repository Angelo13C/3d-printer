use crate::{printer::components::hal::pwm::PwmPin, utils::math::Percentage};

/// A cartridge heater connected to the microcontroller that can be controlled using the `P` pin.
pub struct CartridgeHeater<P: PwmPin>
{
	pin: P,
}

impl<P: PwmPin> CartridgeHeater<P>
{
	/// Returns a [`CartridgeHeater`] that can control its heat percentage through the provided `pin`.
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
