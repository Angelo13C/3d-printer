use crate::utils::math::Percentage;

pub trait PwmPin
{
	type Error;

	fn get_duty_cycle(&self) -> Percentage;
	fn set_duty_cycle(&mut self, percentage: Percentage) -> Result<(), Self::Error>;
}
