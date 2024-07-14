use spin::{Mutex, MutexGuard};

use super::Flag;
use crate::{
	printer::components::motion::{
		kinematics::Kinematics,
		planner::{self, Block},
		N_MOTORS,
	},
	utils::measurement::distance::Distance,
};

static CURRENT_AND_NEXT_BLOCKS: Mutex<Communication> = Mutex::new(Communication {
	current_motion_profile_block: None,
	next_motion_profile_block: None,
	kinematics_functions: None,
	z_axis_distance: None,
});

/// Data shared between the "main thread" and the ticker ISR.
///
/// The main thread uses [`store_blocks_if_necessary`] to pass the data to the ISR.
/// The ISR reads it using [`get_blocks`].
pub struct Communication
{
	pub current_motion_profile_block: Option<planner::Block<N_MOTORS>>,
	next_motion_profile_block: Option<planner::Block<N_MOTORS>>,
	pub kinematics_functions: Option<KinematicsFunctions>,
	z_axis_distance: Option<Distance>,
}

impl Communication
{
	pub fn finish_using_current_block(&mut self)
	{
		self.current_motion_profile_block = self.next_motion_profile_block.take();
	}

	pub fn set_z_axis_distance(&mut self, distance: Distance)
	{
		self.z_axis_distance = Some(distance);
	}
pub fn get_z_axis_distance() -> Option<Distance>
{
	let mut communication = CURRENT_AND_NEXT_BLOCKS.lock();
	communication.z_axis_distance.take()
}

/// Checks if the ticker ISR requires 1 or 2 blocks and eventually uses them.
///
/// Check [`Communication`] for more info.
pub fn store_blocks_if_necessary(
	get_blocks: impl FnOnce(bool) -> (planner::Block<N_MOTORS>, Option<planner::Block<N_MOTORS>>),
)
{
	if let Some(mut communication) = CURRENT_AND_NEXT_BLOCKS.try_lock()
	{
		let current_block_required = communication.current_motion_profile_block.is_none();
		let next_block_required = communication.next_motion_profile_block.is_none();
		if current_block_required || next_block_required
		{
			let (first_block, second_block) = (get_blocks)(current_block_required && next_block_required);

			if current_block_required
			{
				communication.current_motion_profile_block = Some(first_block);
				communication.next_motion_profile_block = second_block;
			}
			else
			{
				communication.next_motion_profile_block = Some(first_block);
			}
		}
	}
}

/// Tries to lock the internal [`Mutex`] and returns `Some(Communication)` if it has been successful,
/// otherwise returns `None`.
///
/// Check [`Communication`] for more info.
pub fn get_blocks() -> Option<MutexGuard<'static, Communication>>
{
	CURRENT_AND_NEXT_BLOCKS.try_lock()
}

/// Returns `true` if there is either the current or the next block is `Some` in [`get_blocks`].
///
/// # Blocking
/// This function will lock a [`Mutex`] to read the current and next blocks.
pub fn is_block_available() -> bool
{
	let communication = CURRENT_AND_NEXT_BLOCKS.lock();

	communication.current_motion_profile_block.is_some() || communication.next_motion_profile_block.is_some()
}

/// Returns `true` if either the current or the next block have the [`Flag::Homing`] set.
///
/// # Blocking
/// This function will lock a [`Mutex`] to read the current and next blocks.
pub fn is_homing_block_available() -> bool
{
	let communication = CURRENT_AND_NEXT_BLOCKS.lock();

	fn check_block(block: Option<&Block<N_MOTORS>>) -> bool
	{
		block.map(|block| block.flags.contains(Flag::Homing)).unwrap_or(false)
	}

	check_block(communication.current_motion_profile_block.as_ref())
		|| check_block(communication.next_motion_profile_block.as_ref())
}

/// Sets the [`KinematicsFunctions`] used by the [`StepperMotorsTicker`] to understand how
/// the rotation of some motors relates to the movement of the tool.
///
/// [`StepperMotorsTicker`]: super::StepperMotorsTicker
pub fn set_kinematics_functions<K: Kinematics>()
{
	let mut communication = CURRENT_AND_NEXT_BLOCKS.lock();
	communication.kinematics_functions = Some(KinematicsFunctions {
		ab_to_x: K::ab_displacement_to_x,
		ab_to_y: K::ab_displacement_to_y,
	});
}

type KinematicFunction = fn(Distance, Distance) -> Distance;
#[derive(Clone, Copy)]
/// Function pointers used to convert the motion of the `a` and `b` stepper motors to movements
/// of the tool in the `X` and `Y` axes (check [`set_kinematics_functions`]).
pub struct KinematicsFunctions
{
	pub ab_to_x: KinematicFunction,
	pub ab_to_y: KinematicFunction,
}
