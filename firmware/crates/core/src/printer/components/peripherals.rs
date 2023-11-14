use embedded_hal::{digital::OutputPin, spi::SpiDevice};

use super::{
	drivers::spi_flash_memory::FlashMemoryChip,
	hal::{
		adc::{Adc, AdcPin},
		pwm::PwmPin,
		timer::Timer,
		uart::Uart,
	},
	motion::{bed_leveling::ZAxisProbe, homing::endstop::Endstop, kinematics::Kinematics},
	time::SystemTime,
};

pub trait Peripherals
{
	type Kinematics: Kinematics;
	type StepperTickerTimer: Timer;

	type LeftDirPin: OutputPin + Send + 'static;
	type LeftStepPin: OutputPin + Send + 'static;
	type RightDirPin: OutputPin + Send + 'static;
	type RightStepPin: OutputPin + Send + 'static;
	type ZAxisDirPin: OutputPin + Send + 'static;
	type ZAxisStepPin: OutputPin + Send + 'static;
	type ExtruderDirPin: OutputPin + Send + 'static;
	type ExtruderStepPin: OutputPin + Send + 'static;

	type UartDriver: Uart;

	type XAxisEndstop: Endstop + Send + 'static;
	type YAxisEndstop: Endstop + Send + 'static;
	type ZAxisEndstop: ZAxisProbe + 'static;

	type FlashChip: FlashMemoryChip + Send + 'static;
	type FlashSpi: SpiDevice<u8> + Send + 'static;

	type CartridgeHeaterPin: PwmPin;
	type HotendAdcPin: AdcPin<Self::Adc>;

	type HeatedBedHeaterPin: PwmPin;
	type HeatedBedAdcPin: AdcPin<Self::Adc>;

	type Adc: Adc;

	type FanPin: PwmPin;

	type SystemTime: SystemTime + Send;

	fn take_kinematics(&mut self) -> Option<Self::Kinematics>;
	fn take_stepper_ticker_timer(&mut self) -> Option<Self::StepperTickerTimer>;

	fn take_left_motor_dir_pin(&mut self) -> Option<Self::LeftDirPin>;
	fn take_left_motor_step_pin(&mut self) -> Option<Self::LeftStepPin>;
	fn take_right_motor_dir_pin(&mut self) -> Option<Self::RightDirPin>;
	fn take_right_motor_step_pin(&mut self) -> Option<Self::RightStepPin>;
	fn take_z_axis_motor_dir_pin(&mut self) -> Option<Self::ZAxisDirPin>;
	fn take_z_axis_motor_step_pin(&mut self) -> Option<Self::ZAxisStepPin>;
	fn take_extruder_motor_dir_pin(&mut self) -> Option<Self::ExtruderDirPin>;
	fn take_extruder_motor_step_pin(&mut self) -> Option<Self::ExtruderStepPin>;

	fn take_uart_driver(&mut self) -> Option<Self::UartDriver>;

	fn take_x_axis_endstop(&mut self) -> Option<Self::XAxisEndstop>;
	fn take_y_axis_endstop(&mut self) -> Option<Self::YAxisEndstop>;
	fn take_z_axis_endstop(&mut self) -> Option<Self::ZAxisEndstop>;

	fn take_flash_chip(&mut self) -> Option<Self::FlashChip>;
	fn take_flash_spi(&mut self) -> Option<Self::FlashSpi>;

	fn take_adc(&mut self) -> Option<Self::Adc>;

	fn take_bed_thermistor_pin(&mut self) -> Option<Self::HeatedBedAdcPin>;
	fn take_bed_cartridge_heater_pin(&mut self) -> Option<Self::HeatedBedHeaterPin>;
	fn take_hotend_thermistor_pin(&mut self) -> Option<Self::HotendAdcPin>;
	fn take_hotend_cartridge_heater_pin(&mut self) -> Option<Self::CartridgeHeaterPin>;

	fn take_layer_fan_pin(&mut self) -> Option<Self::FanPin>;
	fn take_hotend_fan_pin(&mut self) -> Option<Self::FanPin>;

	fn take_system_time(&mut self) -> Option<Self::SystemTime>;
}
