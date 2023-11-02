//! A list of all the values in the parameters of a command supported by this firmware.
//!
//! Check the documentation of [`parameters`] for more details.
//!
//! [`parameters`]: super
use crate::utils::measurement::distance::{Distance, Units};

/// A type of value of a parameter supported by this firmware.
///
/// Check the [`module's`] documentation for more details.
///
/// [`module's`]: self
pub trait GCodeParameterValue
{
	fn from_str(string: &str, units: Units) -> Result<Self, ()>
	where Self: Sized;
}

/// A parameter without a value. This may be useful if you just want to know if a specific identifier is present
/// in the command.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct NoValue;
impl GCodeParameterValue for NoValue
{
	fn from_str(_: &str, _: Units) -> Result<NoValue, ()>
	{
		Ok(NoValue)
	}
}

impl GCodeParameterValue for bool
{
	/// # Examples
	/// ```
	/// # use firmware_core::{utils::measurement::distance::*, printer::components::g_code::parameters::value::GCodeParameterValue};
	/// #
	/// assert_eq!(bool::from_str("1", Units::Millimeters), Ok(true));
	/// assert_eq!(bool::from_str("0", Units::Millimeters), Ok(false));
	/// assert_eq!(bool::from_str("HIGH", Units::Millimeters), Err(()));
	/// assert_eq!(bool::from_str("true", Units::Millimeters), Err(()));
	/// ```
	fn from_str(string: &str, _: Units) -> Result<bool, ()>
	{
		match string
		{
			"1" => Ok(true),
			"0" => Ok(false),
			_ => Err(()),
		}
	}
}

macro_rules! impl_g_code_parameter_values {
    ($($names: ident),*) => {
        $(
			impl GCodeParameterValue for $names
			{
				fn from_str(string: &str, _: Units) -> Result<$names, ()>
				{
					string.parse::<$names>().map_err(|_| ())
				}
			}
		)*
	};
}
impl_g_code_parameter_values!(i8, i16, i32, f32, u8, u16, u32, usize);

impl GCodeParameterValue for Distance
{
	fn from_str(string: &str, units: Units) -> Result<Self, ()>
	where Self: Sized
	{
		units.create_distance(string).map_err(|_| ())
	}
}
