//! Module for managing the homing procedure of the printer, which ensures that the tool is positioned correctly
//! before starting any printing operations.

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

/// Represents the current state of the homing procedure.
#[derive(Clone, PartialEq, Eq)]
pub enum HomingProcedure
{
	/// No homing procedure is currently active.
	None,
	/// Indicates that the homing procedure should start.
	ShouldStart,
	/// Indicates that a homing move is currently being executed.
	Doing(HomingMove),
}

impl HomingProcedure
{
	/// Movement speed at which the printer is homed, in millimeters per second.
	const MOVE_SPEED_MM_SECOND: f32 = 40.;

	/// Starts the homing procedure, beginning with the X axis.
	///
	/// # Parameters
	/// - `planner`: A mutable reference to the planner that manages motion planning.
	/// - `calculate_steps_per_mm`: A function that calculates the number of steps required per millimeter for each axis.
	///
	/// # Returns
	/// Returns `Ok(())` if successful, or an error if the blocks buffer is full.
	pub fn start_homing<const N: usize, K: Kinematics>(
		&mut self, planner: &mut Planner<N>, calculate_steps_per_mm: impl FnOnce() -> [f32; N],
	) -> Result<(), BlocksBufferIsFull>
	{
		*self = Self::ShouldStart;

		Self::plan_move::<N, K>(HomingMove::X, planner, calculate_steps_per_mm)?;

		*self = Self::Doing(HomingMove::X);

		Ok(())
	}

	/// Checks if a homing procedure is currently active.
	///
	/// # Returns
	/// Returns `true` if homing is in progress, otherwise `false`.
	pub fn is_homing(&self) -> bool
	{
		*self != Self::None
	}

	/// Executes the tick function for the homing procedure, progressing through the different axes.
	///
	/// # Parameters
	/// - `planner`: A mutable reference to the planner that manages motion planning.
	/// - `calculate_steps_per_mm`: A function that calculates the number of steps required per millimeter for each axis.
	/// - `z_endstop`: A mutable reference to the Z-axis endstop.
	/// - `bed_size`: A `Vector2` representing the size of the bed.
	///
	/// # Returns
	/// Returns `Ok(())` if successful, or an error if an issue occurs during the tick.
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
						Self::plan_move::<N, K>(HomingMove::Y, planner, calculate_steps_per_mm)
							.map_err(|_| TickError::HomingX)?;

						*self = Self::Doing(HomingMove::Y);
					}
				},
				HomingMove::Y =>
				{
					if !Self::is_homing_move_being_executed(planner)
					{
						// Move the carriage to the center of the bed
						Self::plan_move::<N, K>(
							HomingMove::CenteringForZAxis { bed_size },
							planner,
							calculate_steps_per_mm,
						)
						.map_err(|_| TickError::HomingY)?;

						*self = Self::Doing(HomingMove::CenteringForZAxis { bed_size });
					}
				},
				HomingMove::CenteringForZAxis { bed_size: _ } =>
				{
					if !Self::is_homing_move_being_executed(planner)
					{
						z_endstop
							.prepare_for_homing()
							.map_err(TickError::PreparingZAxisToProbe)?;

						Self::plan_move::<N, K>(HomingMove::Z, planner, calculate_steps_per_mm)
							.map_err(|_| TickError::HomingZ)?;

						*self = Self::Doing(HomingMove::Z);
					}
				},
				HomingMove::Z =>
				{
					if !Self::is_homing_move_being_executed(planner)
					{
						*self = Self::None;
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
			Self::MOVE_SPEED_MM_SECOND,
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

/// Represents the different stages of the homing process.
#[derive(Clone, PartialEq, Eq)]
pub enum HomingMove
{
	/// Homing move for the X axis.
	X,
	/// Homing move for the Y axis.
	Y,
	/// Centering move for the Z axis.
	CenteringForZAxis
	{
		bed_size: Vector2
	},
	/// Homing move for the Z axis.
	Z,
}

impl HomingMove
{
	/// Gets the target position for the specified homing move.
	///
	/// # Parameters
	/// - `N`: The number of dimensions in the motion space.
	///
	/// # Returns
	/// A `VectorN<N>` representing the target position for the homing move.
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

/// Errors that can occur during the homing tick process.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TickError<ZEndstop: Endstop>
{
	/// Error during the X axis homing move.
	HomingX,
	/// Error during the Y axis homing move.
	HomingY,
	/// Error during the Z axis homing move.
	HomingZ,
	/// Error while preparing the Z axis for probing.
	PreparingZAxisToProbe(ZEndstop::HomingError),
}

impl<ZEndstop: Endstop> std::fmt::Debug for TickError<ZEndstop>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Self::HomingX => write!(f, "HomingX"),
			Self::HomingY => write!(f, "HomingY"),
			Self::HomingZ => write!(f, "HomingZ"),
			Self::PreparingZAxisToProbe(arg0) => f.debug_tuple("PreparingZAxisToProbe").field(arg0).finish(),
		}
	}
}
