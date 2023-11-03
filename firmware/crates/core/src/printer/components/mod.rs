pub mod config;
pub mod drivers;
pub mod file_system;
pub mod g_code;
pub mod hal;
pub mod mock;
pub mod motion;
mod peripherals;
pub mod temperature;
pub mod time;

use std::fmt::Debug;

pub use peripherals::*;

use self::{
	config::ComponentsConfig,
	drivers::{cartridge_heater::CartridgeHeater, fan::Fan, thermistor::Thermistor},
	hal::timer::Timer as TimerTrait,
	motion::{
		bed_leveling::{Probe, ZAxisProbe},
		MotionController,
	},
	temperature::{safety::TemperatureSafety, TemperaturePidController},
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
	pub fn new(
		peripherals: &mut P, config: ComponentsConfig<P>,
	) -> Result<Self, CreationError<P::StepperTickerTimer, P::ZAxisEndstop>>
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
			hotend_pid_controller: TemperaturePidController::new(
				Thermistor::new(
					peripherals
						.take_hotend_thermistor_pin()
						.ok_or(CreationError::PeripheralMissing {
							name: "Hotend thermistor",
						})?,
					config.hotend_pid.thermistor.beta,
					config.hotend_pid.thermistor.resistance_at_t0,
					config.hotend_pid.thermistor.other_resistance,
				),
				CartridgeHeater::new(peripherals.take_hotend_cartridge_heater_pin().ok_or(
					CreationError::PeripheralMissing {
						name: "Hotend cartridge heater",
					},
				)?),
				config.hotend_pid.pid_gains,
				TemperatureSafety::new(
					config.hotend_pid.safety.allowed_temperature_range,
					config.hotend_pid.safety.keep_target_temperature_config,
					config.hotend_pid.safety.rise_to_target_temperature_config,
					config.hotend_pid.safety.rise_to_target_temperature_samples_count,
				),
			),
			heated_bed_pid_controller: TemperaturePidController::new(
				Thermistor::new(
					peripherals
						.take_bed_thermistor_pin()
						.ok_or(CreationError::PeripheralMissing { name: "Bed thermistor" })?,
					config.heated_bed_pid.thermistor.beta,
					config.heated_bed_pid.thermistor.resistance_at_t0,
					config.heated_bed_pid.thermistor.other_resistance,
				),
				CartridgeHeater::new(peripherals.take_bed_cartridge_heater_pin().ok_or(
					CreationError::PeripheralMissing {
						name: "Bed cartridge heater",
					},
				)?),
				config.heated_bed_pid.pid_gains,
				TemperatureSafety::new(
					config.heated_bed_pid.safety.allowed_temperature_range,
					config.heated_bed_pid.safety.keep_target_temperature_config,
					config.heated_bed_pid.safety.rise_to_target_temperature_config,
					config.heated_bed_pid.safety.rise_to_target_temperature_samples_count,
				),
			),
			adc: peripherals
				.take_adc()
				.ok_or(CreationError::PeripheralMissing { name: "ADC" })?,
			motion_controller: MotionController::new(config.motion_controller)
				.map_err(CreationError::MotionController)?,
		})
	}

	pub fn tick(&mut self) -> Result<(), TickError<P::ZAxisEndstop>>
	{
		self.clock.tick();

		let delta_time = self.clock.get_delta_time().as_secs_f64();
		self.heated_bed_pid_controller
			.tick(delta_time, &mut self.adc)
			.map_err(TickError::HeatedBedPidController)?;
		self.hotend_pid_controller
			.tick(delta_time, &mut self.adc)
			.map_err(TickError::HotendPidController)?;

		self.motion_controller.tick().map_err(TickError::MotionController)?;

		Ok(())
	}
}

#[derive(Debug)]
/// An error that can occur when you instatiate a [`Printer3DComponents`] struct.
pub enum CreationError<Timer: TimerTrait, ZEndstop: ZAxisProbe>
{
	/// A peripheral from the provided ones is missing (`name` is the name of the peripheral that's missing).
	/// This means that `peripherals.take_...()` returned `None` instead of `Some`.
	PeripheralMissing
	{
		name: &'static str,
	},

	Endstop,

	MotionController(motion::CreationError<Timer, ZEndstop>),
}

/// An error that can occur when you tick a [`Printer3DComponents`] struct.
#[derive(Debug)]
pub enum TickError<ZEndstop: ZAxisProbe>
{
	HeatedBedPidController(temperature::PidUpdateError),
	HotendPidController(temperature::PidUpdateError),
	MotionController(motion::homing::TickError<Probe<ZEndstop>>),
}
