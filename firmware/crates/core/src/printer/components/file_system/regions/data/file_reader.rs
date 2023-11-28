use std::{marker::PhantomData, string::FromUtf8Error, fmt::Debug};

use embedded_hal::spi::{ErrorType, SpiDevice};

use crate::printer::components::{
	drivers::spi_flash_memory::FlashMemoryChip,
	file_system::{
		regions::metadata::{FileMetadata, FileMetadataValidator},
		FileSystem,
	},
};

pub struct FileReader<Chip: FlashMemoryChip, Spi: SpiDevice<u8>>
{
	pub(super) file_metadata: FileMetadata,
	pub(super) cursor: u32,
	pub(super) validator: FileMetadataValidator,
	pub(super) _chip_and_spi: PhantomData<(Chip, Spi)>,
}

pub enum ReadError<Spi: SpiDevice<u8>>
{
	/// An error during the communication with the flash memory chip.
	Spi(<Spi as ErrorType>::Error),
	/// The file has been deleted from the file system.
	DoesntExistAnymore,
	/// All the content of the file has been read and there's nothing else to read.
	EndOfFile,
}

impl<Spi: SpiDevice<u8>> Debug for ReadError<Spi>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Self::Spi(arg0) => f.debug_tuple("Spi").field(arg0).finish(),
			Self::DoesntExistAnymore => write!(f, "DoesntExistAnymore"),
			Self::EndOfFile => write!(f, "EndOfFile"),
		}
	}
}

pub enum ReadNameError<Spi: SpiDevice<u8>>
{
	Spi(<Spi as ErrorType>::Error),
	DoesntExistAnymore,
	InvalidUtf8String(FromUtf8Error),
}

impl<Spi: SpiDevice<u8>> Debug for ReadNameError<Spi>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spi(arg0) => f.debug_tuple("Spi").field(arg0).finish(),
            Self::DoesntExistAnymore => write!(f, "DoesntExistAnymore"),
            Self::InvalidUtf8String(arg0) => f.debug_tuple("InvalidUtf8String").field(arg0).finish(),
        }
    }
}

impl<Chip: FlashMemoryChip, Spi: SpiDevice<u8>> FileReader<Chip, Spi>
{
	/// Reads `data.len()` bytes from the file at the current position. The current position will be moved after the read
	/// by `data.len()` bytes (so you can sequentially read all the bytes in the file you provided to [`FileWriter::write_data`](super::FileWriter::write_data)).
	pub fn read_data(&mut self, file_system: &mut FileSystem<Chip, Spi>, data: &mut [u8])
		-> Result<u32, ReadError<Spi>>
	{
		if !self.validate_file_still_exists(&file_system)
		{
			return Err(ReadError::DoesntExistAnymore);
		}

		if self.has_reached_end_of_file()
		{
			return Err(ReadError::EndOfFile);
		}

		let address = self.file_metadata.start_memory_address + self.file_metadata.file_name_length + self.cursor;
		let read_bytes_count = data
			.len()
			.min((self.file_metadata.file_data_length - self.cursor) as usize);
		file_system
			.spi_flash_memory
			.read(address, &mut data[..read_bytes_count])
			.map_err(ReadError::Spi)?;

		// TODO: Check if a bad block has been developed with the use of the memory and mark it as invalid in the BadBlockTable.

		self.cursor += read_bytes_count as u32;

		Ok(read_bytes_count as u32)
	}

	/// Returns `true` if you have [`read`] all the data present in the file, otherwise returns `false`.
	///
	/// [`read`]: `Self::read_data`
	pub fn has_reached_end_of_file(&self) -> bool
	{
		self.cursor == self.file_metadata.file_data_length
	}

	/// Reads the `file_name` you provided to [`FileSystem::create_file`] from the flash memory.
	pub fn read_name(&mut self, file_system: &mut FileSystem<Chip, Spi>) -> Result<String, ReadNameError<Spi>>
	{
		if !self.validate_file_still_exists(&file_system)
		{
			return Err(ReadNameError::DoesntExistAnymore);
		}

		let mut name_bytes = vec![0; self.file_metadata.file_name_length as usize];
		file_system
			.spi_flash_memory
			.read(self.file_metadata.start_memory_address, &mut name_bytes)
			.map_err(ReadNameError::Spi)?;
		let name = String::from_utf8(name_bytes).map_err(ReadNameError::InvalidUtf8String)?;

		Ok(name)
	}

	/// Returns true if the file read by this struct still exists and the memory address in the metadata is still valid.
	fn validate_file_still_exists(&self, file_system: &FileSystem<Chip, Spi>) -> bool
	{
		self.validator.validate(&file_system.metadatas_region) && file_system.does_file_exist(self.file_metadata.id)
	}
}
