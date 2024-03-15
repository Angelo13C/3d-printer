use std::{fmt::Debug, time::Duration};

use embedded_hal::digital::OutputPin;
pub use linear::*;

use self::{
	axes::Axis,
	bed_leveling::{Probe, ZAxisProbe},
	homing::{endstop::Endstop, HomingProcedure},
	kinematics::Kinematics as KinematicsTrait,
	planner::{communicate_to_ticker, BlocksBufferIsFull, MoveId, Planner, Settings},
	ticker::StepperMotorsTicker,
};
use super::{
	drivers::stepper_motor::{
		tmc2209::{UARTAddress, TMC2209},
		StepperMotor,
	},
	hal::{timer::Timer as TimerTrait, uart::Uart as UartTrait},
};
use crate::{
	printer::components::drivers::stepper_motor::tmc2209::MicrostepsPerStep,
	utils::{
		math::vectors::{Vector2, Vector3, VectorN},
		measurement::distance::Distance,
	},
};

pub mod axes;
pub mod bed_leveling;
pub mod homing;
pub mod kinematics;
mod linear;
pub mod planner;
pub mod ticker;

/// Number of stepper motors controlled by the machine.
pub const N_MOTORS: usize = 4;

const DEFAULT_FEED_RATE: f32 = 3_000.;

/// Controls all the stuff (moves, homing, bed levelling..) related to the movement of everything inside the machine.
pub struct MotionController<Timer: TimerTrait, Kinematics: KinematicsTrait, ZEndstop: ZAxisProbe>
{
	planner: Planner<N_MOTORS>,

	ticker: StepperMotorsTicker<Timer>,

	tmc2209_drivers: [TMC2209; N_MOTORS],
	rotations_to_linear_motions: [RotationToLinearMotion; N_MOTORS],
	kinematics: Kinematics,

	homing_procedure: HomingProcedure,

	current_move: Option<CurrentMove>,
	last_planned_move_end_position: Option<VectorN<N_MOTORS>>,

	z_endstop: Probe<ZEndstop>,

	bed_size: Vector2,
	next_move_feed_rate: f32,

	is_paused: bool,
}

impl<Timer: TimerTrait, Kinematics: KinematicsTrait, ZEndstop: ZAxisProbe> MotionController<Timer, Kinematics, ZEndstop>
{
	pub fn new<
		LeftDirPin: OutputPin + Send + 'static,
		LeftStepPin: OutputPin + Send + 'static,
		RightDirPin: OutputPin + Send + 'static,
		RightStepPin: OutputPin + Send + 'static,
		ZAxisDirPin: OutputPin + Send + 'static,
		ZAxisStepPin: OutputPin + Send + 'static,
		ExtruderDirPin: OutputPin + Send + 'static,
		ExtruderStepPin: OutputPin + Send + 'static,
		XEndstop: Endstop + Send + 'static,
		YEndstop: Endstop + Send + 'static,
		Uart: UartTrait,
	>(
		peripherals: CreationParameters<
			Timer,
			Kinematics,
			LeftDirPin,
			LeftStepPin,
			RightDirPin,
			RightStepPin,
			ZAxisDirPin,
			ZAxisStepPin,
			ExtruderDirPin,
			ExtruderStepPin,
			XEndstop,
			YEndstop,
			ZEndstop,
		>,
		configuration: CreationConfig, uart_driver: &mut Uart,
	) -> Result<Self, CreationError<Timer, ZEndstop, Uart>>
	{
		communicate_to_ticker::set_kinematics_functions::<Kinematics>();

		let mut z_endstop = Probe::new(peripherals.z_endstop, configuration.offset_from_nozzle_of_z_probe);

		let mut ticker = StepperMotorsTicker::new(
			peripherals.left_motor,
			peripherals.right_motor,
			peripherals.z_axis_motor,
			peripherals.extruder_motor,
			peripherals.ticker_timer,
			peripherals.x_endstop,
			peripherals.y_endstop,
			&mut z_endstop,
		)
		.map_err(CreationError::CreateStepperTickerTimer)?;

		ticker.enable().map_err(CreationError::EnableStepperTickerTimer)?;

		Ok(Self {
			planner: Planner::new(
				configuration.planner_blocks_count,
				&ticker,
				configuration.planner_settings,
			),
			ticker,
			kinematics: peripherals.kinematics,
			bed_size: configuration.bed_size,
			current_move: None,
			last_planned_move_end_position: None,
			homing_procedure: HomingProcedure::None,
			tmc2209_drivers: [
				TMC2209::new_using_uart(
					configuration.left_motor.tmc2209_address,
					uart_driver,
					MicrostepsPerStep::M16,
				)
				.map_err(CreationError::CreateLeftTMC2209)?,
				TMC2209::new_using_uart(
					configuration.right_motor.tmc2209_address,
					uart_driver,
					MicrostepsPerStep::M16,
				)
				.map_err(CreationError::CreateRightTMC2209)?,
				TMC2209::new_using_uart(
					configuration.z_axis_motor.tmc2209_address,
					uart_driver,
					MicrostepsPerStep::M16,
				)
				.map_err(CreationError::CreateZAxisTMC2209)?,
				TMC2209::new_using_uart(
					configuration.extruder_motor.tmc2209_address,
					uart_driver,
					MicrostepsPerStep::M16,
				)
				.map_err(CreationError::CreateExtruderTMC2209)?,
			],
			rotations_to_linear_motions: [
				configuration.left_motor.rotation_to_linear_motion,
				configuration.right_motor.rotation_to_linear_motion,
				configuration.z_axis_motor.rotation_to_linear_motion,
				configuration.extruder_motor.rotation_to_linear_motion,
			],
			next_move_feed_rate: DEFAULT_FEED_RATE,
			z_endstop,
			is_paused: false,
		})
	}

	/// Make the tool move to the provided coordinates after the previous moves are completed (and after
	/// [`Self::mark_last_move_as_ready_to_go`] has been called!).
	///
	/// `x`, `y`, `z`, `e` (extruder) are the movements along each respective axis, and for the ones of them that are set
	/// to `None` there won't be any movement along that axis.
	///
	/// The optional `feed_rate` will determine the speed of not only this move, but also all the subsequent ones.
	///
	/// Returns `Err(BlocksBufferIsFull)` if the move couldn't be planned, and you **MUST** call this method again
	/// to try to plan it!
	///
	/// # Warning
	/// This motion controller must be [`ticked`] to effectively execute the moves.
	///
	/// [`ticked`]: Self::tick
	pub fn plan_move(
		&mut self, x: Option<Distance>, y: Option<Distance>, z: Option<Distance>, e: Option<Distance>,
		feed_rate: Option<f32>,
	) -> Result<MoveId, BlocksBufferIsFull>
	{
		if let Some(feed_rate) = feed_rate
		{
			self.next_move_feed_rate = feed_rate;
		}

		let mut last_planned_move_end_position = self.last_planned_move_end_position.clone().unwrap_or_default();
		let mut apply_movement = |movement, axis| {
			if let Some(movement) = movement
			{
				last_planned_move_end_position[axis as usize] = movement;
			}
		};
		(apply_movement)(x, Axis::X);
		(apply_movement)(y, Axis::Y);
		(apply_movement)(z, Axis::Z);
		(apply_movement)(e, Axis::E);
		self.last_planned_move_end_position = Some(last_planned_move_end_position);

		let mut target_position = self.planner.get_position().clone();
		let mut set_target_position_axis = |distance, index| {
			if let Some(distance) = distance
			{
				target_position[index] = distance;
			}
		};
		(set_target_position_axis)(x, Axis::X as usize);
		(set_target_position_axis)(y, Axis::Y as usize);
		(set_target_position_axis)(z, Axis::Z as usize);
		(set_target_position_axis)(e, Axis::E as usize);

		self.planner.plan_move::<Kinematics>(
			target_position,
			calculate_microsteps_per_mm(&self.rotations_to_linear_motions, &self.tmc2209_drivers),
			self.next_move_feed_rate,
		)
	}

	/// Make the last [`planned move`] ready to be executed.
	///
	/// [`planned move`]: Self::plan_move
	pub fn mark_last_move_as_ready_to_go(&mut self)
	{
		self.planner.mark_last_added_move_as_ready_to_go()
	}

	pub fn has_move_been_executed(&self, move_to_check: MoveId) -> bool
	{
		self.planner.has_move_been_executed(move_to_check)
	}

	/// Returns the position of the last [`planned move`] if there has been one, otherwise returns `None`.
	///
	/// [`planned move`]: Self::plan_move
	pub fn get_last_planned_move_end_position(&self) -> Option<VectorN<N_MOTORS>>
	{
		self.last_planned_move_end_position.clone()
	}

	pub fn set_position(&mut self, x: Option<Distance>, y: Option<Distance>, z: Option<Distance>, e: Option<Distance>)
	{
		let mut position = self.planner.get_position().clone();
		let mut apply_position_axis = |value, axis| {
			if let Some(value) = value
			{
				position[axis as usize] = value;
			}
		};
		(apply_position_axis)(x, Axis::X);
		(apply_position_axis)(y, Axis::Y);
		(apply_position_axis)(z, Axis::Z);
		(apply_position_axis)(e, Axis::E);

		self.planner.set_position(position);
	}

	/// This method internally ticks the [`HomingProcedure`] and the [`Planner`], executing the planned moves.
	pub fn tick(&mut self) -> Result<(), homing::TickError<Probe<ZEndstop>>>
	{
		if self.is_paused
		{
			return Ok(());
		}

		self.homing_procedure.tick::<N_MOTORS, Kinematics, _>(
			&mut self.planner,
			|| calculate_microsteps_per_mm(&self.rotations_to_linear_motions, &self.tmc2209_drivers),
			&mut self.z_endstop,
			self.bed_size,
		)?;

		if let Some(current_move_steps_difference) = self.planner.tick()
		{
			let steps_to_distance = |steps_index: usize| {
				let steps = current_move_steps_difference[steps_index];
				self.rotations_to_linear_motions[steps_index].microsteps_to_distance(steps)
			};
			let a_distance = (steps_to_distance)(0);
			let b_distance = (steps_to_distance)(1);

			let steps_difference = VectorN::new([
				Kinematics::ab_displacement_to_x(a_distance, b_distance),
				Kinematics::ab_displacement_to_y(a_distance, b_distance),
				(steps_to_distance)(Axis::Z as usize),
				(steps_to_distance)(Axis::E as usize),
			]);

			let last_end_position = self
				.current_move
				.as_ref()
				.map(|current_move| current_move.end_position.clone())
				.unwrap_or(VectorN::ZERO);
			self.current_move = Some(CurrentMove {
				start_position: last_end_position.clone(),
				end_position: steps_difference + &last_end_position,
				start_time: self.ticker.get_time().ok(),
			});
		}

		Ok(())
	}

	pub fn set_paused(&mut self, paused: bool) -> Result<(), SetPausedError<Timer>>
	{
		if self.is_paused != paused
		{
			self.is_paused = paused;

			match paused
			{
				true => self.ticker.disable().map_err(SetPausedError::TryingToPause)?,
				false => self.ticker.enable().map_err(SetPausedError::TryingToResume)?,
			}
		}

		Ok(())
	}

	/// Make the machine start the [`HomingProcedure`] after all the planned moves are completed.
	///
	/// Returns `Err(BlocksBufferIsFull)` if the procedure couldn't be started, and you **MUST** call this method again
	/// to try to start it!
	pub fn start_homing(&mut self) -> Result<(), BlocksBufferIsFull>
	{
		self.homing_procedure
			.start_homing::<N_MOTORS, Kinematics>(&mut self.planner, || {
				calculate_microsteps_per_mm(&self.rotations_to_linear_motions, &self.tmc2209_drivers)
			})?;
		ticker::start_homing();

		Ok(())
	}

	/// TODO!
	pub fn start_bed_leveling(&mut self)
	{
		todo!();
	}

	/// Returns a mutable reference to the [`TMC2209`] driver you provided to [`Self::new`]
	/// that drives the stepper motor on the specific `axis`.
	///
	/// # Warning
	/// It's better if you don't modify any setting of any TMC2209 driver while there is a print going on.
	/// This is because talking to the chip needs some time, and even delays are used in internal TMC2209 functions.
	/// A delay could block the [`StepperMotorsTicker`] from doing its job resulting in missed steps and bad prints.
	///
	/// Consider changing all the TMC2209's settings before or after a print.
	pub fn get_tmc2209_driver(&mut self, axis: Axis) -> &mut TMC2209
	{
		&mut self.tmc2209_drivers[axis as usize]
	}
}

struct CurrentMove
{
	start_position: VectorN<N_MOTORS>,
	end_position: VectorN<N_MOTORS>,
	start_time: Option<Duration>,
}

/// Data necessary to create a [`MotionController`].
pub struct CreationParameters<
	Timer: TimerTrait,
	Kinematics: KinematicsTrait,
	LeftDirPin: OutputPin + 'static,
	LeftStepPin: OutputPin + 'static,
	RightDirPin: OutputPin + 'static,
	RightStepPin: OutputPin + 'static,
	ZAxisDirPin: OutputPin + 'static,
	ZAxisStepPin: OutputPin + 'static,
	ExtruderDirPin: OutputPin + 'static,
	ExtruderStepPin: OutputPin + 'static,
	XEndstop: Endstop,
	YEndstop: Endstop,
	ZEndstop: ZAxisProbe,
> {
	pub left_motor: StepperMotor<LeftDirPin, LeftStepPin>,
	pub right_motor: StepperMotor<RightDirPin, RightStepPin>,
	pub z_axis_motor: StepperMotor<ZAxisDirPin, ZAxisStepPin>,
	pub extruder_motor: StepperMotor<ExtruderDirPin, ExtruderStepPin>,

	pub ticker_timer: Timer,

	pub kinematics: Kinematics,

	pub x_endstop: XEndstop,
	pub y_endstop: YEndstop,
	pub z_endstop: ZEndstop,
}

pub struct CreationConfig
{
	pub left_motor: MotorConfig,
	pub right_motor: MotorConfig,
	pub z_axis_motor: MotorConfig,
	pub extruder_motor: MotorConfig,

	pub bed_size: Vector2,

	pub offset_from_nozzle_of_z_probe: Vector3,

	pub planner_blocks_count: usize,
	pub planner_settings: Settings<N_MOTORS>,
}

/// Data necessary to configure a stepper motor.
pub struct MotorConfig
{
	pub tmc2209_address: UARTAddress,
	pub rotation_to_linear_motion: RotationToLinearMotion,
}

/// An error that can occur when you instatiate a [`MotionController`] struct.
#[derive(Debug)]
pub enum CreationError<Timer: TimerTrait, ZEndstop: ZAxisProbe, Uart: UartTrait>
{
	CreateStepperTickerTimer(ticker::CreationError<Timer, ZEndstop>),
	EnableStepperTickerTimer(ticker::EnableError<Timer>),

	CreateLeftTMC2209(Uart::Error),
	CreateRightTMC2209(Uart::Error),
	CreateZAxisTMC2209(Uart::Error),
	CreateExtruderTMC2209(Uart::Error),
}

/// An error that can occur when you [`pause or resume`] a [`MotionController`].
///
/// [`pause or resume`]: MotionController::set_paused
pub enum SetPausedError<Timer: TimerTrait>
{
	TryingToPause(Timer::Error),
	TryingToResume(ticker::EnableError<Timer>),
}

impl<Timer: TimerTrait> Debug for SetPausedError<Timer>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Self::TryingToPause(arg0) => f.debug_tuple("TryingToPause").field(arg0).finish(),
			Self::TryingToResume(arg0) => f.debug_tuple("TryingToResume").field(arg0).finish(),
		}
	}
}

fn calculate_microsteps_per_mm(
	rotations_to_linear_motions: &[RotationToLinearMotion; N_MOTORS], tmc2209_drivers: &[TMC2209; N_MOTORS],
) -> [f32; N_MOTORS]
{
	let mut microsteps_per_mm = [0.; N_MOTORS];
	for i in 0..N_MOTORS
	{
		let steps_per_mm = rotations_to_linear_motions[i].distance_to_microsteps(Distance::MILLIMETER) as f32;
		let microsteps_taken_per_step = tmc2209_drivers[i]
			.get_microsteps_per_step()
			.as_max_resolution_microsteps_count() as f32;
		microsteps_per_mm[i] = steps_per_mm / microsteps_taken_per_step;
	}

	microsteps_per_mm
}
