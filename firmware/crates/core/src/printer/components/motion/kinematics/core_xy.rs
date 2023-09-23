use super::Kinematics;
use crate::utils::measurement::distance::Distance;

/// [CoreXY kinematics](https://corexy.com).
///
/// # Examples
/// ```
/// # use firmware_core::{utils::measurement::distance::*, printer::components::motion::kinematics::*};
/// #
/// let x = Distance::from_millimeters(10);
/// let y = Distance::from_millimeters(30);
///
/// assert_eq!(CoreXYKinematics::xy_displacement_to_a(x, y), x + y);
/// assert_eq!(CoreXYKinematics::xy_displacement_to_b(x, y), x - y);
///
/// let a = Distance::from_millimeters(10);
/// let b = Distance::from_millimeters(30);
/// assert_eq!(CoreXYKinematics::ab_displacement_to_x(a, b), (a + b) / 2);
/// assert_eq!(CoreXYKinematics::ab_displacement_to_y(a, b), (a - b) / 2);
/// ```
pub struct CoreXYKinematics;

impl Kinematics for CoreXYKinematics
{
	fn xy_displacement_to_a(x: Distance, y: Distance) -> Distance
	{
		x + y
	}

	fn xy_displacement_to_b(x: Distance, y: Distance) -> Distance
	{
		x - y
	}

	fn ab_displacement_to_x(a: Distance, b: Distance) -> Distance
	{
		(a + b) / 2
	}

	fn ab_displacement_to_y(a: Distance, b: Distance) -> Distance
	{
		(a - b) / 2
	}
}
