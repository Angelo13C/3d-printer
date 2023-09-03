use embedded_hal::digital::v2::InputPin;

use crate::printer::components::hal::interrupt::{InterruptPin, Trigger};

/// A button connected to the microcontroller that can read if it's pressed through the `P` pin.
pub struct Button<P: InputPin>
{
	pin: P,
}

impl<P: InputPin> Button<P>
{
	/// Returns a [`Button`] that can read its state on the provided `pin`.
	pub fn new(pin: P) -> Self
	{
		Self { pin }
	}

	/// Returns `Ok(true)` if the button is pressed, `Ok(false)` if it isn't pressed and
	/// `Err(<P as InputPin>::Error)` if there has been an error while reading the button's state.
	pub fn is_pressed(&self) -> Result<bool, <P as InputPin>::Error>
	{
		self.pin.is_high()
	}
}

impl<P: InputPin + InterruptPin> Button<P>
{
	/// Call the provided `callback` function when the button is pressed (using an interrupt).
	///
	/// # Safety
	/// Check [`P::subscribe`](InterruptPin).
	pub unsafe fn on_pressed(&mut self, callback: impl FnMut() + 'static) -> Result<(), <P as InterruptPin>::Error>
	{
		self.pin.subscribe_to_interrupt(Trigger::PositiveEdge, callback)
	}
}
