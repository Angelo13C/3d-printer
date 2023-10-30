pub mod endstop;

use enumset::enum_set;

use super::{
	axes::Axis,
	homing::endstop::Endstop,
	kinematics::Kinematics,
	planner::{communicate_to_ticker, BlocksBufferIsFull, Flag, Planner},
};
use crate::utils::{
	math::vectors::{Vector2, VectorN},
	measurement::distance::Distance,
};

#[derive(Clone, PartialEq, Eq)]
pub enum HomingProcedure
{
	None,
	Doing(HomingMove),
}

impl HomingProcedure
{
	/// Movement speed at which the printer is homed.
	const MOVE_SPEED_MM_MINUTE: f32 = 40. * 60.;

	/// Starts the homing procedure, by first homing the X axis.
	pub fn start_homing<const N: usize, K: Kinematics>(
		&mut self, planner: &mut Planner<N>, calculate_steps_per_mm: impl FnOnce() -> [f32; N],
	) -> Result<(), BlocksBufferIsFull>
	{
		*self = Self::Doing(HomingMove::X);

		Self::plan_move::<N, K>(HomingMove::X, planner, calculate_steps_per_mm)?;

		Ok(())
	}

	pub fn is_homing(&self) -> bool
	{
		*self != Self::None
	}

	pub fn tick<const N: usize, K: Kinematics, ZEndstop: Endstop>(
		&mut self, planner: &mut Planner<N>, calculate_steps_per_mm: impl FnOnce() -> [f32; N],
		z_endstop: &mut ZEndstop, bed_size: Vector2,
	) -> Result<(), TickError<ZEndstop>>
	{
		if let Self::Doing(homing_axis) = self.clone()
		{
			match homing_axis
			{
				HomingMove::X =>
				{
					if !Self::is_homing_move_being_executed(planner)
					{
						*self = Self::Doing(HomingMove::Y);

						Self::plan_move::<N, K>(HomingMove::Y, planner, calculate_steps_per_mm)
							.map_err(|_| TickError::HomingX)?;
					}
				},
				HomingMove::Y =>
				{
					if !Self::is_homing_move_being_executed(planner)
					{
						*self = Self::Doing(HomingMove::CenteringForZAxis { bed_size });

						// Move the carriage to the center of the bed
						Self::plan_move::<N, K>(
							HomingMove::CenteringForZAxis { bed_size },
							planner,
							calculate_steps_per_mm,
						)
						.map_err(|_| TickError::HomingY)?;
					}
				},
				HomingMove::CenteringForZAxis { bed_size: _ } =>
				{
					if !Self::is_homing_move_being_executed(planner)
					{
						*self = Self::Doing(HomingMove::Z);

						z_endstop
							.prepare_for_homing()
							.map_err(TickError::PreparingZAxisToProbe)?;

						Self::plan_move::<N, K>(HomingMove::Z, planner, calculate_steps_per_mm)
							.map_err(|_| TickError::HomingZ)?;
					}
				},
				HomingMove::Z =>
				{
					if !Self::is_homing_move_being_executed(planner)
					{
						*self = Self::None
					}
				},
			}
		}

		Ok(())
	}

	fn plan_move<const N: usize, K: Kinematics>(
		axis: HomingMove, planner: &mut Planner<N>, calculate_steps_per_mm: impl FnOnce() -> [f32; N],
	) -> Result<(), BlocksBufferIsFull>
	{
		planner.plan_move::<K>(
			axis.target_position(),
			(calculate_steps_per_mm)(),
			Self::MOVE_SPEED_MM_MINUTE,
		)?;
		planner.set_flags_on_last_added_block(enum_set!(Flag::Homing));
		planner.mark_last_added_move_as_ready_to_go();

		Ok(())
	}

	fn is_homing_move_being_executed<const N: usize>(planner: &mut Planner<N>) -> bool
	{
		planner.has_any_move_with_set_flags_planned(enum_set!(Flag::Homing))
			|| communicate_to_ticker::is_homing_block_available()
	}
}

#[derive(Clone, PartialEq, Eq)]
pub enum HomingMove
{
	X,
	Y,
	CenteringForZAxis
	{
		bed_size: Vector2,
	},
	Z,
}

impl HomingMove
{
	fn target_position<const N: usize>(&self) -> VectorN<N>
	{
		const HOMING_DISTANCE: Distance = Distance::from_centimeters(-100);
		let mut target_position = VectorN::ZERO;
		match self
		{
			HomingMove::X => target_position[Axis::X as usize] = HOMING_DISTANCE,
			HomingMove::Y => target_position[Axis::Y as usize] = HOMING_DISTANCE,
			HomingMove::CenteringForZAxis { bed_size } =>
			{
				target_position[Axis::X as usize] = bed_size.x() / 2;
				target_position[Axis::Y as usize] = bed_size.y() / 2;
			},
			HomingMove::Z => target_position[Axis::Z as usize] = HOMING_DISTANCE,
		}
		target_position
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TickError<ZEndstop: Endstop>
{
	HomingX,
	HomingY,
	HomingZ,
	PreparingZAxisToProbe(ZEndstop::HomingError),
}
