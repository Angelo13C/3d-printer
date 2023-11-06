use esp_idf_hal::timer::config::Config;
use firmware_core::utils::measurement::frequency::Frequency;

const STEPPER_TIMER_MAX_FREQUENCY: Frequency = Frequency::from_hertz(10_000_000);

pub const STEPPER_TIMER_CONFIG: Config = Config {
	divider: stepper_timer_divider(),
	xtal: false,
	auto_reload: false,
};

const fn stepper_timer_divider() -> u32
{
	ceil_div(
		crate::peripherals::timer::BASE_CLOCK_FREQUENCY.as_hertz(),
		STEPPER_TIMER_MAX_FREQUENCY.as_hertz(),
	)
}

const fn ceil_div(a: u32, b: u32) -> u32
{
	(a + b - 1) / b
}
