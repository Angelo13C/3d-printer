//! A list of all the identifiers in the parameters of a command supported by this firmware.
//!
//! Check the documentation of [`parameters`] for more details.
//!
//! [`parameters`]: super

use std::{any::TypeId, fmt::Debug, marker::PhantomData};

/// An identifier supported by this firmware.
///
/// Check the [`module's`] documentation for more details.
///
/// [`module's`]: self
pub trait GCodeParameterIdentifier: Copy + Clone + Debug + Default + PartialEq
{
	fn is_this(&mut self, string: &str) -> Result<usize, ()>;
}

/// A [`GCodeParameterIdentifier`] which is a single letter (like `X` or `Y`).
pub trait GCodeParameterIdentifierLetter: GCodeParameterIdentifier
{
	const LETTER: &'static str;
}

macro_rules! impl_g_code_letter_identifiers {
    ($($names: ident),*) => {
        $(
        #[doc="Letter `"]
        #[doc=stringify!($names)]
        #[doc="`."]
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct $names;
        impl GCodeParameterIdentifier for $names
        {
            fn is_this(&mut self, string: &str) -> Result<usize, ()>
            {
                let result = stringify!($names).as_bytes()[0] == string.as_bytes()[0];
                if !result {
                    Err(())
                }
                else {
                    Ok(1)
                }
            }
        }
        impl GCodeParameterIdentifierLetter for $names
        {
            const LETTER: &'static str = stringify!($names);
        }
        )*
    };
}

impl_g_code_letter_identifiers!(E, F, P, R, S, X, Y, Z);

#[derive(Debug, Clone, Copy)]
/// A list of identifiers that could be present in the string of the parameter in any order.
///
/// # Examples
/// ```
/// # use firmware_core::printer::components::g_code::parameters::identifier::*;
/// #
/// assert_eq!(AnyWithoutSpaces::<(X, Y, Z)>::default().is_this("XY").unwrap(), 2);
/// assert_eq!(AnyWithoutSpaces::<(X, Y, Z)>::default().is_this("AH"), Err(()));
/// assert_eq!(AnyWithoutSpaces::<(X, Y, Z)>::default().is_this("Z").unwrap(), 1);
/// assert_eq!(AnyWithoutSpaces::<(X, Y, Z)>::default().is_this("ZYX").unwrap(), 3);
/// assert_eq!(AnyWithoutSpaces::<(X, Y, Z)>::default().is_this("ZYAHX").unwrap(), 3);
/// ```
pub struct AnyWithoutSpaces<T: IdentifierInGroup>(u16, PhantomData<T>);
type BitMask = u16;
impl<T: IdentifierInGroup> AnyWithoutSpaces<T>
{
	/// Returns `true` if the identifier of type `I` is present in the string you provided to [`Self::is_this`]
	/// before calling this function, otherwise returns `false`.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::printer::components::g_code::parameters::identifier::*;
	/// #
	/// let mut identifier = AnyWithoutSpaces::<(X, Y, Z)>::default();
	/// let _ = identifier.is_this("XY");
	/// assert!(identifier.is_identifier_present::<X>());
	/// assert!(identifier.is_identifier_present::<Y>());
	/// assert!(!identifier.is_identifier_present::<Z>());
	/// assert!(!identifier.is_identifier_present::<F>());
	/// ```
	pub fn is_identifier_present<I: 'static>(&self) -> bool
	{
		T::is_identifier_present::<I>(self.0)
	}

	/// Returns `true` if any of the identifiers in contained in the generics of this struct (`T`) is present in the string
	/// you provided to [`Self::is_this`] before calling this function, otherwise returns `false`.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::printer::components::g_code::parameters::identifier::*;
	/// #
	/// let mut identifier = AnyWithoutSpaces::<(X, Y, Z)>::default();
	/// let _ = identifier.is_this("XY");
	/// assert!(identifier.is_any_present());   // Both the X and the Y are present in "XY"
	///
	/// let mut identifier = AnyWithoutSpaces::<(X, Y, Z)>::default();
	/// let _ = identifier.is_this("AH");
	/// assert!(!identifier.is_any_present());  // Neither the X, nor the Y, nor the Z are present in "AH"
	/// ```
	pub fn is_any_present(&self) -> bool
	{
		self.0 != 0
	}
}
impl<T: IdentifierInGroup> GCodeParameterIdentifier for AnyWithoutSpaces<T>
{
	fn is_this(&mut self, string: &str) -> Result<usize, ()>
	{
		let (bits_mask, chars_count) = T::is_this(string);
		self.0 = bits_mask;
		match bits_mask
		{
			0 => Err(()),
			_ => Ok(chars_count),
		}
	}
}

impl<T: IdentifierInGroup> PartialEq for AnyWithoutSpaces<T>
{
	fn eq(&self, other: &Self) -> bool
	{
		self.0 == other.0 && self.1 == other.1
	}
}

/// Used by [`AnyWithoutSpaces`].
pub trait IdentifierInGroup: Copy + Clone + Debug + 'static
{
	fn is_this(string: &str) -> (BitMask, usize);
	fn is_identifier_present<I: 'static>(bit_mask: BitMask) -> bool;
}
macro_rules! impl_identifier_in_group {
    ($($types: ident),*) => {
        #[allow(unused_parens, unused_assignments)]
        impl<$($types: GCodeParameterIdentifierLetter + 'static),*> IdentifierInGroup for ($($types),*)
        {
            fn is_this(string: &str) -> (BitMask, usize) {
                let mut i = 1;
                let mut total_bits = 0;
                let mut total_chars_count = 0;

                $(
                if string.contains($types::LETTER)
                {
                    total_bits |= i;
                    total_chars_count += 1;
                }
                i <<= 1;
                )*

                (total_bits, total_chars_count)
            }

            fn is_identifier_present<I: 'static>(bit_mask: BitMask) -> bool
            {
                let mut i = 1;

                $(
                if TypeId::of::<$types>() == TypeId::of::<I>()
                {
                    return (i & bit_mask) != 0;
                }
                i <<= 1;
                )*

                false
            }
        }
    };
}
impl_identifier_in_group!(T);
impl_identifier_in_group!(T1, T2);
impl_identifier_in_group!(T1, T2, T3);
impl_identifier_in_group!(T1, T2, T3, T4);
impl_identifier_in_group!(T1, T2, T3, T4, T5);
impl_identifier_in_group!(T1, T2, T3, T4, T5, T6);

impl<T: IdentifierInGroup> Default for AnyWithoutSpaces<T>
{
	fn default() -> Self
	{
		Self(0, PhantomData)
	}
}
