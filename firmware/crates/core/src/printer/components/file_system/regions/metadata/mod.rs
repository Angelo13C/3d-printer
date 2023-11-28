mod data_holes;
mod id;
mod metadata;
mod validator;

use embedded_hal::spi::{ErrorType, SpiDevice};
pub use id::*;
pub use metadata::*;
pub use validator::*;

pub(crate) use self::data_holes::*;
use super::data::FilesRegion;
use crate::{
	printer::components::{
		drivers::spi_flash_memory::{FlashMemoryChip, SpiFlashMemory},
		file_system::{bad_blocks::BadBlockTable, RegionsConfig},
	},
	utils::slice_to_array,
};

pub struct FilesMetadatasRegion
{
	files_metadatas: Vec<FileMetadata>,
	highest_used_file_id: FileId,
	writing_to_files_with_id: Vec<FileId>,
	bad_block_table: BadBlockTable,
	metadata_validator_master: FileMetadataValidatorMaster,
}

impl FilesMetadatasRegion
{
	/// Reads the metadatas region you previously [`stored`] from the provided `spi_flash_memory`
	/// at the address specified in `regions_config.metadata_block_range` or creates a default one
	/// if it's the first time the flash memory is used.
	///
	/// Returns `Ok(Self)` if the region was correctly read or if it was missing and the default
	/// one was succesfully created and automatically stored in the chip. Otherwise returns
	/// `Err(...)` (if there has been an error in communicating with the `spi_flash_memory`).
	///
	/// [`stored`]: `Self::store_in_flash`
	pub fn read_from_flash<Chip: FlashMemoryChip, Spi: SpiDevice<u8>>(
		spi_flash_memory: &mut SpiFlashMemory<Chip, Spi>, regions_config: &RegionsConfig,
	) -> Result<Self, <Spi as ErrorType>::Error>
	{
		let mut needs_to_store_in_flash = false;

		let address_offset = *regions_config.metadata_address_range::<Chip>().start();

		const READ_DATA_SIZE: usize = 2048;
		assert_eq!(READ_DATA_SIZE, Chip::PAGE_SIZE as usize);
		let mut data = ArrayIterator::new([0; READ_DATA_SIZE]);

		spi_flash_memory.read(address_offset, data.reset_and_get_slice_mut())?;

		let self_ = if data.next() == 0xFF
		{
			needs_to_store_in_flash = true;

			let bad_block_table = BadBlockTable::from_first_powerup(spi_flash_memory)?;

			Self {
				files_metadatas: Vec::with_capacity(5),
				highest_used_file_id: FileId::FIRST,
				writing_to_files_with_id: Vec::with_capacity(2),
				bad_block_table,
				metadata_validator_master: FileMetadataValidatorMaster::new(),
			}
		}
		else
		{
			let bad_blocks_size = core::mem::size_of::<u16>() * (data.next() as usize);
			let bad_block_table = BadBlockTable::from_bytes(data.take(bad_blocks_size));

			let highest_used_file_id = FileId::from_bytes(data.take_as_array());

			let files_count_stored_in_flash = u16::from_be_bytes(data.take_as_array());
			let mut files_metadatas = Vec::with_capacity(files_count_stored_in_flash as usize);
			let mut current_page_index = 0;
			let mut data_reserve = [0_u8; core::mem::size_of::<FileMetadata>()];
			let mut data_reserve_len = 0;
			for _ in 0..files_count_stored_in_flash
			{
				let bytes = if data_reserve_len == 0
				{
					data.take(core::mem::size_of::<FileMetadata>())
				}
				else
				{
					data_reserve[data_reserve_len..]
						.copy_from_slice(data.take(core::mem::size_of::<FileMetadata>() - data_reserve_len));
					&data_reserve
				};

				let file_metadata = FileMetadata::from_bytes(bytes);
				if file_metadata.id == FileId::WRITING_FILE
				{
					// Delete the corrupted files
					FilesRegion.delete_file(file_metadata, spi_flash_memory)?;
				}
				else
				{
					files_metadatas.push(file_metadata);
				}

				if data.len() < core::mem::size_of::<FileMetadata>()
				{
					current_page_index += 1;

					data_reserve_len = data.len();
					data_reserve.copy_from_slice(data.take(data_reserve_len));

					spi_flash_memory.read(
						address_offset + current_page_index * Chip::PAGE_SIZE,
						data.reset_and_get_slice_mut(),
					)?;
				}
			}

			Self {
				files_metadatas,
				highest_used_file_id,
				writing_to_files_with_id: Vec::with_capacity(2),
				bad_block_table,
				metadata_validator_master: FileMetadataValidatorMaster::new(),
			}
		};

		if needs_to_store_in_flash
		{
			self_.store_in_flash(spi_flash_memory, regions_config)?;
		}

		Ok(self_)
	}

	/// Stores this region in the provided `spi_flash_memory` at the address specified in
	/// `regions_config.metadata_block_range` so that you can later recover it
	/// (even after shutting down the microcontrooler) using [`Self::read_from_flash`].
	///
	/// Returns `Err(...)` if there has been an error in communicating with the flash memory,
	/// otherwise returns `Ok(())`.
	pub fn store_in_flash<Chip: FlashMemoryChip, Spi: SpiDevice<u8>>(
		&self, spi_flash_memory: &mut SpiFlashMemory<Chip, Spi>, regions_config: &RegionsConfig,
	) -> Result<(), <Spi as ErrorType>::Error>
	{
		spi_flash_memory.erase_blocks(regions_config.metadata_block_range.clone())?;

		let self_as_bytes: Vec<u8> = core::iter::once(0_u8)
			.chain(self.highest_used_file_id.to_bytes().into_iter())
			.chain(self.bad_block_table.as_bytes())
			.chain(self.highest_used_file_id.to_bytes().into_iter())
			.chain(
				self.files_metadatas
					.iter()
					.flat_map(|file_metadata| file_metadata.to_bytes().into_iter()),
			)
			.collect();

		spi_flash_memory.program(&self_as_bytes, *regions_config.metadata_address_range::<Chip>().start())?;

		Ok(())
	}

	/// Finds a space large enough to store `data_size` bytes contiguously.
	///
	/// Returns `Ok((new_file_id, new_file_start_address))` if the space has been found,
	/// otherwise returns `Err(NotEnoughSpaceAvailable)`
	///
	/// # Warning
	/// This won't store the metadata of the newly created file in the flash memory, a call
	/// to [`Self::finish_writing_file`] is required.
	pub fn create_file<Chip: FlashMemoryChip>(
		&mut self, bytes_count: u32, regions_config: &RegionsConfig,
	) -> Result<(FileId, u32), NotEnoughSpaceAvailable>
	{
		let data_holes = DataHoles::<Chip>::from_metadatas_region(&self, regions_config);

		match data_holes.find_space_for_new_data(bytes_count)
		{
			data_holes::FreeSpace::NotAvailable => Err(NotEnoughSpaceAvailable),
			// Memory compaction is not implemented yet
			data_holes::FreeSpace::AvailableButRequiresCompacting => Err(NotEnoughSpaceAvailable),
			data_holes::FreeSpace::Available { start_address } =>
			{
				let id = self.highest_used_file_id;
				self.highest_used_file_id = FileId::next(id);

				Ok((id, start_address))
			},
		}
	}

	pub fn start_writing_file<Chip: FlashMemoryChip, Spi: SpiDevice<u8>>(
		&mut self, file_metadata: FileMetadata, spi_flash_memory: &mut SpiFlashMemory<Chip, Spi>,
		regions_config: &RegionsConfig,
	) -> Result<(), <Spi as ErrorType>::Error>
	{
		self.writing_to_files_with_id.push(file_metadata.id);

		self.files_metadatas.push(file_metadata);
		self.store_in_flash(spi_flash_memory, regions_config)?;

		Ok(())
	}

	pub fn finish_writing_file<Chip: FlashMemoryChip, Spi: SpiDevice<u8>>(
		&mut self, file_id: FileId, spi_flash_memory: &mut SpiFlashMemory<Chip, Spi>, regions_config: &RegionsConfig,
	) -> Result<(), <Spi as ErrorType>::Error>
	{
		if let Some(position) = self
			.writing_to_files_with_id
			.iter()
			.position(|&writing_file_id| writing_file_id == file_id)
		{
			self.writing_to_files_with_id.swap_remove(position);

			self.store_in_flash(spi_flash_memory, regions_config)?;
		}

		Ok(())
	}

	/// If a file with the specified `file_id` is stored in this region, returns its metadata
	/// wrapped in `Some`. Otherwise returns `None`.
	pub fn get_file_metadata(&self, file_id: FileId) -> Option<FileMetadata>
	{
		self.files_metadatas
			.iter()
			.find(|file_metadata| file_metadata.id == file_id)
			.cloned()
	}

	/// Returns `true` if a file with the specified `file_id` is stored in this region,
	/// otherwise returns `false`.
	pub fn does_file_exist(&self, file_id: FileId) -> bool
	{
		self.files_metadatas
			.iter()
			.any(|file_metadata| file_metadata.id == file_id)
	}

	/// Removes the file with the specified `file_id` from the region.
	///
	/// # Warning
	/// A call to `Self::store_in_flash` is required after calling this method to actually
	/// store the modified region in the flash memory.
	pub fn delete_file(&mut self, file_id: FileId) -> Result<FileMetadata, FileDoesntExist>
	{
		if let Some(index_to_remove) = self
			.files_metadatas
			.iter()
			.position(|file_metadata| file_metadata.id == file_id)
		{
			let file_metadata = self.files_metadatas[index_to_remove].clone();
			self.files_metadatas.swap_remove(index_to_remove);

			Ok(file_metadata)
		}
		else
		{
			Err(FileDoesntExist)
		}
	}

	/// Returns a reference to the metadatas of all the files stored in this region.
	pub fn get_files_metadatas(&self) -> &[FileMetadata]
	{
		&self.files_metadatas
	}

	/// Returns the current file validator.
	pub fn get_file_validator(&self) -> FileMetadataValidator
	{
		self.metadata_validator_master.create_validator()
	}

	#[allow(dead_code, unused_variables)]
	/// Collect all the fragmented data holes present in the memory in one big chunk, moving all the files
	/// next to each other.
	///
	/// Check [`this`] for more info.
	///
	/// [`this`]: <https://www.geeksforgeeks.org/compaction-in-operating-system/>
	pub fn compact<Chip: FlashMemoryChip, Spi: SpiDevice<u8>>(
		&mut self, spi_flash_memory: &mut SpiFlashMemory<Chip, Spi>,
	) -> Result<(), <Spi as ErrorType>::Error>
	{
		self.metadata_validator_master.invalidate_all_the_instances();

		todo!();
	}
}

pub struct FileDoesntExist;
pub struct NotEnoughSpaceAvailable;

struct ArrayIterator<const N: usize, T>
{
	array: [T; N],
	current_element: usize,
}
impl<const N: usize, T> ArrayIterator<N, T>
{
	fn new(array: [T; N]) -> Self
	{
		Self {
			array,
			current_element: 0,
		}
	}

	fn next(&mut self) -> T
	where T: Clone
	{
		self.take(1)[0].clone()
	}

	fn take(&mut self, n: usize) -> &[T]
	{
		self.current_element += n;

		&self.array[(self.current_element - n)..self.current_element]
	}

	fn take_as_array<const A: usize>(&mut self) -> [T; A]
	where T: Copy
	{
		slice_to_array(self.take(A))
	}

	fn len(&self) -> usize
	{
		N - self.current_element
	}

	fn reset_and_get_slice_mut(&mut self) -> &mut [T; N]
	{
		self.current_element = 0;
		&mut self.array
	}
}
