pub mod config;
pub mod drivers;
pub mod file_system;
pub mod g_code;
pub mod hal;
pub mod mock;
pub mod motion;
mod peripherals;
pub mod print_process;
pub mod temperature;
pub mod time;

use std::fmt::Debug;

pub use peripherals::*;

use self::{
	config::ComponentsConfig,
	drivers::{cartridge_heater::CartridgeHeater, fan::Fan, stepper_motor::StepperMotor, thermistor::Thermistor},
	g_code::execute::GCodeExecuter,
	hal::{timer::Timer as TimerTrait, uart::Uart as UartTrait},
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
	pub uart_driver: P::UartDriver,

	pub hotend_pid_controller: TemperaturePidController<P::CartridgeHeaterPin, P::Adc, P::HotendAdcPin>,
	pub heated_bed_pid_controller: TemperaturePidController<P::HeatedBedHeaterPin, P::Adc, P::HeatedBedAdcPin>,

	pub adc: P::Adc,

	pub clock: Clock<P::SystemTime>,

	pub g_code_executer: Option<GCodeExecuter<P>>,
}

impl<P: Peripherals> Printer3DComponents<P>
{
	pub fn new(
		peripherals: &mut P, config: ComponentsConfig,
	) -> Result<Self, CreationError<P::StepperTickerTimer, P::ZAxisEndstop, P::UartDriver>>
	{
		let mut uart_driver = peripherals
			.take_uart_driver()
			.ok_or(CreationError::PeripheralMissing { name: "Uart driver" })?;
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
			motion_controller: MotionController::new(
				motion::CreationParameters {
					left_motor: StepperMotor::new(
						peripherals
							.take_left_motor_dir_pin()
							.ok_or(CreationError::PeripheralMissing {
								name: "Left motor dir pin",
							})?,
						peripherals
							.take_left_motor_step_pin()
							.ok_or(CreationError::PeripheralMissing {
								name: "Left motor step pin",
							})?,
					),
					right_motor: StepperMotor::new(
						peripherals
							.take_right_motor_dir_pin()
							.ok_or(CreationError::PeripheralMissing {
								name: "Right motor dir pin",
							})?,
						peripherals
							.take_right_motor_step_pin()
							.ok_or(CreationError::PeripheralMissing {
								name: "Right motor step pin",
							})?,
					),
					z_axis_motor: StepperMotor::new(
						peripherals
							.take_z_axis_motor_dir_pin()
							.ok_or(CreationError::PeripheralMissing {
								name: "Z axis motor dir pin",
							})?,
						peripherals
							.take_z_axis_motor_step_pin()
							.ok_or(CreationError::PeripheralMissing {
								name: "Z axis motor step pin",
							})?,
					),
					extruder_motor: StepperMotor::new(
						peripherals
							.take_extruder_motor_dir_pin()
							.ok_or(CreationError::PeripheralMissing {
								name: "Extruder motor dir pin",
							})?,
						peripherals
							.take_extruder_motor_step_pin()
							.ok_or(CreationError::PeripheralMissing {
								name: "Extruder motor step pin",
							})?,
					),
					ticker_timer: peripherals
						.take_stepper_ticker_timer()
						.ok_or(CreationError::PeripheralMissing {
							name: "Stepper ticker timer",
						})?,
					kinematics: peripherals
						.take_kinematics()
						.ok_or(CreationError::PeripheralMissing { name: "Kinematics" })?,
					x_endstop: peripherals
						.take_x_axis_endstop()
						.ok_or(CreationError::PeripheralMissing { name: "X axis endstop" })?,
					y_endstop: peripherals
						.take_y_axis_endstop()
						.ok_or(CreationError::PeripheralMissing { name: "Y axis endstop" })?,
					z_endstop: peripherals
						.take_z_axis_endstop()
						.ok_or(CreationError::PeripheralMissing { name: "Z axis endstop" })?,
				},
				config.motion_controller,
				&mut uart_driver,
			)
			.map_err(CreationError::MotionController)?,
			uart_driver,
			g_code_executer: Some(GCodeExecuter::default()),
		})
	}

	pub fn tick(&mut self) -> Result<(), TickError<P::ZAxisEndstop>>
	{
		self.clock.tick();

		if let Some(mut g_code_executer) = self.g_code_executer.take()
		{
			g_code_executer.tick(self).map_err(TickError::GCodeExecuter)?;
			self.g_code_executer = Some(g_code_executer);
		}

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
pub enum CreationError<Timer: TimerTrait, ZEndstop: ZAxisProbe, Uart: UartTrait>
{
	/// A peripheral from the provided ones is missing (`name` is the name of the peripheral that's missing).
	/// This means that `peripherals.take_...()` returned `None` instead of `Some`.
	PeripheralMissing
	{
		name: &'static str,
	},

	Endstop,

	MotionController(motion::CreationError<Timer, ZEndstop, Uart>),
}

/// An error that can occur when you tick a [`Printer3DComponents`] struct.
#[derive(Debug)]
pub enum TickError<ZEndstop: ZAxisProbe>
{
	GCodeExecuter(g_code::execute::TickError),
	HeatedBedPidController(temperature::PidUpdateError),
	HotendPidController(temperature::PidUpdateError),
	MotionController(motion::homing::TickError<Probe<ZEndstop>>),
}
