use std::marker::PhantomData;

use esp_idf_hal::{
	adc::{Adc as EspAdc, AdcChannelDriver as EspAdcChannelDriver, AdcDriver as EspAdcDriver},
	gpio::ADCPin as EspAdcPin,
};
use esp_idf_sys::EspError;
use firmware_core::{
	printer::components::hal::adc::{Adc as AdcTrait, AdcPin as AdcPinTrait},
	utils::math::Percentage,
};

pub struct Adc<'d, ADC: EspAdc>(pub EspAdcDriver<'d, ADC>);
impl<'d, ADC: EspAdc> AdcTrait for Adc<'d, ADC>
{
	type ReadableValue = AdcReading;

	/// Returns [`AdcReading(4095)`](https://docs.espressif.com/projects/esp-idf/en/v4.4/esp32/api-reference/peripherals/adc.html#:~:text=Maximum%20of%20the%20output%20ADC,4095%20under%20Continuous%20Read%20mode).
	fn max_readable_value(&self) -> Self::ReadableValue
	{
		AdcReading(4095)
	}
}

pub struct AdcReading(u16);
impl std::ops::Div<AdcReading> for AdcReading
{
	type Output = Result<Percentage, ()>;

	fn div(self, rhs: AdcReading) -> Self::Output
	{
		Percentage::from_0_to_1(self.0 as f32 / rhs.0 as f32)
	}
}

pub struct AdcPin<'d, ADC: EspAdc, const ATTENUATION: esp_idf_hal::adc::attenuation::adc_atten_t, Pin: EspAdcPin>(
	EspAdcChannelDriver<'d, ATTENUATION, Pin>,
	PhantomData<ADC>,
);
impl<'d, ADC: EspAdc, const ATTENUATION: esp_idf_hal::adc::attenuation::adc_atten_t, Pin: EspAdcPin>
	AdcPin<'d, ADC, ATTENUATION, Pin>
{
	pub fn new(esp_driver: EspAdcChannelDriver<'d, ATTENUATION, Pin>) -> Self
	{
		Self(esp_driver, PhantomData)
	}
}
impl<'d, ADC: EspAdc, const ATTENUATION: esp_idf_hal::adc::attenuation::adc_atten_t, Pin: EspAdcPin<Adc = ADC>>
	AdcPinTrait<Adc<'d, ADC>> for AdcPin<'d, ADC, ATTENUATION, Pin>
{
	type Error = EspError;

	fn read(&mut self, adc: &mut Adc<'d, ADC>) -> Result<AdcReading, Self::Error>
	{
		Ok(AdcReading(adc.0.read(&mut self.0)?))
	}
}
