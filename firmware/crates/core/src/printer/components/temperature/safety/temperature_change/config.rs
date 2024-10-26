#[derive(Clone, Copy, Debug)]
/// Configuration parameters for temperature change safety.
///
/// This struct holds settings that dictate how temperature changes are managed
/// within the safety mechanisms.
///
/// # Examples
/// ```
/// # use firmware_core::printer::components::temperature::safety::temperature_change::*;
/// let config = TemperatureChangeConfig {
///     period_in_seconds: 15.0,
///     hysteresis: 2.0,
/// };
/// ```
pub struct TemperatureChangeConfig
{
	/// The duration within which the temperature should stabilize.
	pub period_in_seconds: f32,
	/// The allowable fluctuation range around the target temperature.
	pub hysteresis: f32,
}
