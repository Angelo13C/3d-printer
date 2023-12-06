use super::motion;
use crate::utils::math::Percentage;

pub struct ComponentsConfig
{
	pub layer_fan_min_duty_cycle_to_move: Percentage,
	pub hotend_fan_min_duty_cycle_to_move: Percentage,

	pub hotend_pid: temperature::PidConfig,
	pub heated_bed_pid: temperature::PidConfig,

	pub motion_controller: motion::CreationConfig,
}

pub mod temperature
{
	use std::ops::RangeInclusive;

	use crate::{
		printer::components::temperature::{safety::temperature_change::TemperatureChangeConfig, TemperaturePidGains},
		utils::measurement::temperature::Temperature,
	};

	pub struct PidConfig
	{
		pub pid_gains: TemperaturePidGains,
		pub thermistor: ThermistorConfig,
		pub safety: SafetyConfig,
	}

	pub struct ThermistorConfig
	{
		pub beta: u32,
		pub resistance_at_t0: u32,
		pub other_resistance: u32,
	}

	pub struct SafetyConfig
	{
		pub allowed_temperature_range: RangeInclusive<Temperature>,
		pub keep_target_temperature_config: TemperatureChangeConfig,
		pub rise_to_target_temperature_config: TemperatureChangeConfig,
		pub rise_to_target_temperature_samples_count: usize,
	}
}
