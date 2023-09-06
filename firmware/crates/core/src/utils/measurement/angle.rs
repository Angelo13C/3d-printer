use std::{
	fmt::{Debug, Display},
	ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub type Angle = AngleGeneric<f32>;
pub type AngleDouble = AngleGeneric<f64>;

#[derive(Default, Clone, Copy)]
/// An angle value.
pub struct AngleGeneric<T: AngleFunctions>
{
	radians: T,
}

type CyclesCount = u32;
impl<T: AngleFunctions> AngleGeneric<T>
{
	pub const ZERO: Self = Self::from_radians(T::ZERO_ANGLE);
	pub const RIGHT: Self = Self::from_radians(T::RIGHT_ANGLE);
	pub const FLAT: Self = Self::from_radians(T::FLAT_ANGLE);
	pub const ROUND: Self = Self::from_radians(T::ROUND_ANGLE);

	pub const EPSILON: Self = Self::from_radians(T::EPSILON_ANGLE);

	pub const INVALID: Self = Self::from_radians(T::INVALID_ANGLE);

	#[inline]
	pub const fn from_radians(radians: T) -> Self
	{
		Self { radians }
	}

	#[inline]
	pub fn from_degrees(angle: T) -> Self
	{
		Self::from_radians(angle.to_radians())
	}

	#[inline]
	pub const fn into_radians(self) -> T
	{
		self.radians
	}

	#[inline]
	pub fn into_degrees(self) -> T
	{
		self.into_radians().to_degrees()
	}

	pub fn mul_by_constant(self, constant: T) -> Self
	{
		Self::from_radians(self.into_radians() * constant)
	}

	#[inline]
	/// Wrap the angle to range [-180°, 180°).
	pub fn wrap_180(self) -> Self
	{
		Self::from_radians(self.into_radians().wrap_180())
	}

	#[inline]
	/// Wrap the angle in the [0°, 360°) range.
	pub fn wrap_360(self) -> Self
	{
		Self::from_radians(self.into_radians().wrap_360())
	}

	#[inline]
	/// Returns the number of cycles this angle has.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::angle::deg;
	/// #
	/// assert_eq!(deg(750.).cycles_count(), 2);
	/// assert_eq!(deg(90.).cycles_count(), 0);
	/// assert_eq!(deg(-400.).cycles_count(), 1);
	/// ```
	pub fn cycles_count(self) -> CyclesCount
	{
		self.into_radians().cycles_count()
	}

	#[inline]
	pub fn sin(self) -> T
	{
		self.into_radians().sin()
	}
	#[inline]
	pub fn cos(self) -> T
	{
		self.into_radians().cos()
	}
	#[inline]
	pub fn tan(self) -> T
	{
		self.into_radians().tan()
	}
	#[inline]
	/// Computes the arccosine of a number. Return value is Self::INVALID if the number is outside the range [-1, 1].
	pub fn asin(value: T) -> Self
	{
		Self::from_radians(value.asin())
	}
	#[inline]
	pub fn acos(value: T) -> Self
	{
		Self::from_radians(value.acos())
	}
	#[inline]
	pub fn atan(value: T) -> Self
	{
		Self::from_radians(value.atan())
	}
	#[inline]
	pub fn atan2(self, other: AngleGeneric<T>) -> Self
	{
		Self::from_radians(self.into_radians().atan2(other.into_radians()))
	}
	#[inline]
	pub fn sin_cos(self) -> (T, T)
	{
		self.into_radians().sin_cos()
	}
	#[inline]
	pub fn sinh(self) -> T
	{
		self.into_radians().sinh()
	}
	#[inline]
	pub fn cosh(self) -> T
	{
		self.into_radians().cosh()
	}
	#[inline]
	pub fn tanh(self) -> T
	{
		self.into_radians().tanh()
	}
	#[inline]
	pub fn asinh(value: T) -> Self
	{
		Self::from_radians(value.asinh())
	}
	#[inline]
	pub fn acosh(value: T) -> Self
	{
		Self::from_radians(value.acosh())
	}
	#[inline]
	pub fn atanh(value: T) -> Self
	{
		Self::from_radians(value.atanh())
	}

	#[inline]
	/// Check if the inner angle is invalid (it could be invalid after using asin, acos or atan).
	pub fn is_invalid(self) -> bool
	{
		self.into_radians().is_invalid()
	}

	#[inline]
	/// Get the minimum of this and the other angle.
	///
	/// # Note
	/// The returned value is wrapped in the range [0°, 360°).
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::measurement::angle::deg;
	/// #
	/// let minimum = deg(370.);
	/// let maximum = deg(50.);
	/// assert_eq!(minimum.min(maximum), minimum);
	/// assert_eq!(maximum.min(minimum), minimum);
	/// assert_eq!(minimum.min(maximum), deg(10.));
	/// ```
	pub fn min(self, other: Self) -> Self
	{
		let self_360 = self.wrap_360();
		let other_360 = other.wrap_360();
		if self_360.into_radians() > other_360.into_radians()
		{
			other_360
		}
		else
		{
			self_360
		}
	}
	#[inline]
	pub fn max(self, other: Self) -> Self
	{
		let self_360 = self.wrap_360();
		let other_360 = other.wrap_360();
		if self_360.into_radians() < other_360.into_radians()
		{
			other_360
		}
		else
		{
			self_360
		}
	}
	#[inline]
	pub fn clamp(self, min: Self, max: Self) -> Self
	{
		//This should be a more optimized version of using min and max, but it's not really readable
		let self_360 = self.wrap_360();
		let min_360 = min.wrap_360();
		let max_360 = max.wrap_360();
		if self_360.into_radians() < min_360.into_radians()
		{
			min_360
		}
		else if self_360.into_radians() > max_360.into_radians()
		{
			max_360
		}
		else
		{
			self_360
		}
	}
}

#[inline]
/// Returns an [`angle`] from a value in degrees.
///
/// [`angle`]: `AngleGeneric`
pub fn deg<T: AngleFunctions>(angle: T) -> AngleGeneric<T>
{
	AngleGeneric::<T>::from_degrees(angle)
}

#[inline]
/// Returns an [`angle`] from a value in radians.
///
/// [`angle`]: `AngleGeneric`
pub fn rad<T: AngleFunctions>(angle: T) -> AngleGeneric<T>
{
	AngleGeneric::<T>::from_radians(angle)
}

pub trait AngleFunctions:
	EpsilonAngle
	+ Add<Output = Self>
	+ AddAssign
	+ Sub<Output = Self>
	+ SubAssign
	+ Mul<Output = Self>
	+ MulAssign
	+ Div<Output = Self>
	+ DivAssign
	+ Neg<Output = Self>
	+ PartialEq
	+ PartialOrd
	+ Clone
	+ Copy
where Self: Sized
{
	const ZERO_ANGLE: Self;
	const RIGHT_ANGLE: Self;
	const FLAT_ANGLE: Self;
	const ROUND_ANGLE: Self;

	const INVALID_ANGLE: Self;

	fn to_radians(self) -> Self;
	fn to_degrees(self) -> Self;

	fn abs(self) -> Self;

	fn wrap_180(self) -> Self;
	fn wrap_360(self) -> Self;
	fn cycles_count(self) -> CyclesCount;

	fn sin(self) -> Self;
	fn cos(self) -> Self;
	fn tan(self) -> Self;
	fn asin(self) -> Self;
	fn acos(self) -> Self;
	fn atan(self) -> Self;
	fn atan2(self, other: Self) -> Self;
	fn sin_cos(self) -> (Self, Self);
	fn sinh(self) -> Self;
	fn cosh(self) -> Self;
	fn tanh(self) -> Self;
	fn asinh(self) -> Self;
	fn acosh(self) -> Self;
	fn atanh(self) -> Self;

	fn is_invalid(self) -> bool;
}
pub trait EpsilonAngle
{
	const EPSILON_ANGLE: Self;
}

macro_rules! inherit_fn {
	($name: ident) => {
		inherit_fn!($name, Self);
	};
	($name: ident, $return_ty: ty) => {
		#[inline]
		fn $name(self) -> $return_ty
		{
			self.$name()
		}
	};
}
macro_rules! impl_angle_functions {
	($type: ty) => {
		impl AngleFunctions for $type
		{
			const ZERO_ANGLE: Self = 0.0 as Self;
			const RIGHT_ANGLE: Self = Self::FLAT_ANGLE / (2.0 as Self);
			const FLAT_ANGLE: Self = std::f64::consts::PI as Self;
			const ROUND_ANGLE: Self = Self::FLAT_ANGLE * (2.0 as Self);

			const INVALID_ANGLE: Self = Self::NAN;

			inherit_fn!(to_radians);
			inherit_fn!(to_degrees);

			inherit_fn!(abs);

			#[inline]
			fn wrap_180(self) -> Self
			{
				(self - Self::EPSILON) % Self::FLAT_ANGLE
			}
			#[inline]
			fn wrap_360(self) -> Self
			{
				self.rem_euclid(Self::ROUND_ANGLE)
			}
			#[inline]
			fn cycles_count(self) -> CyclesCount
			{
				(self / Self::ROUND_ANGLE).abs().floor() as CyclesCount
			}

			inherit_fn!(sin);
			inherit_fn!(cos);
			inherit_fn!(tan);
			inherit_fn!(asin);
			inherit_fn!(acos);
			inherit_fn!(atan);
			#[inline]
			fn atan2(self, other: Self) -> Self
			{
				self.atan2(other)
			}
			inherit_fn!(sin_cos, (Self, Self));
			inherit_fn!(sinh);
			inherit_fn!(cosh);
			inherit_fn!(tanh);
			inherit_fn!(asinh);
			inherit_fn!(acosh);
			inherit_fn!(atanh);

			#[inline]
			fn is_invalid(self) -> bool
			{
				self.is_finite()
			}
		}
	};
}

impl_angle_functions!(f32);
impl_angle_functions!(f64);
impl EpsilonAngle for f32
{
	//This is 10^-3 degrees in radians
	const EPSILON_ANGLE: Self = 0.000017453292519943296;
}
impl EpsilonAngle for f64
{
	//This is 10^-9 degrees in radians
	const EPSILON_ANGLE: Self = 0.000000000017453292;
}

impl<T: AngleFunctions> Add for AngleGeneric<T>
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output
	{
		rad(self.into_radians() + rhs.into_radians())
	}
}
impl<T: AngleFunctions> AddAssign for AngleGeneric<T>
{
	fn add_assign(&mut self, rhs: Self)
	{
		self.radians += rhs.into_radians()
	}
}

impl<T: AngleFunctions> Sub for AngleGeneric<T>
{
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output
	{
		rad(self.into_radians() - rhs.into_radians())
	}
}
impl<T: AngleFunctions> SubAssign for AngleGeneric<T>
{
	fn sub_assign(&mut self, rhs: Self)
	{
		self.radians -= rhs.into_radians();
	}
}

impl<T: AngleFunctions> Mul<T> for AngleGeneric<T>
{
	type Output = Self;

	fn mul(self, rhs: T) -> Self::Output
	{
		rad(self.into_radians() * rhs)
	}
}
impl<T: AngleFunctions> MulAssign<T> for AngleGeneric<T>
{
	fn mul_assign(&mut self, rhs: T)
	{
		self.radians *= rhs
	}
}

impl<T: AngleFunctions> Div<T> for AngleGeneric<T>
{
	type Output = Self;

	fn div(self, rhs: T) -> Self::Output
	{
		rad(self.into_radians() / rhs)
	}
}
impl<T: AngleFunctions> DivAssign<T> for AngleGeneric<T>
{
	fn div_assign(&mut self, rhs: T)
	{
		self.radians /= rhs
	}
}
impl<T: AngleFunctions> Neg for AngleGeneric<T>
{
	type Output = Self;

	fn neg(self) -> Self::Output
	{
		rad(-self.into_radians())
	}
}

impl<T: AngleFunctions> PartialEq for AngleGeneric<T>
{
	fn eq(&self, other: &Self) -> bool
	{
		(self.wrap_360().into_radians() - other.wrap_360().into_radians()).abs() <= T::EPSILON_ANGLE
	}
}

impl<T: AngleFunctions> PartialOrd for AngleGeneric<T>
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
	{
		self.wrap_360()
			.into_radians()
			.partial_cmp(&other.wrap_360().into_radians())
	}
}

impl<T: AngleFunctions + Display> Display for AngleGeneric<T>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "{}°", self.into_degrees())
	}
}
impl<T: AngleFunctions + Display> Debug for AngleGeneric<T>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "{} rad", self.into_radians())
	}
}

#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn operations()
	{
		assert_eq!(-Angle::RIGHT, deg(-90.0));
		assert_eq!(Angle::RIGHT - Angle::RIGHT, Angle::ZERO);
	}

	#[test]
	fn comparisons()
	{
		assert_eq!(Angle::ZERO, Angle::EPSILON);
		assert_ne!(Angle::ZERO, Angle::EPSILON + deg(0.000001));
		assert_eq!(Angle::ZERO, Angle::ROUND);

		assert!(Angle::FLAT > Angle::RIGHT);
		assert!(Angle::RIGHT > Angle::ZERO);
	}

	#[test]
	fn wrap()
	{
		assert_eq!(Angle::FLAT, Angle::FLAT.wrap_180());
		assert_eq!(-Angle::FLAT, -Angle::FLAT.wrap_180());
		assert_eq!(Angle::FLAT, (Angle::FLAT - Angle::EPSILON).wrap_180());
		assert_eq!(Angle::ZERO, (Angle::FLAT + Angle::EPSILON).wrap_180());

		let angle = Angle::RIGHT + Angle::ROUND * 2.0;
		assert_eq!(Angle::RIGHT, angle.wrap_360());
		assert_eq!(Angle::RIGHT, angle.wrap_180());

		let angle = -Angle::RIGHT - Angle::ROUND * 2.0;
		assert_eq!(-Angle::RIGHT.wrap_360(), angle.wrap_360());
		assert_eq!(-Angle::RIGHT, angle.wrap_180());

		let angle = Angle::FLAT + Angle::RIGHT;
		assert_eq!(angle, (angle + Angle::ROUND).wrap_360());
		assert_eq!(Angle::RIGHT, angle.wrap_180());

		let cycles = 100;
		let angle = Angle::ROUND * (cycles as f32);
		assert_eq!(cycles, angle.cycles_count());
		assert_eq!(cycles, (-angle).cycles_count());
	}

	#[test]
	fn display_and_debug()
	{
		let angle = Angle::FLAT;
		assert_eq!("180°", format!("{}", angle));
		assert_eq!(format!("{} rad", std::f32::consts::PI), format!("{:#?}", angle));
	}
}
