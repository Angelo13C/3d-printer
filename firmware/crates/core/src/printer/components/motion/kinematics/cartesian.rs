use super::Kinematics;
use crate::utils::measurement::distance::Distance;

/// ["Cartesian" kinematics](https://all3dp.com/2/cartesian-3d-printer-delta-scara-belt-corexy-polar/#i-3-miscellaneous-rectilinear-cartesian).
///
/// # Examples
/// ```
/// # use firmware_core::{utils::measurement::distance::*, printer::components::motion::kinematics::*};
/// #
/// let x = Distance::from_millimeters(10);
/// let y = Distance::from_millimeters(30);
///
/// assert_eq!(CartesianKinematics::xy_displacement_to_a(x, y), x);
/// assert_eq!(CartesianKinematics::xy_displacement_to_b(x, y), y);
///
/// let a = Distance::from_millimeters(10);
/// let b = Distance::from_millimeters(30);
/// assert_eq!(CartesianKinematics::ab_displacement_to_x(a, b), a);
/// assert_eq!(CartesianKinematics::ab_displacement_to_y(a, b), b);
/// ```
#[derive(Clone,PartialEq, Eq, Debug)]
pub struct CartesianKinematics;

impl Kinematics for CartesianKinematics
{
	fn xy_displacement_to_a(x: Distance, _: Distance) -> Distance
	{
		x
	}

	fn xy_displacement_to_b(_: Distance, y: Distance) -> Distance
	{
		y
	}

	fn ab_displacement_to_x(a: Distance, _: Distance) -> Distance
	{
		a
	}

	fn ab_displacement_to_y(_: Distance, b: Distance) -> Distance
	{
		b
	}
}
