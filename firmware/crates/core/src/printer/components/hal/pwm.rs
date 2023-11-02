use std::fmt::Debug;

use crate::utils::{math::Percentage, measurement::frequency::Frequency};

pub trait PwmPin
{
	type Error: Debug;

	fn get_duty_cycle(&self) -> Percentage;
	fn set_duty_cycle(&mut self, percentage: Percentage) -> Result<(), Self::Error>;

	fn set_frequency(&mut self, frequency: Frequency) -> Result<(), Self::Error>;
}
