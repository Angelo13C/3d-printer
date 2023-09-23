mod cartesian;

pub use cartesian::*;

use crate::utils::measurement::distance::Distance;

/// Type that represents the kinematics of the machine (i.e. how the motion of the `a` and `b` motors effects
/// the movement of the tool carriage).
pub trait Kinematics
{
	/// Converts a displacement on the XY coordinate system to the displacement required to be done by the `a` motor.
	fn xy_displacement_to_a(x: Distance, y: Distance) -> Distance;
	/// Converts a displacement on the XY coordinate system to the displacement required to be done by the `b` motor.
	fn xy_displacement_to_b(x: Distance, y: Distance) -> Distance;

	/// Converts a displacement done by the `a` and `b` motors to a displacement of the tool on the X axis.
	fn ab_displacement_to_x(a: Distance, b: Distance) -> Distance;
	/// Converts a displacement done by the `a` and `b` motors to a displacement of the tool on the Y axis.
	fn ab_displacement_to_y(a: Distance, b: Distance) -> Distance;
}
