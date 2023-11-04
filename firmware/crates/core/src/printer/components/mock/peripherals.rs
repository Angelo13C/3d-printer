use super::{
	adc::{MockAdc, MockAdcPin},
	pwm::MockPwmPin,
	time::MockSystemTime,
	uart::MockUart,
	z_axis_probe::MockZAxisProbe,
	MockOutputPin, MockSpi, MockTimer,
};
use crate::printer::components::{
	drivers::spi_flash_memory::MT29F2G01ABAGDWB,
	motion::{homing::endstop::ManualEndstop, kinematics::CoreXYKinematics},
	Peripherals,
};

#[derive(Debug)]
pub struct MockPeripherals;

impl Peripherals for MockPeripherals
{
	type Kinematics = CoreXYKinematics;

	type StepperTickerTimer = MockTimer;

	type LeftDirPin = MockOutputPin;
	type LeftStepPin = MockOutputPin;
	type RightDirPin = MockOutputPin;
	type RightStepPin = MockOutputPin;
	type ZAxisDirPin = MockOutputPin;
	type ZAxisStepPin = MockOutputPin;
	type ExtruderDirPin = MockOutputPin;
	type ExtruderStepPin = MockOutputPin;

	type UartDriver = MockUart;

	type XAxisEndstop = ManualEndstop;
	type YAxisEndstop = ManualEndstop;
	type ZAxisEndstop = MockZAxisProbe;

	type CartridgeHeaterPin = MockPwmPin;
	type HotendAdcPin = MockAdcPin;

	type HeatedBedHeaterPin = MockPwmPin;
	type HeatedBedAdcPin = MockAdcPin;

	type FlashChip = MT29F2G01ABAGDWB;
	type FlashSpi = MockSpi;

	type Adc = MockAdc;

	type FanPin = MockPwmPin;

	type SystemTime = MockSystemTime;

	fn take_x_axis_endstop(&mut self) -> Option<Self::XAxisEndstop>
	{
		todo!()
	}

	fn take_y_axis_endstop(&mut self) -> Option<Self::YAxisEndstop>
	{
		todo!()
	}

	fn take_z_axis_endstop(&mut self) -> Option<Self::ZAxisEndstop>
	{
		todo!()
	}

	fn take_bed_cartridge_heater_pin(&mut self) -> Option<Self::CartridgeHeaterPin>
	{
		todo!()
	}

	fn take_hotend_cartridge_heater_pin(&mut self) -> Option<Self::CartridgeHeaterPin>
	{
		todo!()
	}

	fn take_layer_fan_pin(&mut self) -> Option<Self::FanPin>
	{
		todo!()
	}

	fn take_hotend_fan_pin(&mut self) -> Option<Self::FanPin>
	{
		todo!()
	}

	fn take_system_time(&mut self) -> Option<Self::SystemTime>
	{
		todo!()
	}

	fn take_adc(&mut self) -> Option<Self::Adc>
	{
		todo!()
	}

	fn take_bed_thermistor_pin(&mut self) -> Option<Self::HeatedBedAdcPin>
	{
		todo!()
	}

	fn take_hotend_thermistor_pin(&mut self) -> Option<Self::HotendAdcPin>
	{
		todo!()
	}

	fn take_kinematics(&mut self) -> Option<Self::Kinematics>
	{
		todo!()
	}

	fn take_stepper_ticker_timer(&mut self) -> Option<Self::StepperTickerTimer>
	{
		todo!()
	}

	fn take_left_motor_dir_pin(&mut self) -> Option<Self::LeftDirPin>
	{
		todo!()
	}

	fn take_left_motor_step_pin(&mut self) -> Option<Self::LeftStepPin>
	{
		todo!()
	}

	fn take_right_motor_dir_pin(&mut self) -> Option<Self::RightDirPin>
	{
		todo!()
	}

	fn take_right_motor_step_pin(&mut self) -> Option<Self::RightStepPin>
	{
		todo!()
	}

	fn take_z_axis_motor_dir_pin(&mut self) -> Option<Self::ZAxisDirPin>
	{
		todo!()
	}

	fn take_z_axis_motor_step_pin(&mut self) -> Option<Self::ZAxisStepPin>
	{
		todo!()
	}

	fn take_extruder_motor_dir_pin(&mut self) -> Option<Self::ExtruderDirPin>
	{
		todo!()
	}

	fn take_extruder_motor_step_pin(&mut self) -> Option<Self::ExtruderStepPin>
	{
		todo!()
	}

	fn take_uart_driver(&mut self) -> Option<Self::UartDriver>
	{
		todo!()
	}

	fn take_flash_chip(&mut self) -> Option<Self::FlashChip>
	{
		todo!()
	}

	fn take_flash_spi(&mut self) -> Option<Self::FlashSpi>
	{
		todo!()
	}
}
