use std::ops::*;

use super::NumberExt;
use crate::utils::measurement::distance::Distance;

pub type Vector2 = VectorN<2>;
pub type Vector3 = VectorN<3>;

impl Vector2
{
	/// # Examples
	/// ```
	/// # use firmware_core::utils::{measurement::distance::Distance, math::vectors::*};
	/// #
	/// let x = Distance::from_millimeters(10);
	/// let y = Distance::from_millimeters(15);
	///
	/// let vector2 = Vector2::from_xy(x, y);
	///
	/// assert_eq!(vector2.x(), x);
	/// assert_eq!(vector2.y(), y);
	/// ```
	pub const fn from_xy(x: Distance, y: Distance) -> Self
	{
		Self([x, y])
	}

	pub fn x(&self) -> Distance
	{
		self.0[0]
	}
	pub fn y(&self) -> Distance
	{
		self.0[1]
	}
}

impl Vector3
{
	/// # Examples
	/// ```
	/// # use firmware_core::utils::{measurement::distance::Distance, math::vectors::*};
	/// #
	/// let x = Distance::from_millimeters(10);
	/// let y = Distance::from_millimeters(15);
	/// let z = Distance::from_millimeters(5);
	///
	/// let vector3 = Vector3::from_xyz(x, y, z);
	///
	/// assert_eq!(vector3.x(), x);
	/// assert_eq!(vector3.y(), y);
	/// assert_eq!(vector3.z(), z);
	/// ```
	pub const fn from_xyz(x: Distance, y: Distance, z: Distance) -> Self
	{
		Self([x, y, z])
	}

	pub fn x(&self) -> Distance
	{
		self.0[0]
	}
	pub fn y(&self) -> Distance
	{
		self.0[1]
	}
	pub fn z(&self) -> Distance
	{
		self.0[2]
	}
}

impl Copy for Vector2 {}
impl Copy for Vector3 {}

#[derive(PartialEq, Eq, Debug, Clone)]
/// `N` dimensional vector with components stored as [`Distance`].
pub struct VectorN<const N: usize>([Distance; N]);
impl<const N: usize> Index<usize> for VectorN<N>
{
	type Output = Distance;

	fn index(&self, index: usize) -> &Self::Output
	{
		&self.0[index]
	}
}
impl<const N: usize> IndexMut<usize> for VectorN<N>
{
	fn index_mut(&mut self, index: usize) -> &mut Self::Output
	{
		&mut self.0[index]
	}
}

impl<const N: usize> Add<&Self> for VectorN<N>
{
	type Output = Self;

	fn add(mut self, rhs: &Self) -> Self::Output
	{
		for i in 0..N
		{
			self[i] += rhs[i]
		}
		self
	}
}

impl<const N: usize> Sub<&Self> for VectorN<N>
{
	type Output = Self;

	fn sub(mut self, rhs: &Self) -> Self::Output
	{
		for i in 0..N
		{
			self[i] -= rhs[i]
		}
		self
	}
}
impl<const N: usize> Neg for VectorN<N>
{
	type Output = Self;

	fn neg(mut self) -> Self::Output
	{
		for i in 0..N
		{
			self[i] = -self[i];
		}
		self
	}
}

impl<const N: usize> Mul<f32> for VectorN<N>
{
	type Output = Self;

	fn mul(mut self, rhs: f32) -> Self::Output
	{
		self *= rhs;
		self
	}
}
impl<const N: usize> MulAssign<f32> for VectorN<N>
{
	fn mul_assign(&mut self, rhs: f32)
	{
		for i in 0..N
		{
			self[i] *= rhs
		}
	}
}

impl<const N: usize> VectorN<N>
{
	/// ```
	/// # use firmware_core::utils::math::vectors::*;
	/// #
	/// assert_eq!(VectorN::<2>::ZERO.length_millimeters(), 0.);
	/// assert_eq!(VectorN::<3>::ZERO.length_millimeters(), 0.);
	/// assert_eq!(VectorN::<4>::ZERO.length_millimeters(), 0.);
	/// // ...
	/// ```
	pub const ZERO: Self = Self([Distance::ZERO; N]);

	pub const fn new(components: [Distance; N]) -> Self
	{
		Self(components)
	}

	/// Returns the square of the length of this vector in millimeters.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::{measurement::distance::Distance, math::{*, vectors::*}};
	/// #
	/// let x = Distance::from_millimeters(3);
	/// let y = Distance::from_millimeters(4);
	///
	/// let vector2 = VectorN::<2>::from_xy(x, y);
	///
	/// assert_eq!(vector2.length_millimeters_sqr() as i32, x.as_millimeters().sqr() + y.as_millimeters().sqr());
	/// ```
	pub fn length_millimeters_sqr(&self) -> f32
	{
		self.0
			.iter()
			.fold(0., |length, &current| length + current.as_millimeters_f32().sqr())
	}

	/// Returns the length of this vector in millimeters.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::{measurement::distance::Distance, math::{*, vectors::*}};
	/// #
	/// let x = Distance::from_millimeters(3);
	/// let y = Distance::from_millimeters(4);
	///
	/// let vector2 = VectorN::<2>::from_xy(x, y);
	///
	/// assert_eq!(vector2.length_millimeters(), vector2.length_millimeters_sqr().sqrt());
	/// ```
	pub fn length_millimeters(&self) -> f32
	{
		self.length_millimeters_sqr().sqrt()
	}

	pub fn normalized(&self) -> Self
	{
		self.clone() * (1. / self.length_millimeters())
	}

	/// Returns the [`dot product`] of this vector with `other`.
	///
	/// [`dot product`]: https://en.wikipedia.org/wiki/Dot_product
	pub fn dot(&self, other: &Self) -> Distance
	{
		let mut result = Distance::ZERO;
		for i in 0..N
		{
			result +=
				Distance::from_tens_of_nanometers(self[i].as_tens_of_nanometers() * other[i].as_tens_of_nanometers());
		}
		result
	}
}
