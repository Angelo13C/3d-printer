use std::fmt::Debug;

use embedded_hal::spi::{ErrorType, SpiDevice};

use self::regions::{
	data::{FileReader, FileWriter, FilesRegion},
	metadata::{FileDoesntExist, FileId, FileMetadata, FilesMetadatasRegion, NotEnoughSpaceAvailable},
	RegionsConfig,
};
use super::drivers::spi_flash_memory::{FlashMemoryChip, SpiFlashMemory};

pub mod bad_blocks;
pub mod regions;

pub struct FileSystem<Chip: FlashMemoryChip, Spi: SpiDevice<u8>>
{
	spi_flash_memory: SpiFlashMemory<Chip, Spi>,

	regions_config: RegionsConfig,

	metadatas_region: FilesMetadatasRegion,
	files_region: FilesRegion,
}

impl<Chip: FlashMemoryChip, Spi: SpiDevice<u8>> FileSystem<Chip, Spi>
{
	pub fn new(
		mut spi_flash_memory: SpiFlashMemory<Chip, Spi>, regions_config: RegionsConfig,
	) -> Result<Self, CreationError<Spi>>
	{
		Chip::initialize(&mut spi_flash_memory).map_err(CreationError::InitializeChip)?;

		let metadatas_region = FilesMetadatasRegion::read_from_flash(&mut spi_flash_memory, &regions_config)
			.map_err(CreationError::MetadatasRegion)?;
		let files_region = FilesRegion;

		Ok(Self {
			spi_flash_memory,
			regions_config,
			metadatas_region,
			files_region,
		})
	}

	/// Returns `true` if you [`created`] a file before that had the provided `file_id` and you didn't [`delete`] it.
	/// Otherwise returns `false`.
	///
	/// [`created`]: Self::create_file
	/// [`delete`]: Self::delete_file
	pub fn does_file_exist(&self, file_id: FileId) -> bool
	{
		self.metadatas_region.does_file_exist(file_id)
	}

	/// Returns a slice of all the metadatas of all the files you [`created`] and didn't [`delete`].
	///
	/// [`created`]: Self::create_file
	/// [`delete`]: Self::delete_file
	pub fn get_existing_files_metadatas(&self) -> &[FileMetadata]
	{
		self.metadatas_region.get_files_metadatas()
	}

	/// Opens the file with the provided `file_id` so that you can read it in the future.
	///
	/// Returns `Err(FileDoesntExist)` if a file with the provided `file_id` isn't stored in the file system,
	/// otherwise returns `Ok(FileReader)` (check [`FileReader`] to understand how to read the file's content).
	pub fn read_file(&self, file_id: FileId) -> Result<FileReader<Chip, Spi>, FileDoesntExist>
	{
		let metadata = self
			.metadatas_region
			.get_file_metadata(file_id)
			.ok_or(FileDoesntExist)?;

		Ok(self
			.files_region
			.open_file_for_read(metadata, self.metadatas_region.get_file_validator()))
	}

	/// Creates a file with the name `file_name` and whose data will occupy `data_size` bytes.
	///
	/// Returns `Err(NotEnoughSpaceAvailable)` if there's not enough space in the flash memory to store a file of the
	/// the provided size. Otherwise returns `Ok(FileWriter)` (check [`FileWriter`] to understand how to write the file's content).
	pub fn create_file(
		&mut self, file_name: impl Into<String>, data_size: u32,
	) -> Result<FileWriter<Chip, Spi>, NotEnoughSpaceAvailable>
	{
		let file_name = Into::<String>::into(file_name);
		let (file_id, start_address) = self
			.metadatas_region
			.create_file::<Chip>(file_name.len() as u32 + data_size, &self.regions_config)?;

		let file_metadata = FileMetadata {
			id: file_id,
			start_memory_address: start_address,
			file_name_length: file_name.len() as u32,
			file_data_length: data_size,
		};

		Ok(self
			.files_region
			.create_file(file_metadata, file_name, self.metadatas_region.get_file_validator()))
	}

	/// Tries to delete the file with the provided `file_id` from the file system.
	///
	/// Returns `Err(DeleteFileError)` if there has been a [problem] in deleting the file (which may also make it
	/// corrupted), otherwise returns `Ok(())`.
	///
	/// [problem]: DeleteFileError
	pub fn delete_file(&mut self, file_id: FileId) -> Result<(), DeleteFileError<Spi>>
	{
		let file_metadata = self
			.metadatas_region
			.delete_file(file_id)
			.map_err(|_| DeleteFileError::FileDoesntExist)?;

		self.files_region
			.delete_file(file_metadata, &mut self.spi_flash_memory)
			.map_err(DeleteFileError::CantDeleteFile)?;

		self.metadatas_region
			.store_in_flash(&mut self.spi_flash_memory, &self.regions_config)
			.map_err(DeleteFileError::CantDeleteFileMetadata)?;

		Ok(())
	}
}

/// An error returned from [`FileSystem::delete_file`].
pub enum DeleteFileError<Spi: SpiDevice<u8>>
{
	/// The file you tried to delete doesn't exist in the file system.
	FileDoesntExist,
	/// It has been impossible to erase the data portion of the file from the flash memory.
	CantDeleteFile(<Spi as ErrorType>::Error),
	/// It has been impossible to erase the metadata portion of the file from the flash memory.
	CantDeleteFileMetadata(<Spi as ErrorType>::Error),
}

/// An error returned from [`FileSystem::new`].
pub enum CreationError<Spi: SpiDevice<u8>>
{
	InitializeChip(<Spi as ErrorType>::Error),
	MetadatasRegion(<Spi as ErrorType>::Error),
}

impl<Spi: SpiDevice<u8>> Debug for CreationError<Spi>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Self::MetadatasRegion(arg0) => f.debug_tuple("MetadatasRegion").field(arg0).finish(),
			CreationError::InitializeChip(arg0) => f.debug_tuple("InitializeChip").field(arg0).finish(),
		}
	}
}
