use super::{
	adc::{MockAdc, MockAdcPin},
	pwm::MockPwmPin,
	time::MockSystemTime,
	z_axis_probe::MockZAxisProbe,
	MockTimer, MockSpi,
};
use crate::printer::components::{
	motion::{homing::endstop::ManualEndstop, kinematics::CoreXYKinematics},
	Peripherals, drivers::spi_flash_memory::MT29F2G01ABAGDWB,
};

#[derive(Debug)]
pub struct MockPeripherals;

impl Peripherals for MockPeripherals
{
	type Kinematics = CoreXYKinematics;

	type StepperTickerTimer = MockTimer;

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
}
