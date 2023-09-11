use crate::utils::measurement::distance::Distance;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
/// 2 dimensional vector with components stored as [`Distance`].
pub struct Vector2
{
	pub x: Distance,
	pub y: Distance,
}

impl Vector2
{
	/// ```
	/// # use firmware_core::utils::{math::vectors::Vector2, measurement::distance::Distance};
	/// #
	/// assert_eq!(Vector2::ZERO, Vector2 { x: Distance::ZERO, y: Distance::ZERO });
	/// ```
	pub const ZERO: Self = Self {
		x: Distance::ZERO,
		y: Distance::ZERO,
	};
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
/// 3 dimensional vector with components stored as [`Distance`].
pub struct Vector3
{
	pub x: Distance,
	pub y: Distance,
	pub z: Distance,
}

impl Vector3
{
	/// ```
	/// # use firmware_core::utils::{math::vectors::Vector3, measurement::distance::Distance};
	/// #
	/// assert_eq!(Vector3::ZERO, Vector3 { x: Distance::ZERO, y: Distance::ZERO, z: Distance::ZERO });
	/// ```
	pub const ZERO: Self = Self {
		x: Distance::ZERO,
		y: Distance::ZERO,
		z: Distance::ZERO,
	};
}
