use embedded_hal::spi::{ErrorType, SpiDevice};

use crate::printer::components::drivers::spi_flash_memory::*;

#[derive(Default)]
pub struct BadBlockTable
{
	bad_blocks_indices: Vec<u16>,
}

impl BadBlockTable
{
	/// Creates a bad block table from the first power up of the provided `spi_flash_memory`
	/// based on which blocks contain the bad block mark.
	pub fn from_first_powerup<Chip: FlashMemoryChip, Spi: SpiDevice<u8>>(
		spi_flash_memory: &mut SpiFlashMemory<Chip, Spi>,
	) -> Result<Self, <Spi as ErrorType>::Error>
	{

		let mut bad_blocks_indices = Vec::with_capacity(10);

		for block_index in 0..(Chip::MEMORY_SIZE / Chip::BLOCK_SIZE) as u16
		{
			let is_block_valid = Chip::contains_bad_block_mark(block_index, spi_flash_memory)?;
			if !is_block_valid
			{
				bad_blocks_indices.push(block_index);
			}
		}

		Ok(Self { bad_blocks_indices })
	}

	/// Restores the bad block table from a previous one you [`converted`] to bytes.
	///
	/// [`converted`]: `BadBlockTable::as_bytes`
	pub fn from_bytes(bytes: &[u8]) -> Self
	{
		let mut bad_blocks_indices = Vec::with_capacity(bytes.len() / 2);
		bytes
			.windows(2)
			.for_each(|bytes| bad_blocks_indices.push(u16::from_le_bytes([bytes[0], bytes[1]])));

		Self { bad_blocks_indices }
	}

	pub fn mark_block_as_invalid(&mut self, block_index: u16)
	{
		if !self.bad_blocks_indices.contains(&block_index)
		{
			self.bad_blocks_indices.push(block_index);
		}
	}

	pub fn is_block_valid(&self, block_index: u16) -> bool
	{
		self.bad_blocks_indices.contains(&block_index)
	}

	pub fn as_bytes(&self) -> BadBlockTableBytes<'_>
	{
		BadBlockTableBytes {
			bad_blocks_table: self,
			i: 0,
		}
	}

	pub fn indices(&self) -> &[u16]
	{
		&self.bad_blocks_indices
	}
}

pub struct BadBlockTableBytes<'a>
{
	bad_blocks_table: &'a BadBlockTable,
	i: usize,
}

impl<'a> Iterator for BadBlockTableBytes<'a>
{
	type Item = u8;

	fn next(&mut self) -> Option<Self::Item>
	{
		if self.i == 0
		{
			self.i += 1;

			Some(self.bad_blocks_table.bad_blocks_indices.len() as u8)
		}
		else if (self.i - 1) * 2 < self.bad_blocks_table.bad_blocks_indices.len()
		{
			self.i += 1;

			let i = self.i - 1;

			let block = self.bad_blocks_table.bad_blocks_indices[i / 2];

			Some(block.to_le_bytes()[i % 2])
		}
		else
		{
			None
		}
	}
}
