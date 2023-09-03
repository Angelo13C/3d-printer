use crate::utils::math::Percentage;

pub trait PwmPin
{
	type Error;

	fn set_duty_cycle(&mut self, percentage: Percentage) -> Result<(), Self::Error>;
}
