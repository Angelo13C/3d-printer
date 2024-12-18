//! This module contains all the components necessary for the 3D printer to actually operate.
//!
//! It includes various configurations, drivers, motion control, temperature management,
//! and the overall control structures that enable the printer's functionality.
//!
//! # Key struct
//! - [`Printer3DComponents`]: A struct representing the components of the 3D printer.

pub mod config;
pub mod drivers;
pub mod file_system;
pub mod g_code;
pub mod hal;
pub mod mock;
pub mod motion;
pub mod pauser;
mod peripherals;
pub mod print_process;
pub mod temperature;
pub mod time;

use std::fmt::Debug;

use motion::planner::communicate_to_ticker;
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
use super::communication::http::other::printer_state;

/// This struct encapsulates all the elements required to make a 3D print possible, including fans,
/// motion control, temperature controllers, and the G-code executer.
///
/// It manages the initialization and operation of these components.
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
	/// Creates a new instance of `Printer3DComponents`.
	///
	/// This method initializes the printer components using the provided
	/// peripherals and configuration settings. It ensures all necessary
	/// components are properly set up.
	///
	/// # Arguments
	///
	/// * `peripherals` - The hardware peripherals required for the printer.
	/// * `config` - Configuration settings for the printer's components.
	///
	/// # Returns
	///
	/// A `Result` containing the initialized [`Printer3DComponents`] instance
	/// or a [`CreationError`] if an error occurs.
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

	/// Updates the state of the printer components, performing necessary tasks.
	///
	/// This method should be called periodically to ensure the printer operates
	/// correctly. It manages tasks related to temperature control, motion, and
	/// G-code execution.
	pub fn tick(&mut self) -> Result<(), TickError<P::ZAxisEndstop, P::UartDriver, P::StepperTickerTimer>>
	{
		// Time elapsed since the last time you called Self::tick.
		let delta_time = self.clock.get_delta_time().as_secs_f64();
		self.clock.tick();

		if !pauser::is_paused()
		{
			if let Some(mut g_code_executer) = self.g_code_executer.take()
			{
				g_code_executer.tick(self).map_err(TickError::GCodeExecuter)?;
				self.g_code_executer = Some(g_code_executer);
			}
		}

		self.heated_bed_pid_controller
			.tick(delta_time, &mut self.adc)
			.map_err(TickError::HeatedBedPidController)?;
		self.hotend_pid_controller
			.tick(delta_time, &mut self.adc)
			.map_err(TickError::HotendPidController)?;

		printer_state::tick::<P>(&self.hotend_pid_controller, &self.heated_bed_pid_controller);

		let mut is_moving = !pauser::is_paused();
		if is_moving
		{
			if let Some(g_code_executer) = self.g_code_executer.as_ref()
			{
				is_moving = g_code_executer.has_command_to_execute() || communicate_to_ticker::is_block_available();
			}
		}

		self.motion_controller
			.set_paused(!is_moving, &mut self.uart_driver)
			.map_err(TickError::PausingMotionController)?;
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
pub enum TickError<ZEndstop: ZAxisProbe, Uart: UartTrait, Timer: TimerTrait>
{
	GCodeExecuter(g_code::execute::TickError),
	HeatedBedPidController(temperature::PidUpdateError),
	HotendPidController(temperature::PidUpdateError),
	MotionController(motion::TickError<Probe<ZEndstop>>),
	PausingMotionController(motion::SetPausedError<Uart, Timer>),
}

impl<ZEndstop: ZAxisProbe, Uart: UartTrait, Timer: TimerTrait> Debug for TickError<ZEndstop, Uart, Timer>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Self::GCodeExecuter(arg0) => f.debug_tuple("GCodeExecuter").field(arg0).finish(),
			Self::HeatedBedPidController(arg0) => f.debug_tuple("HeatedBedPidController").field(arg0).finish(),
			Self::HotendPidController(arg0) => f.debug_tuple("HotendPidController").field(arg0).finish(),
			Self::MotionController(arg0) => f.debug_tuple("MotionController").field(arg0).finish(),
			Self::PausingMotionController(arg0) => f.debug_tuple("PausingMotionController").field(arg0).finish(),
		}
	}
}
