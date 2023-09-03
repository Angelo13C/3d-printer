/// A percentage value.
#[derive(PartialEq, PartialOrd, Clone, Copy, Default, Debug)]
pub struct Percentage(f32);

impl Percentage
{
	/// The `0%` value.
	pub const ZERO: Self = Self(0.);
	/// The `100%` value.
	pub const FULL: Self = Self(1.);

	/// Creates a new [`Percentage`] from the provided value that must be in the range `0..=100`.
	///
	/// Returns `Ok(Percentage)` if the condition above is met, otherwise returns `Err(())`.
	///
	/// # Example
	/// ```
	/// # use firmware_core::utils::math::Percentage;
	/// #
	/// assert!(Percentage::from_0_to_100(50.).is_ok());
	/// assert!(Percentage::from_0_to_100(250.).is_err());
	/// assert!(Percentage::from_0_to_100(-10.).is_err());
	/// ```
	pub fn from_0_to_100(value: f32) -> Result<Self, ()>
	{
		Self::from_0_to_1(value / 100.)
	}

	/// Creates a new [`Percentage`] from the provided value that must be in the range `0..=1`.
	///
	/// Returns `Ok(Percentage)` if the condition above is met, otherwise returns `Err(())`.
	///
	/// # Example
	/// ```
	/// # use firmware_core::utils::math::Percentage;
	/// #
	/// assert!(Percentage::from_0_to_1(0.5).is_ok());
	/// assert!(Percentage::from_0_to_1(2.5).is_err());
	/// assert!(Percentage::from_0_to_1(-0.2).is_err());
	/// ```
	pub fn from_0_to_1(value: f32) -> Result<Self, ()>
	{
		if value >= 0. && value <= 1.
		{
			Ok(Self(value))
		}
		else
		{
			Err(())
		}
	}

	/// Returns the value of the percentage in the range `0..=1`.
	///
	/// # Example
	/// ```
	/// # use firmware_core::utils::math::Percentage;
	/// #
	/// assert_eq!(Percentage::from_0_to_1(1.).unwrap().into_0_to_1(), 1.);
	/// assert_eq!(Percentage::from_0_to_100(30.).unwrap().into_0_to_1(), 0.3);
	/// ```
	pub fn into_0_to_1(&self) -> f32
	{
		self.0
	}

	/// Returns the value of the percentage in the range `0..=100`.
	///
	/// # Example
	/// ```
	/// # use firmware_core::utils::math::Percentage;
	/// #
	/// assert_eq!(Percentage::from_0_to_100(100.).unwrap().into_0_to_100(), 100.);
	/// assert_eq!(Percentage::from_0_to_1(0.8).unwrap().into_0_to_100(), 80.);
	/// ```
	pub fn into_0_to_100(&self) -> f32
	{
		self.into_0_to_1() * 100.
	}
}
