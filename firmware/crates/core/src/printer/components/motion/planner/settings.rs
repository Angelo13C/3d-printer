pub struct Settings<const N: usize>
{
	pub min_feedrate_mm_s: f32,
	pub min_travel_feedrate_mm_s: f32,
	pub max_feedrate_mm_s: [f32; N],

	pub retract_acceleration: f32,
	pub print_acceleration: f32,
	pub travel_acceleration: f32,

	pub max_acceleration_mm_per_s2: [f32; N],
}
