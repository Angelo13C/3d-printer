#[derive(Clone, Copy, Debug)]
pub struct TemperatureChangeConfig
{
	pub period_in_seconds: f32,
	pub hysteresis: f32,
}
