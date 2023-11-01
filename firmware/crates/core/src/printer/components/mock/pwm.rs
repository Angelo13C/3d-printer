use crate::printer::components::hal::pwm::PwmPin;

pub struct MockPwmPin;
impl PwmPin for MockPwmPin
{
	type Error = ();

	fn get_duty_cycle(&self) -> crate::utils::math::Percentage
	{
		todo!()
	}

	fn set_duty_cycle(&mut self, _: crate::utils::math::Percentage) -> Result<(), Self::Error>
	{
		todo!()
	}

	fn set_frequency(&mut self, _: crate::utils::measurement::frequency::Frequency) -> Result<(), Self::Error>
	{
		todo!()
	}
}
