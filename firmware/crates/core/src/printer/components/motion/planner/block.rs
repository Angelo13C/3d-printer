use enumset::EnumSet;

/// Result of a planned move. It contains all the data necessary to allow the [`StepperMotorsTicker`] to make
/// the move actually happen.
///
/// [`StepperMotorsTicker`]: super::StepperMotorsTicker
pub struct Block<const N: usize>
{
	// Fields used by the bresenham algorithm for tracing the line
	pub steps: [i32; N],       // Step count along each axis
	pub step_event_count: u32, // The maximum step axis count and number of steps required to complete this block.

	// Fields used by the motion planner to manage acceleration
	pub entry_speed_sqr: f32,     // The current planned entry speed at block junction in (mm/min)^2
	pub max_entry_speed_sqr: f32, // Maximum allowable entry speed based on the minimum of junction limit and

	pub acceleration: f32, // Axis-limit adjusted line acceleration in (mm/min^2)
	pub acceleration_steps_per_sec2: f32,
	pub acceleration_rate: u32,
	pub millimeters: f32, // The remaining distance for this block to be executed in (mm)

	pub accelerate_until: u32,
	pub decelerate_after: u32,
	pub initial_speed: f32,
	pub nominal_speed: f32,               // mm/sec
	pub nominal_speed_steps_per_sec: f32, // step/sec
	pub final_speed: f32,

	pub flags: EnumSet<Flag>,
}

impl<const N: usize> Default for Block<N>
{
	fn default() -> Self
	{
		Self {
			steps: [0; N],
			step_event_count: Default::default(),
			entry_speed_sqr: Default::default(),
			max_entry_speed_sqr: Default::default(),

			nominal_speed: Default::default(),
			nominal_speed_steps_per_sec: Default::default(),

			acceleration: 1_000_000.,
			acceleration_steps_per_sec2: Default::default(),
			acceleration_rate: Default::default(),
			millimeters: Default::default(),

			accelerate_until: Default::default(),
			decelerate_after: Default::default(),
			initial_speed: Default::default(),
			final_speed: Default::default(),

			flags: EnumSet::EMPTY,
		}
	}
}

#[derive(enumset::EnumSetType)]
pub enum Flag
{
	/// An homing procedure move.
	Homing,

	/// A bed leveling move.
	BedLeveling,

	/// Recalculate trapezoids on entry junction (for optimization).
	Recalculate,

	/// Nominal speed always reached.
	/// i.e., The segment is long enough, so the nominal speed is reachable if accelerating
	/// from a safe speed (in consideration of jerking from zero speed).
	NominalLength,
}
