use std::{marker::PhantomData, ops::RangeInclusive};

use crate::printer::components::drivers::spi_flash_memory::*;

pub mod data;
pub mod metadata;

/// Configuration data of the regions of the [`FileSystem`](super::FileSystem).
pub struct RegionsConfig<Chip: FlashMemoryChip>
{
	/// The range of the indices of the flash memory's blocks used by the [`FilesMetadatasRegion`](metadata::FilesMetadatasRegion).
	pub metadata_block_range: RangeInclusive<u16>,
	/// The range of the indices of the flash memory's blocks used by the [`FilesRegion`](data::FilesRegion).
	pub data_block_range: RangeInclusive<u16>,
	pub _chip: PhantomData<Chip>,
}

impl<Chip: FlashMemoryChip> RegionsConfig<Chip>
{
	/// Returns the memory address range used by the [`FilesMetadatasRegion`](metadata::FilesMetadatasRegion).
	///
	/// # Examples
	/// ```
	/// # use firmware_core::printer::components::{file_system::regions::*, drivers::spi_flash_memory::*};
	/// #
	/// let start_metadata_block = 0;
	/// let end_metadata_block = 1;
	/// let start_metadata_address = start_metadata_block as u32 * MT29F2G01ABAGDWB::BLOCK_SIZE;
	/// let end_metadata_address = end_metadata_block as u32 * MT29F2G01ABAGDWB::BLOCK_SIZE;
	///
	/// let regions_config = RegionsConfig::<MT29F2G01ABAGDWB>
	/// {
	/// 	metadata_block_range: start_metadata_block..=end_metadata_block,
	/// 	..Default::default()
	/// };
	///
	/// assert_eq!(regions_config.metadata_address_range(), start_metadata_address..=end_metadata_address);
	/// ```
	pub const fn metadata_address_range(&self) -> RangeInclusive<u32>
	{
		Self::block_range_to_address_range(&self.metadata_block_range)
	}

	/// Returns the memory address range used by the [`FilesRegion`](data::FilesRegion).
	///
	/// # Examples
	/// ```
	/// # use firmware_core::printer::components::{file_system::regions::*, drivers::spi_flash_memory::*};
	/// #
	/// let start_data_block = 0;
	/// let end_data_block = 1;
	/// let start_data_address = start_data_block as u32 * MT29F2G01ABAGDWB::BLOCK_SIZE;
	/// let end_data_address = end_data_block as u32 * MT29F2G01ABAGDWB::BLOCK_SIZE;
	///
	/// let regions_config = RegionsConfig::<MT29F2G01ABAGDWB>
	/// {
	/// 	data_block_range: start_data_block..=end_data_block,
	/// 	..Default::default()
	/// };
	///
	/// assert_eq!(regions_config.data_address_range(), start_data_address..=end_data_address);
	/// ```
	pub const fn data_address_range(&self) -> RangeInclusive<u32>
	{
		Self::block_range_to_address_range(&self.data_block_range)
	}

	const fn block_range_to_address_range(block_range: &RangeInclusive<u16>) -> RangeInclusive<u32>
	{
		*block_range.start() as u32 * Chip::BLOCK_SIZE..=*block_range.end() as u32 * Chip::BLOCK_SIZE
	}
}
impl<Chip: FlashMemoryChip> Default for RegionsConfig<Chip>
{
	fn default() -> Self
	{
		Self {
			metadata_block_range: 0..=0,
			data_block_range: 2..=(Chip::MEMORY_SIZE / Chip::BLOCK_SIZE) as u16,
			_chip: PhantomData,
		}
	}
}
