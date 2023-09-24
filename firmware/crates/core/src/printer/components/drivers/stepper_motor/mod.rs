mod direction;
pub mod tmc2209;

pub use direction::*;
use embedded_hal::digital::{ErrorType, OutputPin};

/// A [`stepper motor`] connected to a [`stepper driver`] (like the [`TMC2209`]) which is then connected to the microcontroller through
/// the `DIR` pin and the `STEP` pin.
///
/// When a pulse is sent on the `STEP` pin (using first [`Self::start_step_pulse`] and then [`Self::end_step_pulse`]),
/// the stepper driver will make the motor take a step (or microstep) on a direction based on the voltage of the `DIR` pin.
///
/// [`stepper motor`]: <https://en.wikipedia.org/wiki/Stepper_motor>
/// [`stepper driver`]: <https://en.wikipedia.org/wiki/Driver_circuit>
/// [`TMC2209`]: `tmc2209::TMC2209`
pub struct StepperMotor<DirPin: OutputPin, StepPin: OutputPin>
{
	dir_pin: DirPin,
	step_pin: StepPin,
}

impl<DirPin: OutputPin, StepPin: OutputPin> StepperMotor<DirPin, StepPin>
{
	/// Returns a [`StepperMotor`] that can be controlled with the provided `dir_pin` and `step_pin`.
	///
	/// Check [`struct's documentation for more info`](Self).
	pub fn new(dir_pin: DirPin, step_pin: StepPin) -> Self
	{
		Self { dir_pin, step_pin }
	}

	/// Set the direction in which the step will be taken when you send a pulse on the STEP pin.
	///
	/// Check [`struct's documentation`](Self).
	///
	/// # Warning
	/// The direction actually depends also on how the stepper motor is wired to the stepper driver.
	/// If it is wired in reverse, even the steps it will take are going to be in the opposite direction.
	pub fn set_rotation_direction(&mut self, direction: RotationalDirection)
		-> Result<(), <DirPin as ErrorType>::Error>
	{
		match direction
		{
			RotationalDirection::CW => self.dir_pin.set_high(),
			RotationalDirection::CCW => self.dir_pin.set_low(),
		}
	}

	/// Check [`struct's documentation`](Self).
	pub fn start_step_pulse(&mut self) -> Result<(), <StepPin as ErrorType>::Error>
	{
		self.step_pin.set_high()
	}

	/// Check [`struct's documentation`](Self).
	pub fn end_step_pulse(&mut self) -> Result<(), <StepPin as ErrorType>::Error>
	{
		self.step_pin.set_low()
	}
}
