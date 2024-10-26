//! This module provides the implementation for a fan interface that can be controlled
//! through a PWM signal in a 3D printer.
//!
//! The `Fan` struct allows for speed control of the fan, which is dependent on a
//! minimum duty cycle to start spinning.

use crate::{
	printer::components::hal::pwm::PwmPin,
	utils::math::{self, Percentage},
};

/// A fan connected to the microcontroller that can be controlled using the `P` pin.
pub struct Fan<P: PwmPin>
{
	pin: P,
	/// The minimum duty cycle required to make the fan start moving.
	minimum_duty_cycle_fan_moves: Percentage,
}

impl<P: PwmPin> Fan<P>
{
	/// Returns a [`Fan`] that can control its speed through the provided `pin`, and starts moving when
	/// the [`speed set`] is greater or equal to `minimum_duty_cycle_fan_moves`.
	///
	/// # Warning
	/// The speed that you'll later set using [`Self::set_speed`] will be remapped to the moveable range.
	/// For example this:
	/// ```
	/// # /*
	/// let mut fan = Fan::new(pin, Percentage::from_0_to_1(0.5).unwrap());
	/// fan.set_speed(Percentage::from_0_to_1(0.8).unwrap());
	/// # */
	/// ```
	/// will internally set a duty cycle of 90%, but since the fan starts moving at a 50% duty cycle,
	/// it will effectively move at the 80% of its maximum speed.
	///
	/// [`speed set`]: `Self::set_speed`
	pub fn new(pin: P, minimum_duty_cycle_fan_moves: Percentage) -> Self
	{
		Self {
			pin,
			minimum_duty_cycle_fan_moves,
		}
	}

	/// Sets the speed percentage at which the fan rotates.
	///
	/// Returns `Ok(())` if the speed was set correctly, otherwise returns `Err(error)`.
	///
	/// Check [`Self::new`] for more info.
	pub fn set_speed(&mut self, mut speed: Percentage) -> Result<(), <P as PwmPin>::Error>
	{
		if speed.into_0_to_1() > 0.
		{
			speed = Percentage::from_0_to_1(math::map(
				speed.into_0_to_1(),
				self.minimum_duty_cycle_fan_moves.into_0_to_1()..=1.,
				0 as f32..=1.,
			))
			.unwrap();
		}

		self.pin.set_duty_cycle(speed)
	}
}
