use embedded_hal::digital::{ErrorType, InputPin};

use crate::printer::components::{
	hal::interrupt::{InterruptPin, Trigger},
	motion::homing::endstop::Endstop,
};

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
	pub fn is_pressed(&self) -> Result<bool, <P as ErrorType>::Error>
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
	pub unsafe fn on_pressed(
		&mut self, callback: impl FnMut() + Send + 'static,
	) -> Result<(), <P as InterruptPin>::Error>
	{
		self.subscribe_to_interrupt(Trigger::PositiveEdge, callback)
	}
}

impl<P: InputPin> ErrorType for Button<P>
{
	type Error = P::Error;
}

impl<P: InputPin> InputPin for Button<P>
{
	fn is_high(&self) -> Result<bool, <Self as ErrorType>::Error>
	{
		self.pin.is_high()
	}

	fn is_low(&self) -> Result<bool, <Self as ErrorType>::Error>
	{
		self.pin.is_low()
	}
}

impl<P: InputPin + InterruptPin> InterruptPin for Button<P>
{
	type Error = <P as InterruptPin>::Error;

	unsafe fn subscribe_to_interrupt(
		&mut self, when_to_trigger: Trigger, callback: impl FnMut() + Send + 'static,
	) -> Result<(), Self::Error>
	{
		self.pin.subscribe_to_interrupt(when_to_trigger, callback)
	}
}

impl<P: InputPin + InterruptPin> Endstop for Button<P>
{
	type IsEndReachedError = <P as ErrorType>::Error;
	type OnEndReachedError = <P as InterruptPin>::Error;
	type HomingError = ();

	fn is_end_reached(&self) -> Result<bool, Self::IsEndReachedError>
	{
		self.is_pressed()
	}

	/// Equivalent to [`Button::on_pressed`], even for safety rules.
	unsafe fn on_end_reached(&mut self, callback: impl FnMut() + Send + 'static)
		-> Result<(), Self::OnEndReachedError>
	{
		self.on_pressed(callback)
	}
}
