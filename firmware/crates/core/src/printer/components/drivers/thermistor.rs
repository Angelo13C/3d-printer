use std::marker::PhantomData;

use crate::{
	printer::components::hal::adc::{self, Adc, AdcPin, AdcPinExt},
	utils::{math::Percentage, measurement::temperature::Temperature},
};

/// The `25°C` temperature.
pub const T0: Temperature = Temperature::from_kelvin(25. + Temperature::ZERO_CELSIUS_IN_KELVIN);

/// A thermistor [`connected to the microcontroller using a voltage divider`].
///
/// [`connected to the microcontroller using a voltage divider`]: https://circuitdigest.com/microcontroller-projects/interfacing-Thermistor-with-arduino
pub struct Thermistor<A: Adc, P: AdcPin<A>>
{
	pin: P,
	_adc: PhantomData<A>,
	other_resistance: u32,
	beta: u32,
	resistance_at_t0: u32,
}

impl<A: Adc, P: AdcPin<A>> Thermistor<A, P>
{
	/// Returns a [`Thermistor`] that is connected to the microcontroller through the provided `pin` in a voltage divider setup,
	/// that has the provided `beta` constant and has a `resistance_at_t0` resistance at [`T0`].
	/// The other resistor in the voltage divider is of `other_resistance` Ω.
	pub fn new(pin: P, beta: u32, resistance_at_t0: u32, other_resistance: u32) -> Self
	{
		Self {
			beta,
			resistance_at_t0,
			other_resistance,
			pin,
			_adc: PhantomData,
		}
	}

	/// Reads the current [`Temperature`] from the thermistor.
	///
	/// Returns `Ok(Temperature)` if the read was successfull, otherwise `Err(ReadPercentageError)`.
	pub fn read_temperature(&mut self, adc: &mut A) -> Result<Temperature, adc::ReadPercentageError<A, P>>
	{
		let adc_sample = self.pin.read_percentage(adc)?;
		Ok(self.convert_adc_sample_to_temperature(adc_sample))
	}

	/// Converts an ADC sample from the [`Self::pin`] to a [`Temperature`] measurement.
	///
	/// Thanks to: https://dev.to/apollolabsbin/esp32-embedded-rust-at-the-hal-analog-temperature-sensing-using-the-adc-3106
	fn convert_adc_sample_to_temperature(&self, adc_sample: Percentage) -> Temperature
	{
		// Avoid division by zero below in the calculation of the resistance from the voltage divider
		let mut adc_sample_percentage = adc_sample.into_0_to_1();
		if adc_sample_percentage == 1.
		{
			adc_sample_percentage = 0.999;
		}

		// Formula to calculate resistance from voltage divider
		let current_resistance = self.other_resistance as f32 * (adc_sample_percentage / (1. - adc_sample_percentage));

		let beta = self.beta as f64;
		let resistance_at_t0 = self.resistance_at_t0 as f64;
		let temperature =
			1. / (f64::ln(current_resistance as f64 / resistance_at_t0) / beta + (1. / T0.as_celsius() as f64));

		Temperature::from_kelvin(temperature as f32)
	}
}
