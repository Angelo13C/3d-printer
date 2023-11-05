use esp_idf_hal::gpio::{Input, InterruptType, Pin, PinDriver};
use esp_idf_sys::EspError;
use firmware_core::{
	embedded_hal::digital::{ErrorType, InputPin as InputPinTrait},
	printer::components::hal::interrupt::{self, InterruptPin},
};

pub struct InputPin<'d, P: Pin>(pub PinDriver<'d, P, Input>);

impl<'d, P: Pin> InputPinTrait for InputPin<'d, P>
{
	fn is_high(&self) -> Result<bool, Self::Error>
	{
		Ok(self.0.is_high())
	}

	fn is_low(&self) -> Result<bool, Self::Error>
	{
		Ok(self.0.is_low())
	}
}

impl<'d, P: Pin> ErrorType for InputPin<'d, P>
{
	type Error = <PinDriver<'d, P, Input> as ErrorType>::Error;
}

impl<'d, P: Pin> InterruptPin for InputPin<'d, P>
{
	type Error = EspError;

	unsafe fn subscribe_to_interrupt(
		&mut self, when_to_trigger: interrupt::Trigger, callback: impl FnMut() + Send + 'static,
	) -> Result<(), Self::Error>
	{
		let trigger_type = match when_to_trigger
		{
			interrupt::Trigger::PositiveEdge => InterruptType::PosEdge,
			interrupt::Trigger::NegativeEdge => InterruptType::NegEdge,
			interrupt::Trigger::AnyEdge => InterruptType::AnyEdge,
			interrupt::Trigger::LowLevel => InterruptType::LowLevel,
			interrupt::Trigger::HighLevel => InterruptType::HighLevel,
		};

		self.0.set_interrupt_type(trigger_type)?;
		self.0.subscribe(callback)?;
		self.0.enable_interrupt()?;

		Ok(())
	}
}
