//! PWM (Pulse Width Modulation) module.
//!
//! This module defines a trait for PWM pins, allowing control over the duty cycle
//! and frequency of the PWM signal. It provides methods to set and get the duty
//! cycle and frequency for PWM operations.

use std::fmt::Debug;

use crate::utils::{math::Percentage, measurement::frequency::Frequency};

/// Trait representing a PWM-capable pin.
pub trait PwmPin
{
	/// The type of error that can occur when setting the PWM parameters.
	type Error: Debug;

	/// Gets the current duty cycle of the PWM signal.
	///
	/// Returns the duty cycle as a percentage.
	fn get_duty_cycle(&self) -> Percentage;

	/// Sets the duty cycle of the PWM signal.
	///
	/// # Parameters
	/// - `percentage`: The desired duty cycle as a percentage.
	fn set_duty_cycle(&mut self, percentage: Percentage) -> Result<(), Self::Error>;

	/// Sets the frequency of the PWM signal.
	///
	/// # Parameters
	/// - `frequency`: The desired frequency for the PWM signal.
	fn set_frequency(&mut self, frequency: Frequency) -> Result<(), Self::Error>;
}
