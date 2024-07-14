use core::{
	fmt::Debug,
	sync::atomic::{AtomicBool, Ordering},
	time::Duration,
};

use embedded_hal::digital::OutputPin;

use super::{
	bed_leveling::{Probe, UnifiedBedLevelingProcedure, ZAxisProbe},
	homing::endstop::Endstop,
	planner::{self, Flag},
	N_MOTORS,
};
use crate::{
	printer::components::{
		drivers::stepper_motor::*,
		hal::timer::{self, Timer as TimerTrait, TimerAdditionalFunctionality},
		motion::Axis,
	},
	utils::{
		bresenham::Bresenham,
		math::vectors::Vector2,
		measurement::{distance::Distance, duration::SmallDuration, frequency::Frequency},
	},
};

macro_rules! generate_step_pulses {
	($($should_step: expr => $motor: expr),+) => {
		$(
			if $should_step
			{
				$motor.start_step_pulse().unwrap();
			}
		)+
		$(
			if $should_step
			{
				$motor.end_step_pulse().unwrap();
			}
		)+
	};
}

const DELAY_BETWEEN_TICKS_WITH_NO_MOVEMENT: Duration = Duration::from_micros(500);

/// Attaches an interrupt to the timer you provide to [`Self::new`] that checks if stepper motors should take a step
/// based on the [`planned block`] [`communicated to the ticker`], and eventually makes them take the step.
///
/// [`planned block`]: planner::Block
/// [`communicated to the ticker`]: planner::communicate_to_ticker
pub struct StepperMotorsTicker<Timer: TimerTrait>
{
	timer: Timer,
}

impl<Timer: TimerTrait> StepperMotorsTicker<Timer>
{
	/// Returns `Err(Timer::Error)` if there was an error while [`setting the callback of the timer`], otherwise
	/// in case of success returns `Ok(Self)`.
	///
	/// # Warning
	/// You must call [`Self::enable`] after creating this instance to actually make the ticker work.
	///
	/// [`setting the callback of the timer`]: `TimerTrait::on_alarm`
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
		ZEndstop: ZAxisProbe,
	>(
		left_motor: StepperMotor<LeftDirPin, LeftStepPin>, right_motor: StepperMotor<RightDirPin, RightStepPin>,
		z_axis_motor: StepperMotor<ZAxisDirPin, ZAxisStepPin>,
		extruder_motor: StepperMotor<ExtruderDirPin, ExtruderStepPin>, mut timer: Timer, x_endstop: XEndstop,
		y_endstop: YEndstop, z_endstop: &mut Probe<ZEndstop>,
	) -> Result<Self, CreationError<Timer, ZEndstop>>
	{
		let mut tick_parameters = TickParameters {
			left_motor,
			right_motor,
			z_axis_motor,
			extruder_motor,
			timer: timer.get_additional_functionality(),
			timer_frequency: timer.get_clock_frequency(),
			block_parameters: None,
			x_endstop,
			y_endstop,
			accel_step_rate: DurationInTicks(0),
		};

		unsafe {
			z_endstop
				.on_end_reached(|| Z_AXIS_ENDSTOP_TRIGGERED.store(true, Ordering::Relaxed))
				.map_err(CreationError::OnEndstopTriggered)?;
		}

		unsafe {
			timer
				.on_alarm(move || Self::tick_isr(&mut tick_parameters))
				.map_err(CreationError::OnAlarm)?
		};

		Ok(Self { timer })
	}

	/// Enable the inner timer so that the ticker can make the stepper motors move (by pulsing the STEP pin at the right moment
	/// and with the right value on the DIR pin).
	pub fn enable(&mut self) -> Result<(), EnableError<Timer>>
	{
		self.timer.enable_alarm(true).map_err(EnableError::EnableAlarm)?;

		const START_TIME: Duration = Duration::from_millis(100);
		let mut timer_functionality = self.timer.get_additional_functionality();
		let alarm_time = START_TIME + timer_functionality.get_time().map_err(EnableError::SetAlarm)?;
		timer_functionality
			.set_alarm(alarm_time)
			.map_err(EnableError::SetAlarm)?;

		Ok(())
	}

	/// Disable the inner timer, stopping this ticker from making the stepper motors move.
	pub fn disable(&mut self) -> Result<(), <Timer as TimerTrait>::Error>
	{
		self.timer.enable_alarm(false)
	}

	/// Get the frequency of the inner timer. This represents the minimum interval of time between two steps taken by a stepper motor.
	pub fn get_tick_frequency(&self) -> Frequency
	{
		self.timer.get_clock_frequency()
	}

	pub fn get_time(&self)
		-> Result<Duration, <Timer::AdditionalFunctionality as TimerAdditionalFunctionality>::Error>
	{
		self.timer.get_additional_functionality().get_time()
	}

	fn tick_isr<
		LeftDirPin: OutputPin,
		LeftStepPin: OutputPin,
		RightDirPin: OutputPin,
		RightStepPin: OutputPin,
		ZAxisDirPin: OutputPin,
		ZAxisStepPin: OutputPin,
		ExtruderDirPin: OutputPin,
		ExtruderStepPin: OutputPin,
		XEndstop: Endstop,
		YEndstop: Endstop,
	>(
		parameters: &mut TickParameters<
			LeftDirPin,
			LeftStepPin,
			RightDirPin,
			RightStepPin,
			ZAxisDirPin,
			ZAxisStepPin,
			ExtruderDirPin,
			ExtruderStepPin,
			Timer::AdditionalFunctionality,
			XEndstop,
			YEndstop,
		>,
	)
	{
		let start_isr_time = parameters.timer.get_time_in_ticks().unwrap();
		let mut next_isr_tick = DurationInTicks(timer::duration_to_counter(
			DELAY_BETWEEN_TICKS_WITH_NO_MOVEMENT,
			parameters.timer_frequency,
		) as u32);

		let speed_to_next_isr_tick =
			|duration: DurationInTicks| DurationInTicks(parameters.timer_frequency.as_hertz() / duration.0);
		let mul_steps = |first: u32, second: u32| ((((first as u64) * (second as u64)) + (1 << 23)) >> 24) as u32;

		if let Some(mut communication) = planner::communicate_to_ticker::get_blocks()
		{
			let mut new_block = true;

			let kinematics_functions = communication.kinematics_functions.clone();

			let mut z_axis_distance = None;

			if let Some(block) = communication.current_motion_profile_block.as_mut()
			{
				let mut is_end_reached = false;
				if block.flags.contains(Flag::Homing)
				{
					if let Some(kinematics_conversion) = kinematics_functions
					{
						// This function is needed only because the kinematics functions take `Distance` as parameters instead of `i32`
						let steps_to_distance = |index| Distance::from_tens_of_nanometers(block.steps[index]);

						let a = (steps_to_distance)(0);
						let b = (steps_to_distance)(1);
						let homing_axis = [
							(kinematics_conversion.ab_to_x)(a, b),
							(kinematics_conversion.ab_to_y)(a, b),
							(steps_to_distance)(2),
						]
						.into_iter()
						.enumerate()
						.max_by(|(_, a), (_, b)| a.as_tens_of_nanometers().cmp(&b.as_tens_of_nanometers()))
						.map(|(index, _)| index)
						.unwrap() as u8;

						is_end_reached = match homing_axis
						{
							0 => parameters.x_endstop.is_end_reached().unwrap_or(false),
							1 => parameters.y_endstop.is_end_reached().unwrap_or(false),
							2 => is_z_axis_triggered(),
							_ => panic!("Invalid homing axis index"),
						};
					}
				}

				if block.flags.contains(Flag::BedLeveling) && is_z_axis_triggered()
				{
					is_end_reached = true;

					let block_parameters = parameters.block_parameters.as_ref().unwrap();
					// This assumes that a move with the Flag::BedLevelingProbe moves only 1 motor which is the Z axis motor
					let travelled_distance_along_z_axis = (block_parameters.bresenham.steps_taken()
						* block.travelled_z_distance.as_tens_of_nanometers() as u32)
						/ block.step_event_count;
					z_axis_distance = Some(Distance::from_tens_of_nanometers(
						travelled_distance_along_z_axis as i32,
					));
				}

				if !is_end_reached
				{
					let block_parameters = parameters.block_parameters.get_or_insert_with(|| {
						fn set_motor_direction<DirPin: OutputPin, StepPin: OutputPin>(
							motor: &mut StepperMotor<DirPin, StepPin>, steps: i32,
						)
						{
							motor.set_rotation_direction(RotationalDirection::from_sign(steps));
						}
						(set_motor_direction)(&mut parameters.left_motor, block.steps[Axis::X as usize]);
						(set_motor_direction)(&mut parameters.right_motor, block.steps[Axis::Y as usize]);
						(set_motor_direction)(&mut parameters.z_axis_motor, block.steps[Axis::Z as usize]);
						(set_motor_direction)(&mut parameters.extruder_motor, block.steps[Axis::E as usize]);

						BlockTickParameters {
							acceleration_time: DurationInTicks(0),
							deceleration_time: DurationInTicks(0),
							bresenham: Bresenham::new([0; N_MOTORS], block.steps),
						}
					});

					if let Some(motors_that_take_steps) = block_parameters.bresenham.next()
					{
						let [a, b, c, e] = motors_that_take_steps;

						new_block = false;
						generate_step_pulses!(a => parameters.left_motor,
									b => parameters.right_motor,
									c => parameters.z_axis_motor,
									e => parameters.extruder_motor);

						if block_parameters.bresenham.steps_taken() <= block.accelerate_until
						{
							parameters.accel_step_rate = DurationInTicks(
								(mul_steps)(block_parameters.acceleration_time.0, block.acceleration_rate)
									+ block.initial_speed,
							);
							parameters.accel_step_rate =
								parameters.accel_step_rate.min(DurationInTicks(block.nominal_speed));

							next_isr_tick = (speed_to_next_isr_tick)(parameters.accel_step_rate);
							block_parameters.acceleration_time += next_isr_tick;
						}
						else if block_parameters.bresenham.steps_taken() > block.decelerate_after
						{
							let mut step_rate = DurationInTicks((mul_steps)(
								block_parameters.deceleration_time.0,
								block.acceleration_rate,
							));
							if step_rate < parameters.accel_step_rate
							{
								step_rate =
									(parameters.accel_step_rate - step_rate).max(DurationInTicks(block.final_speed));
							}
							else
							{
								step_rate = DurationInTicks(block.final_speed);
							}

							next_isr_tick = (speed_to_next_isr_tick)(step_rate);
							block_parameters.deceleration_time += next_isr_tick;
						}
						else
						{
							next_isr_tick = (speed_to_next_isr_tick)(DurationInTicks(block.nominal_speed));
						}
					}
				}
			}

			if let Some(z_axis_distance) = z_axis_distance
			{
				communication.set_z_axis_distance(z_axis_distance);
			}

			if new_block
			{
				parameters.block_parameters = None;
				communication.finish_using_current_block();
			}
		}

		// The timer's counter is not resetted, so instead of simply setting the alarm to the `next_isr_tick` the current
		// time (`start_isr_time`) is added to it
		let alarm = start_isr_time + next_isr_tick.0 as u64;
		parameters.timer.set_alarm_in_ticks(alarm).unwrap();
	}
}

static Z_AXIS_ENDSTOP_TRIGGERED: AtomicBool = AtomicBool::new(false);

/// This function must be called each time you start homing (it is called by the [`MotionController`]).
///
/// [`MotionController`]: `super::MotionController`
pub fn start_homing()
{
	Z_AXIS_ENDSTOP_TRIGGERED.store(false, Ordering::Relaxed)
}

/// Returns `true` if the endstop on the Z axis has been triggered since the last time you called either this function or [`start_homing`],
/// otherwise `false`.
fn is_z_axis_triggered() -> bool
{
	Z_AXIS_ENDSTOP_TRIGGERED.swap(false, Ordering::Relaxed)
}

#[derive(Debug)]
pub enum CreationError<Timer: TimerTrait, ZEndstop: ZAxisProbe>
{
	OnAlarm(Timer::Error),
	OnEndstopTriggered(ZEndstop::OnEndReachedError),
}

pub enum EnableError<Timer: TimerTrait>
{
	EnableAlarm(Timer::Error),
	SetAlarm(<Timer::AdditionalFunctionality as TimerAdditionalFunctionality>::Error),
}

impl<Timer: TimerTrait> Debug for EnableError<Timer>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Self::EnableAlarm(arg0) => f.debug_tuple("EnableAlarm").field(arg0).finish(),
			Self::SetAlarm(arg0) => f.debug_tuple("SetAlarm").field(arg0).finish(),
		}
	}
}

struct TickParameters<
	LeftDirPin: OutputPin + 'static,
	LeftStepPin: OutputPin + 'static,
	RightDirPin: OutputPin + 'static,
	RightStepPin: OutputPin + 'static,
	ZAxisDirPin: OutputPin + 'static,
	ZAxisStepPin: OutputPin + 'static,
	ExtruderDirPin: OutputPin + 'static,
	ExtruderStepPin: OutputPin + 'static,
	Timer: TimerAdditionalFunctionality,
	XEndstop: Endstop,
	YEndstop: Endstop,
> {
	left_motor: StepperMotor<LeftDirPin, LeftStepPin>,
	right_motor: StepperMotor<RightDirPin, RightStepPin>,
	z_axis_motor: StepperMotor<ZAxisDirPin, ZAxisStepPin>,
	extruder_motor: StepperMotor<ExtruderDirPin, ExtruderStepPin>,
	timer: Timer,
	timer_frequency: Frequency,

	x_endstop: XEndstop,
	y_endstop: YEndstop,

	block_parameters: Option<BlockTickParameters>,

	accel_step_rate: DurationInTicks,
}

#[derive(Debug)]
struct BlockTickParameters
{
	acceleration_time: DurationInTicks,
	deceleration_time: DurationInTicks,
	bresenham: Bresenham<N_MOTORS>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct DurationInTicks(u32);
impl std::ops::Add for DurationInTicks
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output
	{
		Self(self.0 + rhs.0)
	}
}
impl std::ops::AddAssign for DurationInTicks
{
	fn add_assign(&mut self, rhs: Self)
	{
		*self = *self + rhs
	}
}
impl std::ops::Sub for DurationInTicks
{
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output
	{
		Self(self.0 - rhs.0)
	}
}
impl std::ops::Mul<u32> for DurationInTicks
{
	type Output = Self;

	fn mul(self, rhs: u32) -> Self::Output
	{
		Self(self.0 * rhs)
	}
}
