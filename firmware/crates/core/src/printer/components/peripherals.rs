use embedded_hal::{digital::OutputPin, spi::SpiDevice};

use super::{
	drivers::spi_flash_memory::FlashMemoryChip,
	hal::{
		adc::{Adc, AdcPin},
		pwm::PwmPin,
		timer::Timer,
	},
	motion::{bed_leveling::ZAxisProbe, homing::endstop::Endstop, kinematics::Kinematics},
	time::SystemTime,
};

pub trait Peripherals
{
	type Kinematics: Kinematics;
	type StepperTickerTimer: Timer;

	type LeftDirPin: OutputPin + 'static;
	type LeftStepPin: OutputPin + 'static;
	type RightDirPin: OutputPin + 'static;
	type RightStepPin: OutputPin + 'static;
	type ZAxisDirPin: OutputPin + 'static;
	type ZAxisStepPin: OutputPin + 'static;
	type ExtruderDirPin: OutputPin + 'static;
	type ExtruderStepPin: OutputPin + 'static;

	type XAxisEndstop: Endstop;
	type YAxisEndstop: Endstop;
	type ZAxisEndstop: ZAxisProbe;

	type FlashChip: FlashMemoryChip + Send + 'static;
	type FlashSpi: SpiDevice<u8> + Send + 'static;

	type CartridgeHeaterPin: PwmPin;
	type HotendAdcPin: AdcPin<Self::Adc>;

	type HeatedBedHeaterPin: PwmPin;
	type HeatedBedAdcPin: AdcPin<Self::Adc>;

	type Adc: Adc;

	type FanPin: PwmPin;

	type SystemTime: SystemTime;

	fn take_x_axis_endstop(&mut self) -> Option<Self::XAxisEndstop>;
	fn take_y_axis_endstop(&mut self) -> Option<Self::YAxisEndstop>;
	fn take_z_axis_endstop(&mut self) -> Option<Self::ZAxisEndstop>;

	fn take_adc(&mut self) -> Option<Self::Adc>;

	fn take_bed_thermistor_pin(&mut self) -> Option<Self::HeatedBedAdcPin>;
	fn take_bed_cartridge_heater_pin(&mut self) -> Option<Self::HeatedBedHeaterPin>;
	fn take_hotend_thermistor_pin(&mut self) -> Option<Self::HotendAdcPin>;
	fn take_hotend_cartridge_heater_pin(&mut self) -> Option<Self::CartridgeHeaterPin>;

	fn take_layer_fan_pin(&mut self) -> Option<Self::FanPin>;
	fn take_hotend_fan_pin(&mut self) -> Option<Self::FanPin>;

	fn take_system_time(&mut self) -> Option<Self::SystemTime>;
}
