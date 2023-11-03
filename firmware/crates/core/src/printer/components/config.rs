use embedded_hal::digital::OutputPin;

use super::{
	hal::timer::Timer as TimerTrait,
	motion::{self, bed_leveling::ZAxisProbe, homing::endstop::Endstop, kinematics::Kinematics as KinematicsTrait},
};
use crate::utils::math::Percentage;

pub struct ComponentsConfig<
	Timer: TimerTrait,
	Kinematics: KinematicsTrait,
	LeftDirPin: OutputPin + 'static,
	LeftStepPin: OutputPin + 'static,
	RightDirPin: OutputPin + 'static,
	RightStepPin: OutputPin + 'static,
	ZAxisDirPin: OutputPin + 'static,
	ZAxisStepPin: OutputPin + 'static,
	ExtruderDirPin: OutputPin + 'static,
	ExtruderStepPin: OutputPin + 'static,
	XEndstop: Endstop + 'static,
	YEndstop: Endstop + 'static,
	ZEndstop: ZAxisProbe,
> {
	pub layer_fan_min_duty_cycle_to_move: Percentage,
	pub hotend_fan_min_duty_cycle_to_move: Percentage,

	pub hotend_pid: temperature::PidConfig,
	pub heated_bed_pid: temperature::PidConfig,

	pub motion_controller: motion::CreationParameters<
		Timer,
		Kinematics,
		LeftDirPin,
		LeftStepPin,
		RightDirPin,
		RightStepPin,
		ZAxisDirPin,
		ZAxisStepPin,
		ExtruderDirPin,
		ExtruderStepPin,
		XEndstop,
		YEndstop,
		ZEndstop,
	>,
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
