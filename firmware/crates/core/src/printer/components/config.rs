//! Configuration module for printer components.
//!
//! This module defines the configurations required for various components
//! of a 3D printer, including temperature control, motion settings, and fan
//! duty cycles.
//!
//! # Key struct
//! - [`ComponentsConfig`]: main configuration structure that holds the settings
//! for the printer's components, including PID settings for temperature control and
//! motion control parameters.

use super::motion;
use crate::utils::math::Percentage;

/// Holds configurations for different components.
pub struct ComponentsConfig
{
	/// Minimum duty cycle to activate the layer fan.
	pub layer_fan_min_duty_cycle_to_move: Percentage,

	/// Minimum duty cycle to activate the hotend fan.
	pub hotend_fan_min_duty_cycle_to_move: Percentage,

	/// PID settings for the hotend.
	pub hotend_pid: temperature::PidConfig,

	/// PID settings for the heated bed.
	pub heated_bed_pid: temperature::PidConfig,

	/// Configuration settings for the motion controller.
	pub motion_controller: motion::CreationConfig,
}

/// Temperature-related configurations.
///
/// This module includes structures to define PID settings for temperature
/// control, thermistor configurations, and safety parameters.
pub mod temperature
{
	use std::ops::RangeInclusive;

	use crate::{
		printer::components::temperature::{safety::temperature_change::TemperatureChangeConfig, TemperaturePidGains},
		utils::measurement::temperature::Temperature,
	};

	/// PID configuration for temperature control.
	///
	/// This struct holds the necessary parameters for PID control of temperatures,
	/// including the PID gains, thermistor configuration, and safety parameters.
	pub struct PidConfig
	{
		/// The PID gains for temperature control.
		pub pid_gains: TemperaturePidGains,

		/// Configuration settings for the thermistor.
		pub thermistor: ThermistorConfig,

		/// Safety configurations for temperature control.
		pub safety: SafetyConfig,
	}

	/// Configuration settings for the thermistor.
	///
	/// This struct defines the parameters necessary to configure a thermistor,
	/// including its beta value and resistance characteristics.
	pub struct ThermistorConfig
	{
		/// The beta value of the thermistor.
		pub beta: u32,

		/// The resistance at the reference temperature (T0).
		pub resistance_at_t0: u32,

		/// The resistance of the other resistor in the voltage divider.
		pub other_resistance: u32,
	}

	/// Safety configuration for temperature control.
	///
	/// This struct holds settings for ensuring safe temperature operations,
	/// including the allowed temperature range and configurations for temperature
	/// changes.
	pub struct SafetyConfig
	{
		/// The allowed temperature range for safe operation.
		pub allowed_temperature_range: RangeInclusive<Temperature>,

		/// Configuration for maintaining the target temperature.
		pub keep_target_temperature_config: TemperatureChangeConfig,

		/// Configuration for rising to the target temperature.
		pub rise_to_target_temperature_config: TemperatureChangeConfig,

		/// Number of samples to consider when rising to target temperature.
		pub rise_to_target_temperature_samples_count: usize,
	}
}
