//! This module provides the implementation for controlling a servo motor using PWM signals.
//!
//! The [`ServoMotor`] struct allows for positioning a servo motor by sending pulse durations
//! on a specified pin. The [`ServoPosition`] struct represents the angle at which the servo
//! should be set.

use crate::{
	printer::components::hal::pwm::PwmPin,
	utils::{
		math::{self, Percentage},
		measurement::{angle::Angle, duration::SmallDuration, frequency::Frequency},
	},
};

/// The `50Hz` value usually used by servo motors.
pub const REFRESH_RATE: Frequency = Frequency::from_hertz(50);

const MIN_ANGLE: Angle = Angle::ZERO;
const MAX_ANGLE: Angle = Angle::FLAT;

/// A [`servo motor`] connected to the microcontroller that can be controlled by sending pulses on the `P` pin.
///
/// [`servo motor`]: <https://en.wikipedia.org/wiki/Servomotor>
pub struct ServoMotor<P: PwmPin, const MIN_PULSE_DURATION: u32 = 500, const MAX_PULSE_DURATION: u32 = 2500>
{
	pin: P,
	was_moved_before: bool,
}

impl<P: PwmPin, const MIN_PULSE_DURATION: u32, const MAX_PULSE_DURATION: u32>
	ServoMotor<P, MIN_PULSE_DURATION, MAX_PULSE_DURATION>
{
	/// Creates a [`ServoMotor`] that receives commands on the provided `pin`, settings the `pin`'s PWM frequency to [`REFRESH_RATE`].
	///
	/// Returns `Ok(ServoMotor)` if the pin's PWM frequency was correctly set, otherwise `Err(PwmPin::Error)`.
	///
	/// # Note
	/// Unlike some other servo libraries, this function won't move the servo motor to its [`neutral position`].
	///
	/// [`neutral position`]: `ServoPosition::NEUTRAL`
	pub fn new(mut pin: P) -> Result<Self, <P as PwmPin>::Error>
	{
		pin.set_frequency(REFRESH_RATE)?;

		Ok(Self {
			pin,
			was_moved_before: false,
		})
	}

	/// Sends a command to the servo motor to move to the provided `target_position`.
	///
	/// Returns `Ok(())` if the command was correctly sent, otherwise `Err(PwmPin::Error)`.
	pub fn move_to(
		&mut self, target_position: ServoPosition<MIN_PULSE_DURATION, MAX_PULSE_DURATION>,
	) -> Result<(), <P as PwmPin>::Error>
	{
		self.was_moved_before = true;

		self.pin.set_duty_cycle(target_position.as_duty_cycle())
	}

	/// Gets the `target position` you previously [`set`].
	///
	/// Returns `Some(ServoPosition)` if you ever set the target position before, otherwise `None`.
	///
	/// [`set`]: `Self::move_to`
	pub fn get_target_position(&self) -> Option<ServoPosition<MIN_PULSE_DURATION, MAX_PULSE_DURATION>>
	{
		self.was_moved_before.then_some(
			ServoPosition::from_angle(MAX_ANGLE * self.pin.get_duty_cycle().into_0_to_1())
				.unwrap_or(ServoPosition::NEUTRAL),
		)
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Position held by a [`ServoMotor`].
///
/// It can be [`send`] as a command to the servo motor to make it move at that position.
///
/// [`send`]: `ServoMotor::move_to`
pub struct ServoPosition<const MIN_PULSE_DURATION: u32 = 500, const MAX_PULSE_DURATION: u32 = 2500>(SmallDuration);
impl<const MIN_PULSE_DURATION: u32, const MAX_PULSE_DURATION: u32> ServoPosition<MIN_PULSE_DURATION, MAX_PULSE_DURATION>
{
	/// The neutral angle a [`ServoMotor`] holds (`90°`).
	pub const NEUTRAL: Self = Self(SmallDuration::from_micros(
		(MIN_PULSE_DURATION + MAX_PULSE_DURATION) / 2,
	));

	/// Creates a [`ServoPosition`] representing the provided `angle`.
	///
	/// Returns `Ok(Self)` if the `angle` is in the range a servo motor can hold `[0°, 180°]`, otherwise `Err(())`.
	pub fn from_angle(angle: Angle) -> Result<Self, ()>
	{
		Self::from_pulse_duration(SmallDuration::from_tens_of_nanos(math::map(
			angle.into_radians(),
			MIN_ANGLE.into_radians()..=MAX_ANGLE.into_radians(),
			100. * MIN_PULSE_DURATION as f32..=100. * MAX_PULSE_DURATION as f32,
		) as u32))
	}

	/// Creates a [`ServoPosition`] that has the provided [`pulse duration`].
	///
	/// Returns `Ok(Self)` if the duration is in the range a servo motor can hold `[500us, 2500us]`, otherwise `Err(())`.
	///
	/// [`pulse duration`]: https://en.wikipedia.org/wiki/Servo_control#Pulse_duration
	pub const fn from_pulse_duration(pulse_duration: SmallDuration) -> Result<Self, ()>
	{
		if pulse_duration.as_micros() < MIN_PULSE_DURATION || pulse_duration.as_micros() > MAX_PULSE_DURATION
		{
			return Err(());
		}

		Ok(Self(pulse_duration))
	}

	/// Returns the [`Angle`] this [`ServoPosition`] represents.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::printer::components::drivers::servo_motor::ServoPosition;
	/// # use firmware_core::utils::measurement::angle::deg;
	/// #
	/// assert_eq!(ServoPosition::<500, 2500>::from_angle(deg(0.)).unwrap().as_angle(), deg(0.));
	/// assert_eq!(ServoPosition::<500, 2500>::from_angle(deg(50.)).unwrap().as_angle(), deg(50.));
	/// assert_eq!(ServoPosition::<500, 2500>::from_angle(deg(150.)).unwrap().as_angle(), deg(150.));
	/// ```
	pub fn as_angle(&self) -> Angle
	{
		Angle::from_radians(math::map(
			self.0.as_tens_of_nanos() as f32,
			100. * MIN_PULSE_DURATION as f32..=100. * MAX_PULSE_DURATION as f32,
			MIN_ANGLE.into_radians()..=MAX_ANGLE.into_radians(),
		))
	}

	fn as_duty_cycle(&self) -> Percentage
	{
		Percentage::from_0_to_1(
			self.0.as_tens_of_nanos() as f32 / Into::<SmallDuration>::into(REFRESH_RATE).as_tens_of_nanos() as f32,
		)
		.unwrap()
	}
}
