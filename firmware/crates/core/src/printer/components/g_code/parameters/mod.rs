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

pub mod identifier;
pub mod value;

#[derive(Clone, Copy, Debug, PartialEq)]
/// A parameter of a G-code command.
///
/// Check the [`module's`] documentation for more details.
///
/// [`module's`]: self
pub struct Param<I: GCodeParameterIdentifier, V: GCodeParameterValue>
{
	identifier: I,
	value: Option<V>,
}

impl<I: GCodeParameterIdentifier, V: GCodeParameterValue> Param<I, V>
{
	pub fn new(identifier: Option<I>, value: Option<V>) -> Self
	{
		Self {
			identifier: identifier.unwrap_or_default(),
			value,
		}
	}
}

impl<I: GCodeParameterIdentifier, V: GCodeParameterValue> From<Option<V>> for Param<I, V>
{
	fn from(value: Option<V>) -> Self
	{
		Self::new(None, value)
	}
}
impl<I: GCodeParameterIdentifier, V: GCodeParameterValue> From<V> for Param<I, V>
{
	fn from(value: V) -> Self
	{
		Self::new(None, Some(value))
	}
}
