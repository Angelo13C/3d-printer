use core::time::Duration;

use esp_idf_hal::{
	gpio::OutputPin,
	ledc::SpeedMode,
	peripheral::Peripheral,
	rmt::{
		config::{Loop, TransmitConfig},
		FixedLengthSignal, PinState, Pulse, RmtChannel, TxRmtDriver,
	},
};
use esp_idf_sys::{esp, ledc_set_freq, EspError};
use firmware_core::{
	printer::components::hal::pwm::PwmPin as PwmPinTrait,
	utils::{
		math::Percentage,
		measurement::{duration::SmallDuration, frequency::Frequency},
	},
};

pub struct LedcPwmPin<'d>(pub esp_idf_hal::ledc::LedcDriver<'d>);
impl<'d> PwmPinTrait for LedcPwmPin<'d>
{
	type Error = EspError;

	fn get_duty_cycle(&self) -> Percentage
	{
		Percentage::from_0_to_1(self.0.get_duty() as f32 / self.0.get_max_duty() as f32).unwrap_or(Percentage::FULL)
	}

	fn set_duty_cycle(&mut self, percentage: Percentage) -> Result<(), Self::Error>
	{
		self.0
			.set_duty((percentage.into_0_to_1() * self.0.get_max_duty() as f32) as u32)
	}

	fn set_frequency(&mut self, frequency: Frequency) -> Result<(), Self::Error>
	{
		esp!(unsafe { ledc_set_freq(SpeedMode::LowSpeed.into(), self.0.timer(), frequency.as_hertz()) })?;
		Ok(())
	}
}

pub struct RmtPwmPin<'d>
{
	tx_rmt_driver: TxRmtDriver<'d>,
	current_duty_cycle: Percentage,
	current_frequency: Frequency,
}

impl<'d> RmtPwmPin<'d>
{
	/// RMT source clock's frequency [`80MHz`].
	///
	/// [`80MHz`]: https://docs.espressif.com/projects/esp-idf/en/v4.4/esp32/api-reference/peripherals/rmt.html#:~:text=The%20RMT%20source%20clock%20is,changed%20to%20REF_TICK%20or%20XTAL.
	const CLOCK_FREQUENCY: Frequency = Frequency::from_hertz(80_000_000);

	const MAX_PWM_FREQUENCY: Frequency = Frequency::from_hertz(500_000);

	const DEFAULT_FREQUENCY: Frequency = Frequency::from_hertz(5_000);
	const DEFAULT_DUTY_CYCLE: Percentage = Percentage::ZERO;

	pub fn new<C: RmtChannel>(
		channel: impl Peripheral<P = C> + 'd, pin: impl Peripheral<P = impl OutputPin> + 'd,
	) -> Result<Self, EspError>
	{
		let tx_rmt_driver = TxRmtDriver::new(
			channel,
			pin,
			&TransmitConfig {
				clock_divider: (Self::CLOCK_FREQUENCY.as_hertz() / Self::MAX_PWM_FREQUENCY.as_hertz() * 2) as u8,
				mem_block_num: 1,
				carrier: None,
				looping: Loop::Endless,
				idle: Some(PinState::Low),
				aware_dfs: false,
				..Default::default()
			},
		)?;

		Ok(Self {
			tx_rmt_driver,
			current_duty_cycle: Self::DEFAULT_DUTY_CYCLE,
			current_frequency: Self::DEFAULT_FREQUENCY,
		})
	}

	fn create_signal(
		&self, low_value_duration: SmallDuration, high_value_duration: SmallDuration,
	) -> Result<FixedLengthSignal<1>, EspError>
	{
		let counter_clock = self.tx_rmt_driver.counter_clock()?;
		let mut signal = FixedLengthSignal::new();
		signal.set(
			0,
			&(
				Pulse::new_with_duration(
					counter_clock,
					PinState::Low,
					&Duration::from_nanos(low_value_duration.as_nanos()),
				)?,
				Pulse::new_with_duration(
					counter_clock,
					PinState::High,
					&Duration::from_nanos(high_value_duration.as_nanos()),
				)?,
			),
		)?;

		Ok(signal)
	}

	fn apply_pwm_changes(&mut self) -> Result<(), <Self as PwmPinTrait>::Error>
	{
		let cycle_duration = Into::<SmallDuration>::into(self.current_frequency);
		let high_value_duration = cycle_duration * self.current_duty_cycle.into_0_to_1();
		let signal = self.create_signal(cycle_duration - high_value_duration, high_value_duration)?;
		self.tx_rmt_driver.start(signal)?;

		Ok(())
	}
}

impl<'d> PwmPinTrait for RmtPwmPin<'d>
{
	type Error = EspError;

	fn get_duty_cycle(&self) -> Percentage
	{
		self.current_duty_cycle
	}

	fn set_duty_cycle(&mut self, percentage: Percentage) -> Result<(), Self::Error>
	{
		self.current_duty_cycle = percentage;

		self.apply_pwm_changes()
	}

	fn set_frequency(&mut self, frequency: Frequency) -> Result<(), Self::Error>
	{
		self.current_frequency = frequency;

		self.apply_pwm_changes()
	}
}
