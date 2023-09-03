use super::hal::pwm::PwmPin;

pub trait Peripherals
{
	type CartridgeHeaterPin: PwmPin;
	type FanPin: PwmPin;

	fn take_bed_cartridge_heater_pin(&mut self) -> Option<Self::CartridgeHeaterPin>;
	fn take_hotend_cartridge_heater_pin(&mut self) -> Option<Self::CartridgeHeaterPin>;
	fn take_layer_fan_pin(&mut self) -> Option<Self::FanPin>;
	fn take_hotend_fan_pin(&mut self) -> Option<Self::FanPin>;
}
