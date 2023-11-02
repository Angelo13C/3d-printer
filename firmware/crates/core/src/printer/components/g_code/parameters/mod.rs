//! [`G-code commands`] can have 0, 1, or more parameters.
//! Each [`parameter`] starts with an [`identifier`] (like `X` or `Y`) and then there is the [`value`] of the parameter
//! (which can be a string, a bool, a float...)
//!
//! # Examples
//! ```text
//! X20.1
//! ```
//! ```text
//! Y-5.3
//! ```
//! ```text
//! S60
//! ```
//!
//! [`parameter`]: GCodeParameter
//! [`identifier`]: GCodeParameterIdentifier
//! [`value`]: GCodeParameterValue
//! [`G-code commands`]: super::commands

use self::{identifier::GCodeParameterIdentifier, value::GCodeParameterValue};
use crate::utils::measurement::distance::Units;

pub mod identifier;
pub mod value;

/// A parameter of a G-code command.
///
/// Check the [`module's`] documentation for more details.
///
/// [`module's`]: self
pub trait GCodeParameter
{
	type Value;

	fn convert(string: &str, units: Units) -> Option<Self::Value>;
}

impl<I: GCodeParameterIdentifier, V: GCodeParameterValue> GCodeParameter for (I, V)
{
	type Value = (I, V);

	fn convert(string: &str, units: Units) -> Option<(I, V)>
	{
		let mut identifier = I::default();
		if let Ok(offset) = identifier.is_this(string)
		{
			let offseted_string = &string[offset..];
			if let Ok(result) = V::from_str(offseted_string, units)
			{
				Some((identifier, result))
			}
			else
			{
				None
			}
		}
		else
		{
			None
		}
	}
}
