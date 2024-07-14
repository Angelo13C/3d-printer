use crate::{
	printer::components::motion::{axes::Axis, kinematics::Kinematics},
	utils::{
		math::{
			self,
			vectors::{Vector2, VectorN},
		},
		measurement::distance::Distance,
	},
};

#[derive(Default)]
pub enum UnifiedBedLevelingProcedure
{
	#[default]
	None,
	Working(UnifiedBedLeveling),
	Done(UnifiedBedLeveling),
}

impl UnifiedBedLevelingProcedure
{
	pub fn new() -> Self
	{
		Self::default()
	}

	pub fn has_started(&self) -> bool
	{
		match self
		{
			UnifiedBedLevelingProcedure::Working(_) => true,
			_ => false,
		}
	}

	pub fn start(&mut self, bed_size: Vector2, grid_size: (u8, u8), default_value: Distance)
	{
		*self = Self::Working(UnifiedBedLeveling::new(bed_size, grid_size, default_value));
	}

	pub fn set_point_correction(&mut self, index: u16, point_correction: Distance) -> Result<(), ()>
	{
		let function = |inner: &mut UnifiedBedLeveling| {
			inner.points_correction[index as usize] = point_correction;
		};
		match self
		{
			UnifiedBedLevelingProcedure::None => return Err(()),
			UnifiedBedLevelingProcedure::Working(inner) => (function)(inner),
			UnifiedBedLevelingProcedure::Done(inner) => (function)(inner),
		}

		Ok(())
	}

	pub fn finish_procedure(&mut self) -> Result<(), ()>
	{
		match self
		{
			UnifiedBedLevelingProcedure::Working(inner) =>
			{
				*self = Self::Done(std::mem::replace(inner, UnifiedBedLeveling::empty()))
			},
			_ => return Err(()),
		}

		Ok(())
	}

	pub fn apply<const N: usize>(&mut self, target_position: &mut VectorN<N>)
	{
		assert!(N >= 3);

		if let UnifiedBedLevelingProcedure::Done(inner) = self
		{
			let x = target_position[Axis::X as usize];
			let y = target_position[Axis::Y as usize];

			let point_correction = inner.get_point_correction(Vector2::from_xy(x, y));
			target_position[Axis::Z as usize] += point_correction;
		}
	}
}

/// # Grid
/// Points are stored starting from the bottom left, going first on the right and then when a rows ends going up.
/// Here there is an example for a grid of size 4x5:
/// ```text
///              19
///              ↓
///        · · · ·
///        · · · ·
///        · · · ·
///    4 → · · · ·
///    0 → · · · ·
///        ↑     ↑
///        0     3
/// ```
pub struct UnifiedBedLeveling
{
	points_correction: Vec<Distance>,
	number_of_points_in_x: u8,
	bed_size: Vector2,
}

impl UnifiedBedLeveling
{
	/// # Panics
	/// Panics if `grid_size.0` or `grid_size.1` are less than 2.
	fn new(bed_size: Vector2, grid_size: (u8, u8), default_point_correction: Distance) -> Self
	{
		assert!(grid_size.0 > 1 && grid_size.1 > 1);

		Self {
			points_correction: vec![default_point_correction; grid_size.0 as usize * grid_size.1 as usize],
			number_of_points_in_x: grid_size.0,
			bed_size,
		}
	}

	fn empty() -> Self
	{
		Self {
			points_correction: Vec::new(),
			number_of_points_in_x: 0,
			bed_size: Vector2::ZERO,
		}
	}

	/// It uses bilinear interpolation.
	fn get_point_correction(&self, point: Vector2) -> Distance
	{
		fn axis(point: Distance, bed_size_in_axis: Distance, number_of_points_in_axis: u8) -> f32
		{
			let point_in_bed = point / bed_size_in_axis;
			point_in_bed * number_of_points_in_axis as f32
		}
		let x = axis(point.x(), self.bed_size.x(), self.number_of_points_in_x);
		let y = axis(point.y(), self.bed_size.y(), self.number_of_points_in_y());

		let interpolate_edge = |corner_xy: (u8, u8)| {
			let edge_points_offsets = (
				self.points_correction[self.xy_to_index(corner_xy.0, corner_xy.1)],
				self.points_correction[self.xy_to_index(corner_xy.0, corner_xy.1 + 1)],
			);

			math::lerp(y.fract(), edge_points_offsets.0..=edge_points_offsets.1)
		};

		let left_edge_correction = (interpolate_edge)((x.trunc() as u8, y.trunc() as u8));
		let right_edge_correction = (interpolate_edge)((1 + x.trunc() as u8, y.trunc() as u8));
		math::lerp(x.fract(), left_edge_correction..=right_edge_correction)
	}

	pub fn index_to_xy(&self, index: u8) -> (u8, u8)
	{
		let x = index / self.number_of_points_in_x;
		(x, index - (x * self.number_of_points_in_x))
	}

	fn xy_to_index(&self, x: u8, y: u8) -> usize
	{
		self.number_of_points_in_x as usize * y as usize + x as usize
	}

	fn number_of_points_in_y(&self) -> u8
	{
		(self.points_correction.len() / self.number_of_points_in_x as usize) as u8
	}
}
