use super::{
	adc::{MockAdc, MockAdcPin},
	pwm::MockPwmPin,
	time::MockSystemTime,
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
}
