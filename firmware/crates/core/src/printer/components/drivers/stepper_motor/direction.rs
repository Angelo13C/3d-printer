#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
/// Direction of rotation.
pub enum RotationalDirection
{
	/// Clockwise
	CW = 1,
	/// Counter-clockwise
	CCW = -1,
}

impl RotationalDirection
{
	/// Returns [`RotationalDirection::CW`] if the provided `value` has a positive sign, otherwise returns [`RotationalDirection::CCW`].
	///
	/// # Examples
	/// ```
	/// # use firmware_core::printer::components::drivers::stepper_motor::*;
	/// #
	/// assert_eq!(RotationalDirection::from_sign(40.4), RotationalDirection::CW);
	/// assert_eq!(RotationalDirection::from_sign(0.), RotationalDirection::CW);
	/// assert_eq!(RotationalDirection::from_sign(-1.), RotationalDirection::CCW);
	/// ```
	pub fn from_sign(value: f32) -> Self
	{
		match value.is_sign_positive()
		{
			true => Self::CW,
			false => Self::CCW,
		}
	}

	/// Returns [`RotationalDirection::CW`] if `self` is [`RotationalDirection::CCW`], otherwise returns [`RotationalDirection::CCW`].
	///
	/// # Examples
	/// ```
	/// # use firmware_core::printer::components::drivers::stepper_motor::*;
	/// #
	/// assert_eq!(RotationalDirection::CW.opposite(), RotationalDirection::CCW);
	/// assert_eq!(RotationalDirection::CCW.opposite(), RotationalDirection::CW);
	/// ```
	pub fn opposite(&self) -> Self
	{
		match self
		{
			RotationalDirection::CW => RotationalDirection::CCW,
			RotationalDirection::CCW => RotationalDirection::CW,
		}
	}
}
