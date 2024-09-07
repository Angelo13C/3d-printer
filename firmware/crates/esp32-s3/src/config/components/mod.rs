mod adc;
mod flash;
mod pwm;
mod stepper_ticker_timer;
mod uart;

pub use adc::*;
use firmware_core::{
	printer::components::{
		config::{
			temperature::{PidConfig, SafetyConfig, ThermistorConfig},
			ComponentsConfig,
		},
		drivers::stepper_motor::tmc2209,
		motion::{self, RotationToLinearMotion},
		temperature::{safety::temperature_change::TemperatureChangeConfig, TemperaturePidGains},
	},
	utils::{
		math::{
			vectors::{Vector2, Vector3},
			Percentage,
		},
		measurement::{distance::Distance, temperature::Temperature},
	},
};
pub use flash::*;
pub use pwm::*;
pub use stepper_ticker_timer::*;
pub use uart::*;

pub fn configuration() -> ComponentsConfig
{
	ComponentsConfig {
		layer_fan_min_duty_cycle_to_move: Percentage::from_0_to_100(30.).unwrap(),
		hotend_fan_min_duty_cycle_to_move: Percentage::from_0_to_100(30.).unwrap(),
		hotend_pid: PidConfig {
			pid_gains: TemperaturePidGains {
				p: 100.,
				i: 10.,
				d: 750.,
			},
			thermistor: ThermistorConfig {
				beta: 3_950,
				resistance_at_t0: 100_000,
				other_resistance: 4_700,
			},
			safety: SafetyConfig {
				allowed_temperature_range: Temperature::from_celsius(0.)..=Temperature::from_celsius(260.),
				keep_target_temperature_config: TemperatureChangeConfig {
					period_in_seconds: 40.,
					hysteresis: 4.,
				},
				rise_to_target_temperature_config: TemperatureChangeConfig {
					period_in_seconds: 20.,
					hysteresis: 2.,
				},
				rise_to_target_temperature_samples_count: 20,
			},
		},
		heated_bed_pid: PidConfig {
			pid_gains: TemperaturePidGains {
				p: 1000.,
				i: 10.,
				d: 10.,
			},
			thermistor: ThermistorConfig {
				beta: 3_950,
				resistance_at_t0: 100_000,
				other_resistance: 4_700,
			},
			safety: SafetyConfig {
				allowed_temperature_range: Temperature::from_celsius(0.)..=Temperature::from_celsius(110.),
				keep_target_temperature_config: TemperatureChangeConfig {
					period_in_seconds: 20.,
					hysteresis: 2.,
				},
				rise_to_target_temperature_config: TemperatureChangeConfig {
					period_in_seconds: 90.,
					hysteresis: 2.,
				},
				rise_to_target_temperature_samples_count: 45,
			},
		},
		motion_controller: motion::CreationConfig {
			left_motor: motion::MotorConfig {
				tmc2209_address: tmc2209::UARTAddress::from_ms_pins_state(false, false),
				rotation_to_linear_motion: RotationToLinearMotion::new_connected_to_belt_driven(
					16,
					Distance::from_millimeters(2),
					200 * 256,
				),
			},
			right_motor: motion::MotorConfig {
				tmc2209_address: tmc2209::UARTAddress::from_ms_pins_state(false, true),
				rotation_to_linear_motion: RotationToLinearMotion::new_connected_to_belt_driven(
					16,
					Distance::from_millimeters(2),
					200 * 256,
				),
			},
			z_axis_motor: motion::MotorConfig {
				tmc2209_address: tmc2209::UARTAddress::from_ms_pins_state(true, false),
				rotation_to_linear_motion: RotationToLinearMotion::new_connected_to_lead_screw(
					4,
					Distance::from_millimeters(2),
					200 * 256,
				),
			},
			extruder_motor: motion::MotorConfig {
				tmc2209_address: tmc2209::UARTAddress::from_ms_pins_state(true, true),
				rotation_to_linear_motion: RotationToLinearMotion::new(
					Distance::from_micrometers((11. * 1_000. * 3.14) as i32),
					200 * 256,
				),
			},
			bed_size: Vector2::from_xy(Distance::from_millimeters(235), Distance::from_millimeters(235)),
			offset_from_nozzle_of_z_probe: Vector3::from_xyz(
				Distance::from_millimeters(115),
				Distance::from_millimeters(348),
				Distance::from_millimeters(-55),
			),
			planner_blocks_count: 512,
			planner_settings: motion::planner::Settings {
				min_feedrate_mm_s: 0.2,
				min_travel_feedrate_mm_s: 0.5,
				max_feedrate_mm_s: [200., 200., 10., 45.],
				retract_acceleration: 1_500.,
				print_acceleration: 1_500.,
				travel_acceleration: 2_000.,
				max_acceleration_mm_per_s2: [9000., 9000., 100., 10000.],
			},
		},
	}
}
