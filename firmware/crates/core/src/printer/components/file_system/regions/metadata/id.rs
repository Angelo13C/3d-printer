#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Debug)]
/// Unique identifier of a file for a 3D printer.
/// 2 files on the same printer can never have the same [`FileId`], even after one of them has been deleted (the value of 
/// FileIds is never reused).
pub struct FileId(u32);

impl FileId
{
	/// [`FileId`] of the first file ever created in the file system.
	pub const FIRST: Self = Self(0);

	/// [`FileId`] of a file that is currently being written. A newly created file is assigned this ID instead of
	/// a "normal one" until all its content has been written. This is because if for example the printer loses
	/// power while writing to a file, the file is invalid and all of the blocks occupied by it should be erased.
	pub const WRITING_FILE: Self = Self(u32::MAX);

	pub const fn next(other: Self) -> Self
	{
		Self(other.0 + 1)
	}

	pub const fn from_bytes(bytes: [u8; 4]) -> Self
	{
		Self(u32::from_le_bytes(bytes))
	}

	pub const fn to_bytes(&self) -> [u8; 4]
	{
		self.0.to_le_bytes()
	}
}
