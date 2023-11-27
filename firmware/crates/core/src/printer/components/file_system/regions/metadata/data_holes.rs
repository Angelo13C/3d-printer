use std::{marker::PhantomData, ops::RangeInclusive, fmt::Debug};

use super::FilesMetadatasRegion;
use crate::{
	printer::components::{drivers::spi_flash_memory::*, file_system::RegionsConfig},
	utils::math::NumberExt,
};

pub struct DataHoles<Chip: FlashMemoryChip>
{
	block_holes: Vec<RangeInclusive<u16>>,
	_chip: PhantomData<Chip>,
}

impl<Chip: FlashMemoryChip> DataHoles<Chip>
{
	pub fn from_metadatas_region(files_metadatas_region: &FilesMetadatasRegion, regions_config: &RegionsConfig)
		-> Self
	{
		let mut used_slots: Vec<RangeInclusive<u16>> = files_metadatas_region
			.bad_block_table
			.indices()
			.iter()
			.map(|value| *value..=*value)
			.chain(core::iter::once(regions_config.metadata_block_range.clone()))
			.chain(files_metadatas_region.files_metadatas.iter().map(|metadata| {
				let start_block_index = Chip::get_block_index_of_address(metadata.start_memory_address);
				let end_block_index = Chip::get_block_index_of_address(metadata.end_memory_address());
				start_block_index..=end_block_index
			}))
			.collect();

		used_slots.sort_by(|a, b| a.start().cmp(b.start()));

		let mut holes = Vec::with_capacity(used_slots.len() + 1);

		if *used_slots[0].start() != 0
		{
			holes.push(0..=(*used_slots[0].start() - 1));
		}

		for i in 0..(used_slots.len() - 1)
		{
			holes.push((used_slots[i].end() + 1)..=(used_slots[i + 1].start() - 1));
		}
		if let Some(last_hole) = used_slots.last()
		{
			holes.push((*last_hole.end() + 1)..=Chip::get_block_index_of_address(Chip::MEMORY_SIZE));
		}

		holes.retain(|hole| !hole.is_empty());

		Self {
			block_holes: holes,
			_chip: PhantomData,
		}
	}

	/// Check if there's enough space in the data holes to contain `data_size` bytes.
	pub fn find_space_for_new_data(&self, data_size: u32) -> FreeSpace
	{
		let required_blocks = data_size.ceil_div(Chip::BLOCK_SIZE) as u16;

		let mut total_available_space = 0;
		for hole in &self.block_holes
		{
			let hole_size = hole.len() as u16;
			if hole_size >= required_blocks
			{
				return FreeSpace::Available {
					start_address: Chip::get_address_of_block_index(*hole.start()),
				};
			}

			total_available_space += hole_size;
		}

		match total_available_space >= required_blocks
		{
			true => FreeSpace::AvailableButRequiresCompacting,
			false => FreeSpace::NotAvailable,
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FreeSpace
{
	/// There's not enough space in the memory to store the file.
	NotAvailable,
	/// There aren't holes large enough to store the file, but if you compact the memory an hole of the required space
	/// will be available.
	AvailableButRequiresCompacting,
	/// There's enough space at `start_address` to store the file.
	Available
	{
		start_address: u32
	},
}

impl<Chip: FlashMemoryChip> Debug for DataHoles<Chip>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataHoles").field("block_holes", &self.block_holes).finish()
    }
}