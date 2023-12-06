use esp_idf_hal::{
	ledc::{config::TimerConfig, Resolution, SpeedMode},
	units::Hertz,
};
use firmware_core::printer::components::drivers::servo_motor;

pub const FANS_PWM_TIMER: TimerConfig = TimerConfig {
	frequency: Hertz(500),
	resolution: Resolution::Bits12,
	speed_mode: SpeedMode::LowSpeed,
};
pub const BED_HEATER_PWM_TIMER: TimerConfig = TimerConfig {
	frequency: Hertz(1_000),
	resolution: Resolution::Bits11,
	speed_mode: SpeedMode::LowSpeed,
};
pub const HOTEND_HEATER_PWM_TIMER: TimerConfig = TimerConfig {
	frequency: Hertz(1_000),
	resolution: Resolution::Bits11,
	speed_mode: SpeedMode::LowSpeed,
};
pub const BL_TOUCH_SIGNAL_PWM_TIMER: TimerConfig = TimerConfig {
	frequency: Hertz(servo_motor::REFRESH_RATE.as_hertz()),
	resolution: Resolution::Bits12,
	speed_mode: SpeedMode::LowSpeed,
};
