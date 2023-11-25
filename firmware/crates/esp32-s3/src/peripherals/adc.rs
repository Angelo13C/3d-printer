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

	/// The readable value is the number of millivolts, because internally esp_idf_hal converts the ADC reading (which
	/// goes from 0 to 4095) to the range 0..3300 mV.
	fn max_readable_value(&self) -> Self::ReadableValue
	{
		AdcReading(crate::config::components::ADC_MAX_READABLE_MILLIVOLTS)
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
