use super::id::FileId;
use crate::utils::slice_to_array;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct FileMetadata
{
	pub id: FileId,
	pub start_memory_address: u32,
	pub file_name_length: u32,
	pub file_data_length: u32,
}

impl FileMetadata
{
	pub fn to_bytes(&self) -> [u8; core::mem::size_of::<Self>()]
	{
		let mut bytes = self
			.id
			.to_bytes()
			.into_iter()
			.chain(self.start_memory_address.to_le_bytes())
			.chain(self.file_name_length.to_le_bytes())
			.chain(self.file_data_length.to_le_bytes());

		std::array::from_fn(|_| bytes.next().unwrap())
	}

	/// # Examples
	/// ```
	/// # use firmware_core::printer::components::file_system::regions::metadata::*;
	/// #
	/// let file_metadata = FileMetadata
	/// {
	///     id: FileId::FIRST,
	///     start_memory_address: 0x3000,
	///     file_name_length: 20,
	///     file_data_length: 500_000
	/// };
	///
	/// assert_eq!(file_metadata, FileMetadata::from_bytes(&file_metadata.to_bytes()));
	/// ```
	pub fn from_bytes(bytes: &[u8]) -> Self
	{
		Self {
			id: FileId::from_bytes(slice_to_array(&bytes[0..])),
			start_memory_address: u32::from_le_bytes(slice_to_array(&bytes[4..])),
			file_name_length: u32::from_le_bytes(slice_to_array(&bytes[8..])),
			file_data_length: u32::from_le_bytes(slice_to_array(&bytes[12..])),
		}
	}

	pub fn end_memory_address(&self) -> u32
	{
		self.start_memory_address + self.file_name_length + self.file_data_length
	}
}
