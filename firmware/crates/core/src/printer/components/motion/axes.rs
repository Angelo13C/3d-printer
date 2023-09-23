//! Helpers to deal with the axes of the machine.

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[repr(u8)]
/// Represents an axis of the machine.
pub enum Axis
{
	/// `X` axis.
	X,

	/// `Y` axis.
	Y,

	/// `Z` axis.
	Z,

	/// `Extruder` axis.
	E,
}
