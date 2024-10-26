use enumset::EnumSet;
use pid_control::Controller;

use super::safety::{self, TemperatureSafety};
use crate::{
	printer::components::{
		drivers::{cartridge_heater::CartridgeHeater, thermistor::Thermistor},
		hal::{
			adc::{Adc, AdcPin, ReadPercentageError},
			pwm::PwmPin,
		},
	},
	utils::{
		math::{self, Percentage},
		measurement::temperature::Temperature,
	},
};

/// A [`PID controller`] used to control the temperature of a system in a closed loop.
///
/// To use it, first [`create`] the controller, than whenever you want you can [`choose the target temperature`]
/// and you must continually call [`tick`] to make the controller actually do the work.
///
/// [`PID controller`]: https://en.wikipedia.org/wiki/Proportional%E2%80%93integral%E2%80%93derivative_controller
/// [`create`]: `Self::new`
/// [`choose the target temperature`]: `Self::set_target_temperature`
/// [`tick`]: `Self::tick`
pub struct PidController<CHP: PwmPin, TADC: Adc, TP: AdcPin<TADC>>
{
	thermistor: Thermistor<TADC, TP>,
	cartridge_heater: CartridgeHeater<CHP>,
	pid_control: pid_control::PIDController,
	safety: TemperatureSafety,

	has_target_temperature: bool,
	last_current_temperature_sample: Option<Temperature>,
}

impl<CHP: PwmPin, TADC: Adc, TP: AdcPin<TADC>> PidController<CHP, TADC, TP>
{
	/// The minimum limit output by the PID control. Take this in consideration when setting the `PidGains`.
	pub const PID_CONTROL_MIN_LIMIT: f64 = 0.;
	/// The maximum limit output by the PID control. Take this in consideration when setting the `PidGains`.
	pub const PID_CONTROL_MAX_LIMIT: f64 = 100.;

	/// Returns a [`PidController`] that will control the `cartridge heater`'s current based on the [`set target temperature`]
	/// and the [`current temperature`] read in the provided `thermistor` using the provided gains.
	///
	/// [`set target temperature`]: `Self::set_target_temperature`
	/// [`current temperature`]: `Self::get_current_temperature`
	pub fn new(
		thermistor: Thermistor<TADC, TP>, cartridge_heater: CartridgeHeater<CHP>, pid_gains: PidGains,
		safety: TemperatureSafety,
	) -> Self
	{
		let mut pid_control =
			pid_control::PIDController::new(pid_gains.p as f64, pid_gains.i as f64, pid_gains.d as f64);
		pid_control.set_limits(Self::PID_CONTROL_MIN_LIMIT, Self::PID_CONTROL_MAX_LIMIT);

		Self {
			thermistor,
			cartridge_heater,
			pid_control,
			safety,
			has_target_temperature: false,
			last_current_temperature_sample: None,
		}
	}

	/// Returns the [`PidGains`] previously set on this PID controller.
	pub fn get_pid_gains(&self) -> PidGains
	{
		PidGains {
			p: self.pid_control.p_gain as f32,
			i: self.pid_control.i_gain as f32,
			d: self.pid_control.d_gain as f32,
		}
	}

	/// Set the PID gains of this controller. Check [`Self::PID_CONTROL_MIN_LIMIT`] and [`Self::PID_CONTROL_MAX_LIMIT`] to see in what
	/// range the values should be.
	pub fn set_pid_gains(&mut self, pid_gains: &PidGains)
	{
		self.pid_control.p_gain = pid_gains.p as f64;
		self.pid_control.i_gain = pid_gains.i as f64;
		self.pid_control.d_gain = pid_gains.d as f64;
	}

	/// Reads the current [`Temperature`] of the PID controller.
	///
	/// Returns `Ok(Temperature)` if the read was succesful, otherwise `Err(ReadPercentageError)`.
	pub fn get_current_temperature(&mut self, adc: &mut TADC) -> Result<Temperature, ReadPercentageError<TADC, TP>>
	{
		match self.thermistor.read_temperature(adc)
		{
			Ok(temperature) =>
			{
				self.last_current_temperature_sample = Some(temperature);
				Ok(temperature)
			},
			Err(error) => Err(error),
		}
	}

	/// Gets the [`Temperature`] read using [`Self::get_current_temperature`] the last time you called that function.
	///
	/// Returns `None` if [`Self::get_current_temperature`] has never been successfull since the instantation of this
	/// struct.
	pub fn get_last_sample_of_current_temperature(&self) -> Option<Temperature>
	{
		self.last_current_temperature_sample
	}

	/// Returns the [`Temperature`] the PID controller is trying to reach.
	pub fn get_target_temperature(&self) -> Option<Temperature>
	{
		self.has_target_temperature
			.then_some(Temperature::from_kelvin(self.pid_control.target() as f32))
	}

	/// Sets the [`Temperature`] the PID controller will try to reach.
	///
	/// # Warning
	/// You need to call [`PidController::tick`] after this to effectively make the PID controller work to reach it,
	/// even when setting `None` as the `target_tempearture`.
	pub fn set_target_temperature(&mut self, target_temperature: Option<Temperature>)
	{
		if let Some(target_temperature) = target_temperature
		{
			self.pid_control.set_target(target_temperature.as_kelvin() as f64);
			self.has_target_temperature = true;
		}
		else
		{
			self.pid_control.set_target(0.);
			self.pid_control.reset();
			self.has_target_temperature = false;
		}
	}

	/// Make the PID controller work to try to reach its [`target temperature`].
	///
	/// [`target temperature`]: `Self::get_target_temperature`
	pub fn tick(&mut self, delta_time: f64, adc: &mut TADC) -> Result<(), TickError>
	{
		let (current_temperature, should_stop_heating) = self.evaluate_safety(delta_time, adc)?;
		let pwm_percentage = match should_stop_heating
		{
			true => Percentage::ZERO,
			false =>
			{
				let pwm_value = self
					.pid_control
					.update(current_temperature.as_kelvin() as f64, delta_time);
				Percentage::from_0_to_1(math::map(
					pwm_value,
					Self::PID_CONTROL_MIN_LIMIT..=Self::PID_CONTROL_MAX_LIMIT,
					0_f64..=1_f64,
				) as f32)
				.unwrap()
			},
		};

		self.cartridge_heater
			.set_heat_percentage(pwm_percentage)
			.map_err(|_| TickError::SetCartridgeHeaterPercentage)
	}

	fn evaluate_safety(&mut self, delta_time: f64, adc: &mut TADC) -> Result<(Temperature, bool), TickError>
	{
		let current_temperature = self
			.get_current_temperature(adc)
			.map_err(|_| TickError::CantReadTemperature)?;

		let target_temperature = self.get_target_temperature();

		let safety_errors = self
			.safety
			.is_temperature_safe(current_temperature, target_temperature, delta_time as f32);
		if !safety_errors.is_empty()
		{
			self.cartridge_heater
				.set_heat_percentage(Percentage::ZERO)
				.map_err(|_| TickError::SetCartridgeHeaterPercentage)?;
			return Err(TickError::ReadTemperatureIsWrong(safety_errors));
		}

		Ok((current_temperature, target_temperature.is_none()))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// An error that occurred when calling [`tick`] on a PID controller.
///
/// [`tick`]: PidController::tick
pub enum TickError
{
	/// It has been impossible to [`read`] the thermistor's temperature.
	///
	/// [`read`]: `Thermistor::read_temperature`
	CantReadTemperature,

	/// The thermistor's `temperature` has been [`read`], but it's an irregular value.
	///
	/// **It could be that the thermistor is damaged, or its connection to the microcontroller is damaged...**
	/// It could also be a false positive: but it's always better to abort the print and turn off the heaters
	/// to prevent fire hazards. Then if it was a false positive, it means that the parameters passed
	/// to [`Safety::new`] are too strict.
	///
	/// [`read`]: `Thermistor::read_temperature`
	/// [`Safety::new`]: `safety::TemperatureSafety::new`
	ReadTemperatureIsWrong(EnumSet<safety::TemperatureError>),

	/// It has been impossible to [`set`] the cartridge heater's heat percentage.
	///
	/// [`set`]: `CartridgeHeater::set_heat_percentage`
	SetCartridgeHeaterPercentage,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Values of the `proportional`, `integral` and `derivative` gains of a PID controller.
pub struct PidGains
{
	/// [`Proportial component`](https://en.wikipedia.org/wiki/Proportional%E2%80%93integral%E2%80%93derivative_controller#Proportional).
	pub p: f32,
	/// [`Integral component`](https://en.wikipedia.org/wiki/Proportional%E2%80%93integral%E2%80%93derivative_controller#Integral).
	pub i: f32,
	/// [`Derivative component`](https://en.wikipedia.org/wiki/Proportional%E2%80%93integral%E2%80%93derivative_controller#Derivative).
	pub d: f32,
}
