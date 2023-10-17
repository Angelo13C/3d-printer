use super::{
	hal::{
		adc::{Adc, AdcPin},
		pwm::PwmPin,
		timer::Timer,
	},
	time::SystemTime,
};

pub trait Peripherals
{
	type CartridgeHeaterPin: PwmPin;
	type HotendAdcPin: AdcPin<Self::Adc>;

	type HeatedBedHeaterPin: PwmPin;
	type HeatedBedAdcPin: AdcPin<Self::Adc>;

	type Adc: Adc;

	type FanPin: PwmPin;

	type SystemTime: SystemTime;

	fn take_bed_cartridge_heater_pin(&mut self) -> Option<Self::CartridgeHeaterPin>;
	fn take_hotend_cartridge_heater_pin(&mut self) -> Option<Self::CartridgeHeaterPin>;
	fn take_layer_fan_pin(&mut self) -> Option<Self::FanPin>;
	fn take_hotend_fan_pin(&mut self) -> Option<Self::FanPin>;

	fn take_system_time(&mut self) -> Option<Self::SystemTime>;
}
