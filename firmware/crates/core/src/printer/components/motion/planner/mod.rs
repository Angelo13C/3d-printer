//! Translates movements (for example a `10mm` move in the `X` axis) to motion profiles "understood" by [`stepper motors`].
//!
//! The generated motion profiles are optimized to make the move as fast as possible without provoking problems
//! (for example making a stepper lose steps).
//!
//! [`stepper motors`]: `super::super::drivers::stepper_motor::StepperMotor`

use enumset::EnumSet;
use ringbuffer::{AllocRingBuffer, RingBuffer};

use super::{axes::Axis, kinematics::Kinematics, ticker::StepperMotorsTicker, N_MOTORS};
use crate::{
	printer::components::hal::timer::Timer,
	utils::{
		math::{vectors::VectorN, NumberExt},
		measurement::{distance::Distance, frequency::Frequency},
	},
};

mod block;
pub mod communicate_to_ticker;
mod settings;

pub use block::*;
pub use settings::*;

const MIN_STEP_RATE: f32 = 120.;

// Minimum planner junction speed. Sets the default minimum speed the planner plans for at the end
// of the buffer and all stops. This should not be much greater than zero and should only be changed
// if unwanted behavior is observed on a user's machine when running at very slow speeds.
const MINIMUM_PLANNER_SPEED: f32 = 0.05; // (mm/s)

const JUNCTION_DEVIATION_MM: f32 = 0.02;

/// The planning involves calculating the most optimal speed and acceleration profiles
pub struct Planner<const N: usize>
{
	current_position: VectorN<N>,

	current_move_id: MoveId,
	last_move_id: MoveId,

	previous_normalized_displacement: Option<VectorN<N>>,
	previous_nominal_speed: f32,

	blocks: AllocRingBuffer<Block<N>>,
	ready_to_go_blocks_count: usize,
	settings: Settings<N>,

	most_optimized_block_index: usize,

	stepper_ticker_frequency: Frequency,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MoveId(u32);
impl MoveId
{
	const NULL: Self = Self(u32::MAX);

	pub fn is_null(&self) -> bool
	{
		*self == Self::NULL
	}

	const fn next(other: Self) -> Self
	{
		Self(other.0.wrapping_add(1))
	}
}
impl Default for MoveId
{
	fn default() -> Self
	{
		Self::NULL
	}
}

impl std::fmt::Debug for MoveId
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		if self.is_null()
		{
			write!(f, "MoveId::NULL")
		}
		else
		{
			f.debug_tuple("MoveId").field(&self.0).finish()
		}
	}
}

impl Planner<N_MOTORS>
{
	pub fn tick(&mut self) -> Option<[i32; N_MOTORS]>
	{
		if self.has_any_move_planned() && self.ready_to_go_blocks_count > 0
		{
			let mut next_position = None;

			communicate_to_ticker::store_blocks_if_necessary(|mut two_blocks_required| {
				if self.ready_to_go_blocks_count < 2
				{
					two_blocks_required = false;
				}

				self.current_move_id = MoveId::next(self.current_move_id);

				let used_blocks_count = 1 + two_blocks_required as usize;
				self.most_optimized_block_index -= used_blocks_count;
				self.ready_to_go_blocks_count -= used_blocks_count;

				// Safety: above we check the ring buffer isn't empty and this function is called at most once, so there must be at least 1 block in the buffer
				let first = unsafe { self.blocks.dequeue().unwrap_unchecked() };
				let second = two_blocks_required.then(|| self.blocks.dequeue()).flatten();

				next_position = self.blocks.front().map(|block| block.steps.clone());

				(first, second)
			});

			next_position
		}
		else
		{
			None
		}
	}
}

impl<const N: usize> Planner<N>
{
	pub fn new(
		blocks_count: usize, stepper_motors_ticker: &StepperMotorsTicker<impl Timer>, settings: Settings<N>,
	) -> Self
	{
		assert!(N >= 2);

		let blocks_power_of_two = blocks_count.ilog2() as usize;

		Self {
			current_position: VectorN::ZERO,
			current_move_id: MoveId::NULL,
			last_move_id: MoveId::NULL,
			blocks: AllocRingBuffer::with_capacity_power_of_2(blocks_power_of_two),
			settings,
			previous_normalized_displacement: None,
			previous_nominal_speed: 0.,
			most_optimized_block_index: 0,
			ready_to_go_blocks_count: 0,
			stepper_ticker_frequency: stepper_motors_ticker.get_tick_frequency(),
		}
	}

	/// Returns a mutable reference to the [`Settings`] you provided to [`Self::new`].
	pub fn get_settings_mut(&mut self) -> &mut Settings<N>
	{
		&mut self.settings
	}

	/// Returns the `target_position` of the last [`planned move`].
	///
	/// [`planned move`]: Self::plan_move
	pub fn get_position(&self) -> &VectorN<N>
	{
		&self.current_position
	}

	/// Manually sets the current position of the planner, making it think that the next planned move will start
	/// from this position.
	pub fn set_position(&mut self, position: VectorN<N>)
	{
		self.current_position = position;
	}

	pub fn mark_last_added_move_as_ready_to_go(&mut self)
	{
		self.ready_to_go_blocks_count += 1;
	}

	pub fn has_move_been_executed(&self, move_to_check: MoveId) -> bool
	{
		move_to_check < self.current_move_id || move_to_check.is_null()
	}

	/// Plans a move that will make the tool move to the `target_position` starting from the `target_position` of the last
	/// planned move.
	///
	/// `steps_per_mm` contains the number of steps required to move the tool by `1mm` on each axis.
	///
	/// The planned move won't be sent to the [`StepperMotorsTicker`] until you call [`Self::mark_last_added_move_as_ready_to_go`].
	///
	/// Check [`Self`] for more info.
	pub fn plan_move<K: Kinematics>(
		// The move speed is equivalent to the feedrate
		&mut self,
		target_position: VectorN<N>,
		steps_per_mm: [f32; N],
		mut move_speed_mm_s: f32,
	) -> Result<MoveId, BlocksBufferIsFull>
	{
		if self.blocks.is_full()
		{
			return Err(BlocksBufferIsFull);
		}

		let mut displacement = target_position.clone() - &self.current_position;
		if displacement == VectorN::ZERO
		{
			return Ok(self.last_move_id);
		}

		let mut block = Block::<N>::default();

		// Calculate how many steps each motor should do to move at the target_position
		let (x, y) = (displacement[Axis::X as usize], displacement[Axis::Y as usize]);
		displacement[0] = K::xy_displacement_to_a(x, y);
		displacement[1] = K::xy_displacement_to_b(x, y);
		for i in 0..N
		{
			block.steps[i] = (displacement[i].as_millimeters_f32() * steps_per_mm[i]) as i32;
		}

		block.step_event_count = *block.steps.iter().max().unwrap() as u32;

		// Here it enable the motors of the axes that do at least 1 step.. but I don't know if I should do it

		// Limit the minimum movement speed
		move_speed_mm_s = move_speed_mm_s.max(
			if displacement[Axis::E as usize] == Distance::ZERO
			{
				self.settings.min_travel_feedrate_mm_s
			}
			else
			{
				self.settings.min_feedrate_mm_s
			},
		);

		// Calculate the duration of the move based on its length and its speed
		let move_length = displacement.length_millimeters();
		let inverse_move_length = 1. / move_length;
		let inverse_move_duration_s = inverse_move_length * move_speed_mm_s;

		block.millimeters = move_length;

		block.nominal_speed = move_speed_mm_s;
		block.nominal_speed_steps_per_sec = (block.step_event_count as f32 * inverse_move_duration_s).ceil();

		// Limit the whole speed based on the max speed on each axis
		let mut speed_on_axes_mm_s = [0.; N];
		let mut speed_factor = 1_f32;
		for i in 0..N
		{
			speed_on_axes_mm_s[i] = displacement[i].as_millimeters_f32() * inverse_move_duration_s;
			speed_factor = speed_factor.min(self.settings.max_feedrate_mm_s[i] / speed_on_axes_mm_s[i]);
		}

		if speed_factor < 1.
		{
			for i in 0..N
			{
				speed_on_axes_mm_s[i] *= speed_factor;
			}
			block.nominal_speed *= speed_factor;
		}

		// Get the acceleration of the move in steps per second²
		let steps_per_length = block.step_event_count as f32 * inverse_move_length;
		let is_retracting = displacement[Axis::E as usize].as_tens_of_nanometers() < 0;
		let mut acceleration_steps_per_sec2 = (steps_per_length
			* if is_retracting
			{
				self.settings.retract_acceleration
			}
			else if displacement[Axis::E as usize] == Distance::ZERO
			{
				self.settings.travel_acceleration
			}
			else
			{
				self.settings.print_acceleration
			})
		.ceil();

		// Limit the acceleration during non-retraction moves
		if !is_retracting
		{
			let mut max_acceleration_steps_per_sec2 = [0.; N];
			for i in 0..N
			{
				max_acceleration_steps_per_sec2[i] = self.settings.max_acceleration_mm_per_s2[i] * steps_per_mm[i];
			}

			let mut limit_acceleration = |axis| {
				if block.steps[axis] != 0 && max_acceleration_steps_per_sec2[axis] < acceleration_steps_per_sec2
				{
					let rate = block.step_event_count as f32 / block.steps[axis] as f32;
					acceleration_steps_per_sec2 =
						acceleration_steps_per_sec2.min(max_acceleration_steps_per_sec2[axis] * rate);
				}
			};

			(limit_acceleration)(Axis::X as usize);
			(limit_acceleration)(Axis::Y as usize);
			(limit_acceleration)(Axis::Z as usize);
			(limit_acceleration)(Axis::E as usize);
		}

		block.acceleration_steps_per_sec2 = acceleration_steps_per_sec2;
		block.acceleration = acceleration_steps_per_sec2 / steps_per_length;
		block.acceleration_rate =
			(acceleration_steps_per_sec2 * ((1 << 24) as f32 / self.stepper_ticker_frequency.as_hertz() as f32)) as u32;

		let normalized_displacement = displacement * inverse_move_length;
		let mut vmax_junction_sqr = 0.;
		if let Some(previous_normalized_displacement) = self.previous_normalized_displacement.as_ref()
		{
			let normalized_displacement_clone = normalized_displacement.clone();
			let junction_cos_theta_distance =
				normalized_displacement_clone.dot(&-previous_normalized_displacement.clone());
			let mut junction_cos_theta = junction_cos_theta_distance.as_micrometers() as f32;

			if junction_cos_theta > 0.99999
			{
				// For a 0 degree acute junction, just set minimum junction speed.
				vmax_junction_sqr = MINIMUM_PLANNER_SPEED * MINIMUM_PLANNER_SPEED;
			}
			else
			{
				let normalized_junction =
					(normalized_displacement_clone - previous_normalized_displacement).normalized();

				let mut junction_acceleration = block.acceleration;
				for i in 0..N
				{
					if normalized_junction[i] != Distance::ZERO
					{
						if junction_acceleration * (normalized_junction[i].as_millimeters() as f32).abs()
							> self.settings.max_acceleration_mm_per_s2[i]
						{
							junction_acceleration = (self.settings.max_acceleration_mm_per_s2[i]
								/ normalized_junction[i].as_millimeters() as f32)
								.abs();
						}
					}
				}

				// Avoid division by 0
				junction_cos_theta = junction_cos_theta.max(-0.99999);

				let sin_theta_d2 = f32::sqrt(0.5 * (1. - junction_cos_theta));
				vmax_junction_sqr = junction_acceleration * JUNCTION_DEVIATION_MM * sin_theta_d2 / (1. - sin_theta_d2);

				// For small moves with >135° junction (octagon) find speed for approximate arc
				if block.millimeters < 1. && junction_cos_theta < -0.7071067812
				{
					let junction_theta = junction_cos_theta.acos();
					let limit_sqr = (block.millimeters * junction_acceleration) / junction_theta;
					vmax_junction_sqr = vmax_junction_sqr.min(limit_sqr);
				}
			}

			vmax_junction_sqr = vmax_junction_sqr.min(block.nominal_speed.sqr().min(self.previous_nominal_speed.sqr()));
		}

		self.previous_normalized_displacement = Some(normalized_displacement);

		block.max_entry_speed_sqr = vmax_junction_sqr;
		block.entry_speed_sqr = MINIMUM_PLANNER_SPEED * MINIMUM_PLANNER_SPEED;

		let max_reachable_speed_sqr =
			Self::max_reachable_speed_sqr(-block.acceleration, MINIMUM_PLANNER_SPEED.sqr(), block.millimeters);
		if block.nominal_speed.sqr() <= max_reachable_speed_sqr
		{
			block.flags.insert(Flag::NominalLength);
		}

		self.previous_nominal_speed = block.nominal_speed;

		self.current_position = target_position;

		self.blocks.push(block);

		self.recalculate(0.);

		self.last_move_id = MoveId::next(self.last_move_id);
		Ok(self.last_move_id)
	}

	pub fn has_any_move_planned(&self) -> bool
	{
		!self.blocks.is_empty()
	}

	pub fn has_any_move_with_set_flags_planned(&self, flags: EnumSet<Flag>) -> bool
	{
		// Should I take in consideration only the moves marked as ready to go?
		self.blocks.iter().any(|block| block.flags.is_superset(flags))
	}

	pub fn set_flags_on_last_added_block(&mut self, flags: EnumSet<Flag>)
	{
		if let Some(block) = self.blocks.back_mut()
		{
			block.flags.insert_all(flags);
		}
	}

	fn calculate_trapezoid_for_block(block: &mut Block<N>, entry_factor: f32, exit_factor: f32)
	{
		let mut initial_rate = (block.nominal_speed_steps_per_sec * entry_factor).ceil();
		let mut final_rate = (block.nominal_speed_steps_per_sec * exit_factor).ceil(); // (steps per second)

		initial_rate = initial_rate.max(MIN_STEP_RATE);
		final_rate = final_rate.max(MIN_STEP_RATE);

		// Steps for acceleration, plateau and deceleration
		let mut plateau_steps = block.step_event_count as i32;
		let mut accelerate_steps = 0;
		let mut decelerate_steps = 0;

		let accel = block.acceleration_steps_per_sec2;
		if accel != 0.
		{
			let inverse_accel = 1. / accel;
			let half_inverse_accel = inverse_accel / 2.;
			let nominal_rate_sq = block.nominal_speed_steps_per_sec.sqr();
			// Steps required for acceleration, deceleration to/from nominal rate
			let decelerate_steps_float = half_inverse_accel * (nominal_rate_sq - final_rate.sqr());
			let mut accelerate_steps_float = half_inverse_accel * (nominal_rate_sq - initial_rate.sqr());
			accelerate_steps = accelerate_steps_float.ceil() as u32;
			decelerate_steps = decelerate_steps_float.floor() as u32;

			// Steps between acceleration and deceleration, if any
			plateau_steps -= (accelerate_steps + decelerate_steps) as i32;

			// Does accelerate_steps + decelerate_steps exceed step_event_count?
			// Then we can't possibly reach the nominal rate, there will be no cruising.
			// Calculate accel / braking time in order to reach the final_rate exactly
			// at the end of this block.
			if plateau_steps < 0
			{
				accelerate_steps_float =
					((block.step_event_count as f32 + accelerate_steps_float - decelerate_steps_float) * 0.5).ceil();
				accelerate_steps = (accelerate_steps_float.max(0.) as u32).min(block.step_event_count);
				decelerate_steps = block.step_event_count - accelerate_steps;
			}
		}

		block.accelerate_until = accelerate_steps;
		block.decelerate_after = block.step_event_count - decelerate_steps;
		block.initial_speed = initial_rate;
		block.final_speed = final_rate;
	}

	/// Calculate the maximum reachable speed squared at this point, in order  to reach
	/// 'target_velocity_sqr' using 'acceleration' within a given 'distance'.
	fn max_reachable_speed_sqr(accel: f32, target_velocity_sqr: f32, distance: f32) -> f32
	{
		return target_velocity_sqr - 2. * accel * distance;
	}

	fn reverse_pass_kernel(current: &mut Block<N>, next: Option<&Block<N>>, safe_exit_speed_sqr: f32)
	{
		// If entry speed is already at the maximum entry speed, and there was no change of speed
		// in the next block, there is no need to recheck. Block is cruising and there is no need to
		// compute anything for this block,
		// If not, block entry speed needs to be recalculated to ensure maximum possible planned speed.
		let max_entry_speed_sqr = current.max_entry_speed_sqr;

		// Compute maximum entry speed decelerating over the current block from its exit speed.
		// If not at the maximum entry speed, or the previous block entry speed changed
		if current.entry_speed_sqr != max_entry_speed_sqr
			|| next.map(|next| next.flags.contains(Flag::Recalculate)).unwrap_or(false)
		{
			// If nominal length true, max junction speed is guaranteed to be reached.
			// If a block can de/ac-celerate from nominal speed to zero within the length of the block, then
			// the current block and next block junction speeds are guaranteed to always be at their maximum
			// junction speeds in deceleration and acceleration, respectively. This is due to how the current
			// block nominal speed limits both the current and next maximum junction speeds. Hence, in both
			// the reverse and forward planners, the corresponding block junction speed will always be at the
			// the maximum junction speed and may always be ignored for any speed reduction checks.

			let next_entry_speed_sqr = match next
			{
				Some(next) => next.entry_speed_sqr,
				None => safe_exit_speed_sqr.max(MINIMUM_PLANNER_SPEED.sqr()),
			};

			let new_entry_speed_sqr = if current.flags.contains(Flag::NominalLength)
			{
				max_entry_speed_sqr
			}
			else
			{
				max_entry_speed_sqr.min(Self::max_reachable_speed_sqr(
					-current.acceleration,
					next_entry_speed_sqr,
					current.millimeters,
				))
			};

			if current.entry_speed_sqr != new_entry_speed_sqr
			{
				current.flags.insert(Flag::Recalculate);
				current.entry_speed_sqr = new_entry_speed_sqr;
			}
		}
	}

	fn reverse_pass(&mut self, safe_exit_speed_sqr: f32)
	{
		// Reverse Pass: Coarsely maximize all possible deceleration curves back-planning from the last
		// block in buffer. Cease planning when the last optimal planned or tail pointer is reached.
		// NOTE: Forward pass will later refine and correct the reverse pass to create an optimal plan.
		let mut next = None;
		for current in self.blocks.iter_mut().rev()
		{
			Self::reverse_pass_kernel(current, next, safe_exit_speed_sqr);
			next = Some(current);
		}
	}

	// The kernel called by recalculate() when scanning the plan from first to last entry.
	fn forward_pass_kernel(
		previous: Option<&Block<N>>, current: &mut Block<N>, current_block_index: usize,
		most_optimized_block_index: &mut usize,
	)
	{
		if let Some(previous) = previous
		{
			// If the previous block is an acceleration block, too short to complete the full speed
			// change, adjust the entry speed accordingly. Entry speeds have already been reset,
			// maximized, and reverse-planned. If nominal length is set, max junction speed is
			// guaranteed to be reached. No need to recheck.
			if !previous.flags.contains(Flag::NominalLength) && previous.entry_speed_sqr < current.entry_speed_sqr
			{
				// Compute the maximum reachable speed
				let new_entry_speed_sqr = Self::max_reachable_speed_sqr(
					-previous.acceleration,
					previous.entry_speed_sqr,
					previous.millimeters,
				);

				// If true, current block is full-acceleration and we can move the planned pointer forward.
				if new_entry_speed_sqr < current.entry_speed_sqr
				{
					// Mark we need to recompute the trapezoidal shape, and do it now,
					// so the stepper ISR does not consume the block before being recalculated
					current.flags.insert(Flag::Recalculate);

					// Always <= max_entry_speed_sqr. Backward pass sets this.
					current.entry_speed_sqr = new_entry_speed_sqr; // Always <= max_entry_speed_sqr. Backward pass sets this.

					// Set optimal plan pointer.
					*most_optimized_block_index = current_block_index;
				}
			}

			// Any block set at its maximum entry speed also creates an optimal plan up to this
			// point in the buffer. When the plan is bracketed by either the beginning of the
			// buffer and a maximum entry speed or two maximum entry speeds, every block in between
			// cannot logically be further improved. Hence, we don't have to recompute them anymore.
			if current.entry_speed_sqr == current.max_entry_speed_sqr
			{
				*most_optimized_block_index = current_block_index;
			}
		}
	}

	fn forward_pass(&mut self)
	{
		// Forward Pass: Forward plan the acceleration curve from the planned pointer onward.
		// Also scans for optimal plan breakpoints and appropriately updates the planned pointer.
		let mut previous = None;
		for (i, current) in self.blocks.iter_mut().enumerate().skip(self.most_optimized_block_index)
		{
			// If there's no previous block or the previous block is not
			// BUSY (thus, modifiable) run the forward_pass_kernel. Otherwise,
			// the previous block became BUSY, so assume the current block's
			// entry speed can't be altered (since that would also require
			// updating the exit speed of the previous block).
			Self::forward_pass_kernel(previous, current, i, &mut self.most_optimized_block_index);
			previous = Some(current);
		}
	}

	fn recalculate(&mut self, safe_exit_speed_sqr: f32)
	{
		// If there is just one block, no planning can be done. Avoid it!
		if self.blocks.len() >= 2
		{
			self.reverse_pass(safe_exit_speed_sqr);
			self.forward_pass();
		}
		self.recalculate_trapezoids(safe_exit_speed_sqr);
	}

	/**
	 * Recalculate the trapezoid speed profiles for all blocks in the plan
	 * according to the entry_factor for each junction. Must be called by
	 * recalculate() after updating the blocks.
	 */
	fn recalculate_trapezoids(&mut self, safe_exit_speed_sqr: f32)
	{
		// Go from the tail (currently executed block) to the first block, without including it)
		let mut previous_option: Option<&mut Block<N>> = None;
		let mut previous_entry_speed = 0.;
		let mut current_entry_speed;
		for current in self.blocks.iter_mut()
		{
			current_entry_speed = current.entry_speed_sqr.sqrt();

			if let Some(previous) = previous_option
			{
				// If the next block is marked to RECALCULATE, also mark the previously-fetched one
				if current.flags.contains(Flag::Recalculate)
				{
					previous.flags.insert(Flag::Recalculate);
				}

				// Recalculate if current block entry or exit junction speed has changed.
				if previous.flags.contains(Flag::Recalculate)
				{
					// NOTE: Entry and exit factors always > 0 by all previous logic operations.
					let nomr = 1. / previous.nominal_speed;
					Self::calculate_trapezoid_for_block(
						previous,
						previous_entry_speed * nomr,
						current_entry_speed * nomr,
					);

					// Reset current only to ensure next trapezoid is computed - The
					// stepper is free to use the block from now on.
					previous.flags.remove(Flag::Recalculate);
				}
			}

			previous_option = Some(current);
			previous_entry_speed = current_entry_speed;
		}

		// Last/newest block in buffer. Always recalculated.
		if let Some(previous) = previous_option
		{
			// Exit speed is set with MINIMUM_PLANNER_SPEED unless some code higher up knows better.
			current_entry_speed = MINIMUM_PLANNER_SPEED.max(safe_exit_speed_sqr.sqrt());

			// Mark the next(last) block as RECALCULATE, to prevent the Stepper ISR running it.
			// As the last block is always recalculated here, there is a chance the block isn't
			// marked as RECALCULATE yet. That's the reason for the following line.
			previous.flags.insert(Flag::Recalculate);

			let nomr = 1. / previous.nominal_speed;
			Self::calculate_trapezoid_for_block(previous, previous_entry_speed * nomr, current_entry_speed * nomr);

			// Reset block to ensure its trapezoid is computed - The stepper is free to use
			// the block from now on.
			previous.flags.remove(Flag::Recalculate);
		}
	}
}

pub struct BlocksBufferIsFull;
