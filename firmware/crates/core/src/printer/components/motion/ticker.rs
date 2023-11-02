use std::{
	fmt::Debug,
	sync::atomic::{AtomicBool, Ordering},
	time::Duration,
};

use embedded_hal::digital::OutputPin;

use super::{
	bed_leveling::{Probe, ZAxisProbe},
	homing::endstop::Endstop,
	planner::{self, Flag},
	N_MOTORS,
};
use crate::{
	printer::components::{
		drivers::stepper_motor::StepperMotor,
		hal::timer::{Timer as TimerTrait, TimerAdditionalFunctionality},
	},
	utils::{
		bresenham::Bresenham,
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
		LeftDirPin: OutputPin + 'static,
		LeftStepPin: OutputPin + 'static,
		RightDirPin: OutputPin + 'static,
		RightStepPin: OutputPin + 'static,
		ZAxisDirPin: OutputPin + 'static,
		ZAxisStepPin: OutputPin + 'static,
		ExtruderDirPin: OutputPin + 'static,
		ExtruderStepPin: OutputPin + 'static,
		XEndstop: Endstop + 'static,
		YEndstop: Endstop + 'static,
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
			block_parameters: None,
			x_endstop,
			y_endstop,
			accel_step_rate: 0.,
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

		const START_TIME: Duration = Duration::from_micros(1);
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
		let start_isr_time = parameters.timer.get_time().unwrap().as_nanos() as u64;
		let mut next_isr_tick = SmallDuration::from_micros(1);

		if let Some(mut communication) = planner::communicate_to_ticker::get_blocks()
		{
			let mut new_block = true;

			let kinematics_functions = communication.kinematics_functions.clone();

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

				if !is_end_reached
				{
					let block_parameters = parameters.block_parameters.get_or_insert_with(|| BlockTickParameters {
						acceleration_time: SmallDuration::ZERO,
						deceleration_time: SmallDuration::ZERO,
						bresenham: Bresenham::new([0; N_MOTORS], block.steps),
					});

					if let Some(motors_that_take_steps) = block_parameters.bresenham.next()
					{
						new_block = false;

						block.step_event_count += motors_that_take_steps.iter().filter(|&v| *v).count() as u32;
						let [a, b, c, e] = motors_that_take_steps;

						generate_step_pulses!(a => parameters.left_motor,
									b => parameters.right_motor,
									c => parameters.z_axis_motor,
									e => parameters.extruder_motor);

						if block.step_event_count <= block.accelerate_until
						{
							parameters.accel_step_rate = block_parameters.acceleration_time.as_seconds_f32()
								* block.acceleration_rate as f32 + block.initial_speed;
							parameters.accel_step_rate = parameters.accel_step_rate.min(block.nominal_speed);

							next_isr_tick = SmallDuration::from_seconds_f32(parameters.accel_step_rate);
							block_parameters.acceleration_time += next_isr_tick;
						}
						else if block.step_event_count > block.decelerate_after
						{
							let mut step_rate =
								block_parameters.deceleration_time.as_seconds_f32() * block.acceleration_rate as f32;
							if step_rate < parameters.accel_step_rate
							{
								step_rate = (parameters.accel_step_rate - step_rate).max(block.final_speed);
							}
							else
							{
								step_rate = block.final_speed;
							}

							next_isr_tick = SmallDuration::from_seconds_f32(step_rate);
							block_parameters.deceleration_time += next_isr_tick;
						}
						else
						{
							next_isr_tick = SmallDuration::from_seconds_f32(block.nominal_speed);
						}
					}
				}
			}

			if new_block
			{
				parameters.block_parameters = None;
				communication.finish_using_current_block();
			}
		}

		// The timer's counter is not resetted, so instead of simply setting the alarm to the `next_isr_tick` the current
		// time (`start_isr_time`) is added to it
		let alarm = Duration::from_nanos(next_isr_tick.as_nanos() + start_isr_time);
		parameters.timer.set_alarm(alarm).unwrap();
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

	x_endstop: XEndstop,
	y_endstop: YEndstop,

	block_parameters: Option<BlockTickParameters>,

	accel_step_rate: f32,
}

struct BlockTickParameters
{
	acceleration_time: SmallDuration,
	deceleration_time: SmallDuration,
	bresenham: Bresenham<N_MOTORS>,
}
