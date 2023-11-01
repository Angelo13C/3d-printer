use crate::{
	printer::components::hal::adc::{Adc, AdcPin},
	utils::math::Percentage,
};

pub struct MockAdcPin;
impl AdcPin<MockAdc> for MockAdcPin
{
	type Error = ();

	fn read(&mut self, _: &mut MockAdc) -> Result<<MockAdc as Adc>::ReadableValue, Self::Error>
	{
		todo!()
	}
}

pub struct MockAdc;
impl Adc for MockAdc
{
	type ReadableValue = MockAdcReading;

	fn max_readable_value(&self) -> Self::ReadableValue
	{
		todo!()
	}
}

pub struct MockAdcReading(u16);
impl std::ops::Div<MockAdcReading> for MockAdcReading
{
	type Output = Result<Percentage, ()>;

	fn div(self, rhs: MockAdcReading) -> Self::Output
	{
		Percentage::from_0_to_1(self.0 as f32 / rhs.0 as f32)
	}
}
