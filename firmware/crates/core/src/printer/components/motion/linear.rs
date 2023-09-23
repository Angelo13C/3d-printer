use crate::utils::measurement::distance::Distance;

/// An helper struct used to calculate the distance taken by an object connected to a mechanism
/// that converts the rotation of a motor to a linear displacement of the object.
pub struct RotationToLinearMotion
{
	tens_of_nanometers_per_microstep: f32,
}

impl RotationToLinearMotion
{
	/// Returns a [`RotationToLinearMotion`] that converts a full revolution of the motor to
	/// `distance_taken_per_revolution`, and a full revolution corresponds to `microsteps_per_revolution`
	/// microsteps.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::{printer::components::motion::*, utils::measurement::distance::*};
	/// #
	/// let distance_taken_per_revolution = Distance::from_millimeters(10);
	/// let microsteps_per_revolution = 200 * 256;
	/// let converter = RotationToLinearMotion::new(
	/// 	distance_taken_per_revolution, microsteps_per_revolution as u32);
	///
	/// assert_eq!(converter.distance_to_microsteps(distance_taken_per_revolution), microsteps_per_revolution);
	/// assert_eq!(converter.distance_to_microsteps(distance_taken_per_revolution * 25), microsteps_per_revolution * 25);
	///
	/// assert_eq!(converter.microsteps_to_distance(microsteps_per_revolution), distance_taken_per_revolution);
	/// assert_eq!(converter.microsteps_to_distance(microsteps_per_revolution * 8), distance_taken_per_revolution * 8);
	/// ```
	pub fn new(distance_taken_per_revolution: Distance, microsteps_per_revolution: u32) -> Self
	{
		Self {
			tens_of_nanometers_per_microstep: distance_taken_per_revolution.as_tens_of_nanometers() as f32
				/ microsteps_per_revolution as f32,
		}
	}

	/// Returns a [`RotationToLinearMotion`] from a belt driven mechanism with `teeth_on_motor_pulley`
	/// teeths on the driven pulley, with the provided `pitch_of_belt`, and that takes `microsteps_per_revolution`
	/// microsteps to do a full revolution.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::{printer::components::motion::*, utils::measurement::distance::*};
	/// #
	/// let pitch_of_belt = Distance::from_millimeters(2);
	/// let teeth_on_motor_pulley = 16;
	/// let microsteps_per_revolution = 200 * 256;
	/// let converter = RotationToLinearMotion::new_connected_to_belt_driven(
	/// 	teeth_on_motor_pulley, pitch_of_belt, microsteps_per_revolution as u32);
	///
	/// assert_eq!(converter.distance_to_microsteps(pitch_of_belt * teeth_on_motor_pulley as i32),
	/// 	microsteps_per_revolution);
	/// assert_eq!(converter.microsteps_to_distance(microsteps_per_revolution),
	/// 	pitch_of_belt * teeth_on_motor_pulley as i32);
	/// ```
	pub fn new_connected_to_belt_driven(
		teeth_on_motor_pulley: u32, pitch_of_belt: Distance, microsteps_per_revolution: u32,
	) -> Self
	{
		Self::new(pitch_of_belt * teeth_on_motor_pulley as i32, microsteps_per_revolution)
	}

	/// Returns a [`RotationToLinearMotion`] from a lead screw mechanism with `starts` number of starts,
	/// with the provided `thread_pitch`, and that takes `microsteps_per_revolution` microsteps to do a
	/// full revolution.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::{printer::components::motion::*, utils::measurement::distance::*};
	/// #
	/// let starts = 4;
	/// let thread_pitch = Distance::from_millimeters(2);
	/// let microsteps_per_revolution = 200 * 256;
	/// let converter = RotationToLinearMotion::new_connected_to_lead_screw(
	/// 	starts, thread_pitch, microsteps_per_revolution);
	///
	/// assert_eq!(converter.distance_to_microsteps(thread_pitch * starts as i32),
	/// 	microsteps_per_revolution as i32);
	/// assert_eq!(converter.microsteps_to_distance(microsteps_per_revolution as i32),
	/// 	thread_pitch * starts as i32);
	/// ```
	pub fn new_connected_to_lead_screw(starts: u32, thread_pitch: Distance, microsteps_per_revolution: u32) -> Self
	{
		Self::new(thread_pitch * starts as i32, microsteps_per_revolution)
	}

	/// Returns the `microsteps` count the motor would need to take for the connected object to be moved
	/// by the provided `distance`.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::{printer::components::motion::*, utils::measurement::distance::*};
	/// #
	/// let distance_taken_per_revolution = Distance::from_millimeters(8);
	/// let microsteps_per_revolution = (200 * 256) as i32;
	/// let converter = RotationToLinearMotion::new(distance_taken_per_revolution, microsteps_per_revolution as u32);
	/// assert_eq!(converter.distance_to_microsteps(distance_taken_per_revolution), microsteps_per_revolution);
	/// assert_eq!(converter.distance_to_microsteps(distance_taken_per_revolution) * 10, microsteps_per_revolution * 10);
	/// assert_eq!(converter.distance_to_microsteps(distance_taken_per_revolution) / 10, microsteps_per_revolution / 10);
	/// ```
	pub fn distance_to_microsteps(&self, distance: Distance) -> i32
	{
		(distance.as_tens_of_nanometers() as f32 / self.tens_of_nanometers_per_microstep).round() as i32
	}

	/// Returns the [`Distance`] the object would be moved if you actually made the motor take the provided
	/// `microsteps` count.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::{printer::components::motion::*, utils::measurement::distance::*};
	/// #
	/// let distance_taken_per_revolution = Distance::from_millimeters(8);
	/// let microsteps_per_revolution = (200 * 256) as i32;
	/// let converter = RotationToLinearMotion::new(distance_taken_per_revolution, microsteps_per_revolution as u32);
	/// assert_eq!(converter.distance_to_microsteps(distance_taken_per_revolution), microsteps_per_revolution);
	/// assert_eq!(converter.distance_to_microsteps(distance_taken_per_revolution) * 10, microsteps_per_revolution * 10);
	/// assert_eq!(converter.distance_to_microsteps(distance_taken_per_revolution) / 10, microsteps_per_revolution / 10);
	/// ```
	pub fn microsteps_to_distance(&self, microsteps: i32) -> Distance
	{
		Distance::from_tens_of_nanometers((self.tens_of_nanometers_per_microstep * microsteps as f32) as i32)
	}
}
