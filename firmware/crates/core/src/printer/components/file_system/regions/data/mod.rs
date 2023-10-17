mod file_reader;
mod file_writer;

use std::marker::PhantomData;

use embedded_hal::spi::{ErrorType, SpiDevice};
pub use file_reader::*;
pub use file_writer::*;

use super::metadata::{FileMetadata, FileMetadataValidator};
use crate::printer::components::drivers::spi_flash_memory::{FlashMemoryChip, FlashMemoryChipExt, SpiFlashMemory};

/// There are 2 assumptions used in all the methods of this struct that must be held:
/// - 2 files can't partially occupy the same block (basically the allocation unit size of the file system is the size
/// of 1 block of the flash memory).
/// - The pages within the ranges of the addresses where a write occurs are always erased and ready to be programmed.
pub struct FilesRegion;

impl FilesRegion
{
	pub fn create_file<Chip: FlashMemoryChip, Spi: SpiDevice<u8>>(
		&self, mut file_metadata: FileMetadata, name: impl Into<String>, file_validator: FileMetadataValidator,
	) -> FileWriter<Chip, Spi>
	{
		let name = name.into();
		file_metadata.file_name_length = name.len() as u32;

		FileWriter {
			file_metadata,
			name: Some(name),
			cursor: 0,
			has_finished_writing: false,
			_chip_and_spi: PhantomData,
			validator: file_validator,
		}
	}

	pub fn open_file_for_read<Chip: FlashMemoryChip, Spi: SpiDevice<u8>>(
		&self, file_metadata: FileMetadata, file_validator: FileMetadataValidator,
	) -> FileReader<Chip, Spi>
	{
		FileReader {
			file_metadata,
			cursor: 0,
			validator: file_validator,
			_chip_and_spi: PhantomData,
		}
	}

	pub fn delete_file<Chip: FlashMemoryChip, Spi: SpiDevice<u8>>(
		&self, file_metadata: FileMetadata, spi_flash_memory: &mut SpiFlashMemory<Chip, Spi>,
	) -> Result<(), <Spi as ErrorType>::Error>
	{
		let start_block_index = Chip::get_block_index_of_address(file_metadata.start_memory_address);
		let end_block_index = Chip::get_block_index_of_address(file_metadata.end_memory_address());
		spi_flash_memory.erase_blocks(start_block_index..=end_block_index)?;

		Ok(())
	}
}
