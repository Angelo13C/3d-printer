mod unified;
mod z_axis_probe;

use enumset::enum_set;
pub use unified::*;
pub use z_axis_probe::*;

use super::{
	axes::Axis,
	kinematics::Kinematics,
	planner::{communicate_to_ticker, BlocksBufferIsFull, Flag, Planner},
};
use crate::utils::{
	math::vectors::{Vector2, VectorN},
	measurement::distance::Distance,
};

pub struct BedLevelingProcedure
{
	unified_bed_leveling: UnifiedBedLevelingProcedure,
	bed_size: Vector2,
	/// The whole purpose of this variable is to solve the problem of the block buffer being full in the middle of the
	/// bed leveling procedure. With this variable it can re-start from the point where it left off.
	current_point_planned_index: u16,
	current_point_index: u16,
}

impl BedLevelingProcedure
{
	/// Movement speed at which the printer is probed.
	const MOVE_SPEED_MM_SECOND: f32 = 40.;
	/// Movement speed of probing at which the printer is probed.
	const PROBE_MOVE_SPEED_MM_SECOND: f32 = 10.;
	/// Distance of the z axis probe from the bed (z axis) for each probed point.
	pub const DISTANCE_FROM_BED: Distance = Distance::from_centimeters(3);
	/// Distance of the z axis probe from the bed margins (x and y axes).
	const DISTANCE_FROM_BED_MARGINS: Distance = Distance::from_centimeters(3);
	/// Amount of grid points for the bed leveling procedure (on the x and the y axes).
	const BED_LEVELING_GRID_SIZE: (u8, u8) = (5, 5);

	pub fn new(bed_size: Vector2) -> Self
	{
		Self {
			unified_bed_leveling: UnifiedBedLevelingProcedure::new(),
			bed_size,
			current_point_planned_index: 0,
			current_point_index: 0,
		}
	}

	pub fn start<const N: usize, K: Kinematics>(
		&mut self, planner: &mut Planner<N>, steps_per_mm: [f32; N],
	) -> Result<(), BlocksBufferIsFull>
	{
		let mut start_y = 0;
		let mut start_x = 0;

		// Restart from where the procedure stopped due to the block buffer being full
		if self.current_point_planned_index != 0
		{
			start_y = ((self.current_point_planned_index / 2) / Self::BED_LEVELING_GRID_SIZE.0 as u16) as u8;
			start_x = ((self.current_point_planned_index / 2) % Self::BED_LEVELING_GRID_SIZE.0 as u16) as u8;
		}
		// It's a new start
		else
		{
			self.current_point_index = 0;

			self.unified_bed_leveling
				.start(self.bed_size, Self::BED_LEVELING_GRID_SIZE, Distance::ZERO);
		}

		let distance_between_points = (self.bed_size - &Vector2::new([Self::DISTANCE_FROM_BED_MARGINS * 2; 2]));
		let distance_between_points = Vector2::from_xy(
			distance_between_points.x() / Self::BED_LEVELING_GRID_SIZE.0,
			distance_between_points.y() / Self::BED_LEVELING_GRID_SIZE.1,
		);

		for y in start_y..Self::BED_LEVELING_GRID_SIZE.1
		{
			for x in start_x..Self::BED_LEVELING_GRID_SIZE.0
			{
				let mut target_position_array = planner.get_position().get_internal_array().clone();

				target_position_array[Axis::X as usize] =
					Self::DISTANCE_FROM_BED_MARGINS + distance_between_points.x() * x;
				target_position_array[Axis::Y as usize] =
					Self::DISTANCE_FROM_BED_MARGINS + distance_between_points.y() * y;

				if self.current_point_planned_index % 2 == 0
				{
					target_position_array[Axis::Z as usize] = Self::DISTANCE_FROM_BED;
					let target_position = VectorN::new(target_position_array);
					planner.plan_move::<K>(target_position, steps_per_mm, Self::MOVE_SPEED_MM_SECOND)?;
					planner.mark_last_added_move_as_ready_to_go();
					self.current_point_planned_index += 1;
				}

				target_position_array[Axis::Z as usize] = Distance::from_centimeters(-50);
				let target_position = VectorN::new(target_position_array);
				planner.plan_move::<K>(target_position, steps_per_mm, Self::PROBE_MOVE_SPEED_MM_SECOND)?;
				planner.set_flags_on_last_added_block(enum_set!(Flag::BedLeveling));
				planner.mark_last_added_move_as_ready_to_go();
				self.current_point_planned_index += 1;
			}

			start_x = 0;
		}

		self.current_point_planned_index = 0;

		Ok(())
	}

	pub fn tick(&mut self)
	{
		if let Some(probing_distance) = communicate_to_ticker::get_z_axis_distance()
		{
			let point_correction = probing_distance - Self::DISTANCE_FROM_BED;
			self.unified_bed_leveling
				.set_point_correction(self.current_point_index, point_correction);

			self.current_point_index += 1;

			if self.current_point_index == Self::BED_LEVELING_GRID_SIZE.0 as u16 * Self::BED_LEVELING_GRID_SIZE.1 as u16
			{
				self.unified_bed_leveling.finish_procedure().unwrap();
			}
		}
	}

	pub fn apply<const N: usize>(&mut self, target_position: &mut VectorN<N>)
	{
		self.unified_bed_leveling.apply(target_position)
	}
}
