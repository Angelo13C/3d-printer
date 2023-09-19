/// Extends the functionality of numbers types (like `f32`).
pub trait NumberExt
{
	/// Returns the square of self (`self * self`).
	fn sqr(&self) -> Self;
}

impl NumberExt for f32
{
	/// Returns the square of self (`self * self`).
	///
	/// # Examples
	/// ```
	/// use firmware_core::utils::math::NumberExt;  // You need to import this trait!
	///
	/// assert_eq!(4_f32.sqr(), 4. * 4.);
	/// assert_eq!(2.5.sqr(), 2.5 * 2.5);
	/// assert_eq!(80_f32.sqr().sqrt(), 80_f32);
	/// ```
	fn sqr(&self) -> Self
	{
		*self * *self
	}
}

impl NumberExt for i32
{
	/// Returns the square of self (`self * self`).
	///
	/// # Examples
	/// ```
	/// use firmware_core::utils::math::NumberExt;  // You need to import this trait!
	///
	/// assert_eq!(4.sqr(), 4 * 4);
	/// assert_eq!(25.sqr(), 25 * 25);
	/// ```
	fn sqr(&self) -> Self
	{
		*self * *self
	}
}
