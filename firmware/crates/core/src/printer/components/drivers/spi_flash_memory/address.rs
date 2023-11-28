//! The addresses are stored all in [`little endian`](https://en.wikipedia.org/wiki/Endianness#Byte_addressing).

use std::{fmt::Debug, marker::PhantomData};

use super::chip::FlashMemoryChip;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// Identifies the byte location within a page to be accessed in a flash memory.
pub struct ColumnAddress([u8; 2]);
impl ColumnAddress
{
	/// Returns a `ColumnAddress` from a `column_address` which represents the byte location
	/// within a page and a `plane_index` that represents which plane the page is in.
	pub const fn new(mut column_address: u16, plane_index: u8) -> Self
	{
		column_address |= (plane_index as u16) << 12;
		Self(column_address.to_be_bytes())
	}

	/// Returns the column address as bytes that can be sent over SPI.
	pub const fn as_bytes(&self) -> &[u8; 2]
	{
		&self.0
	}
}

/// Identifies the page, block and LUN to be accessed in a flash memory.
pub struct RowAddress<Chip: FlashMemoryChip>([u8; 3], PhantomData<Chip>);
impl<Chip: FlashMemoryChip> RowAddress<Chip>
{
	/// Returns a [`RowAddress`] from the address of a byte in the memory.
	pub const fn from_page_index(page_index: u32) -> Self
	{
		let bytes = page_index.to_be_bytes();
		Self([bytes[1], bytes[2], bytes[3]], PhantomData)
	}

	/// Returns a [`RowAddress`] from the address of a byte in the memory.
	///
	/// It's the same as [`RowAddress::from_page_index`] with the difference that `address`
	/// is equal to `page_index * Chip::PAGE_SIZE`.
	pub const fn from_memory_address(address: u32) -> Self
	{
		Self::from_page_index(address / Chip::PAGE_SIZE)
	}

	/// Returns the row address as bytes that can be sent over SPI.
	pub const fn as_bytes(&self) -> &[u8; 3]
	{
		&self.0
	}

	/// Returns the index of the page identified by this address.
	pub fn get_page_index(&self) -> u32
	{
		u32::from_be_bytes(std::array::from_fn(|i| self.0.get(i.overflowing_sub(1).0).map(|value| *value).unwrap_or(0)))
	}

	/// Returns the index of the plane of the page identified by this address.
	pub fn get_plane_index(&self) -> u8
	{
		let page_index = self.get_page_index();
		if Chip::PLANES_PER_LUN == 2
		{
			((page_index & Chip::PAGES_PER_BLOCK) > 0) as u8
		}
		else
		{
			((page_index / Chip::PAGES_PER_BLOCK) % Chip::PLANES_PER_LUN) as u8
		}
	}
}

impl<Chip: FlashMemoryChip> Clone for RowAddress<Chip>
{
	fn clone(&self) -> Self
	{
		Self(self.0.clone(), self.1.clone())
	}
}
impl<Chip: FlashMemoryChip> Copy for RowAddress<Chip> {}

impl<Chip: FlashMemoryChip> PartialEq for RowAddress<Chip>
{
	fn eq(&self, other: &Self) -> bool
	{
		self.0 == other.0
	}
}
impl<Chip: FlashMemoryChip> Eq for RowAddress<Chip> {}

impl<Chip: FlashMemoryChip> Debug for RowAddress<Chip>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		f.debug_tuple(&format!("RowAddress<{}>", core::any::type_name::<Chip>()))
			.field(&self.0)
			.finish()
	}
}

#[cfg(test)]
mod tests
{
	use super::{super::MT29F2G01ABAGDWB, *};

	#[test]
	fn row_address_instantation()
	{
		const PAGES_TO_ADDRESS: u32 = 100;
		for i in 0..PAGES_TO_ADDRESS
		{
			assert_eq!(
				RowAddress::<MT29F2G01ABAGDWB>::from_page_index(i),
				RowAddress::<MT29F2G01ABAGDWB>::from_memory_address(i * MT29F2G01ABAGDWB::PAGE_SIZE)
			);
		}
	}
}
