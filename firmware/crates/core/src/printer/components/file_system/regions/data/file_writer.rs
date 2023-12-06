use std::marker::PhantomData;

use embedded_hal::spi::{ErrorType, SpiDevice};

use crate::printer::components::{
	drivers::spi_flash_memory::FlashMemoryChip,
	file_system::{
		regions::metadata::{FileId, FileMetadata, FileMetadataValidator},
		FileSystem,
	},
};

pub struct FileWriter<Chip: FlashMemoryChip, Spi: SpiDevice<u8>>
{
	pub(super) file_metadata: FileMetadata,
	pub(super) name: Option<String>,
	pub(super) cursor: u32,
	pub(super) has_finished_writing: bool,
	pub(super) validator: FileMetadataValidator,
	pub(super) _chip_and_spi: PhantomData<(Chip, Spi)>,
}

impl<Chip: FlashMemoryChip, Spi: SpiDevice<u8>> FileWriter<Chip, Spi>
{
	pub fn write_data(&mut self, file_system: &mut FileSystem<Chip, Spi>, data: &[u8]) -> Result<(), WriteError<Spi>>
	{
		// If it's the first time you try to write to this file
		if let Some(name) = self.name.take()
		{
			log::info!(
				"Start writing a file with name \"{name}\" and with a size of {} bytes",
				self.file_metadata.file_data_length
			);
			file_system
				.metadatas_region
				.start_writing_file(
					self.file_metadata.clone(),
					&mut file_system.spi_flash_memory,
					&file_system.regions_config,
				)
				.map_err(WriteError::Spi)?;

			file_system
				.spi_flash_memory
				.program(name.as_bytes(), self.file_metadata.start_memory_address)
				.map_err(WriteError::Spi)?;
		}

		file_system
			.spi_flash_memory
			.program(
				data,
				self.file_metadata.start_memory_address + self.file_metadata.file_name_length + self.cursor,
			)
			.map_err(WriteError::Spi)?;

		// TODO: Check if a bad block has been developed with the use of the memory and mark it as invalid in the BadBlockTable.

		self.cursor += data.len() as u32;

		Ok(())
	}

	pub fn finish_writing(mut self, file_system: &mut FileSystem<Chip, Spi>) -> Result<(), WriteError<Spi>>
	{
		self.validate_file_still_exists(file_system)?;

		file_system
			.metadatas_region
			.finish_writing_file(
				self.file_metadata.id,
				&mut file_system.spi_flash_memory,
				&file_system.regions_config,
			)
			.map_err(WriteError::Spi)?;
		self.has_finished_writing = true;

		Ok(())
	}

	fn validate_file_still_exists(&self, file_system: &FileSystem<Chip, Spi>) -> Result<(), WriteError<Spi>>
	{
		if self.validator.validate(&file_system.metadatas_region) && !file_system.does_file_exist(self.file_metadata.id)
		{
			Err(WriteError::DoesntExistAnymore)
		}
		else
		{
			Ok(())
		}
	}
}

pub enum WriteError<Spi: SpiDevice<u8>>
{
	Spi(<Spi as ErrorType>::Error),
	DoesntExistAnymore,
}

impl<Spi: SpiDevice<u8>> std::fmt::Debug for WriteError<Spi>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Self::Spi(arg0) => f.debug_tuple("Spi").field(arg0).finish(),
			Self::DoesntExistAnymore => write!(f, "DoesntExistAnymore"),
		}
	}
}

impl<Chip: FlashMemoryChip, Spi: SpiDevice<u8>> Drop for FileWriter<Chip, Spi>
{
	fn drop(&mut self)
	{
		if !self.has_finished_writing
		{
			panic!("FileWriter instance dropped without calling FileWriter::finish_writing() first");
		}
	}
}
