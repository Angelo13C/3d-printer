pub mod config;
pub mod drivers;
pub mod file_system;
pub mod hal;
pub mod motion;
mod peripherals;
pub mod temperature;
pub mod time;

pub use peripherals::*;

use self::{
	config::ComponentsConfig, drivers::fan::Fan, motion::MotionController, temperature::TemperaturePidController,
	time::Clock,
};

pub struct Printer3DComponents<P: Peripherals>
{
	pub layer_fan: Fan<P::FanPin>,
	pub hotend_fan: Fan<P::FanPin>,

	pub motion_controller: MotionController<P::StepperTickerTimer, P::Kinematics, P::ZAxisEndstop>,

	pub hotend_pid_controller: TemperaturePidController<P::CartridgeHeaterPin, P::Adc, P::HotendAdcPin>,
	pub heated_bed_pid_controller: TemperaturePidController<P::HeatedBedHeaterPin, P::Adc, P::HeatedBedAdcPin>,

	pub adc: P::Adc,

	pub clock: Clock<P::SystemTime>,
}

impl<P: Peripherals> Printer3DComponents<P>
{
	pub fn new(peripherals: &mut P, config: ComponentsConfig) -> Result<Self, CreationError>
	{
		Ok(Self {
			layer_fan: Fan::new(
				peripherals
					.take_layer_fan_pin()
					.ok_or(CreationError::PeripheralMissing { name: "Layer fan" })?,
				config.layer_fan_min_duty_cycle_to_move,
			),
			hotend_fan: Fan::new(
				peripherals
					.take_hotend_fan_pin()
					.ok_or(CreationError::PeripheralMissing { name: "Hotend fan" })?,
				config.hotend_fan_min_duty_cycle_to_move,
			),
			clock: Clock::new(
				peripherals
					.take_system_time()
					.ok_or(CreationError::PeripheralMissing { name: "System time" })?,
			),
			hotend_pid_controller: todo!(),
			heated_bed_pid_controller: todo!(),
			adc: todo!(),
			motion_controller: todo!(),
		})
	}

	pub fn tick(&mut self)
	{
		self.clock.tick();

		todo!()
	}
}

#[derive(Debug)]
/// An error that can occur when you instatiate a [`Printer3DComponents`] struct.
pub enum CreationError
{
	/// A peripheral from the provided ones is missing (`name` is the name of the peripheral that's missing).
	/// This means that `peripherals.take_...()` returned `None` instead of `Some`.
	PeripheralMissing
	{
		name: &'static str
	},
}
