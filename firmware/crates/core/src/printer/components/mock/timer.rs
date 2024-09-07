use crate::{
	printer::components::hal::timer::{Timer, TimerAdditionalFunctionality},
	utils::measurement::frequency::Frequency,
};

pub struct MockTimer;
impl Timer for MockTimer
{
	type Error = ();
	type AdditionalFunctionality = MockTimerAdditionalFunctionality;

	fn get_additional_functionality(&self) -> Self::AdditionalFunctionality
	{
		MockTimerAdditionalFunctionality
	}

	fn get_clock_frequency(&self) -> crate::utils::measurement::frequency::Frequency
	{
		Frequency::from_hertz(1_000_000)
	}

	unsafe fn on_alarm(&mut self, _: impl FnMut() + 'static) -> Result<(), Self::Error>
	{
		todo!()
	}

	fn enable_alarm(&mut self, _: bool) -> Result<(), Self::Error>
	{
		todo!()
	}

	fn get_alarm_in_ticks(&self) -> Result<u64, Self::Error>
	{
		todo!()
	}
}

pub struct MockTimerAdditionalFunctionality;
impl TimerAdditionalFunctionality for MockTimerAdditionalFunctionality
{
	type Error = ();

	fn set_alarm(&mut self, _: std::time::Duration) -> Result<(), Self::Error>
	{
		todo!()
	}

	fn get_time(&self) -> Result<std::time::Duration, Self::Error>
	{
		todo!()
	}

	fn set_alarm_in_ticks(&mut self, ticks: u64) -> Result<(), Self::Error>
	{
		todo!()
	}

	fn get_time_in_ticks(&self) -> Result<u64, Self::Error>
	{
		todo!()
	}
}
