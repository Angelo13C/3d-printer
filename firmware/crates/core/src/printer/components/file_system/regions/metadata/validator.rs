use super::FilesMetadatasRegion;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FileMetadataValidator(u32);

impl FileMetadataValidator
{
	pub const fn validate(&self, files_metadata_region: &FilesMetadatasRegion) -> bool
	{
		self.0 >= files_metadata_region.metadata_validator_master.0
	}
}

pub(super) struct FileMetadataValidatorMaster(u32);
impl FileMetadataValidatorMaster
{
	pub const fn new() -> Self
	{
		Self(0)
	}

	pub const fn create_validator(&self) -> FileMetadataValidator
	{
		FileMetadataValidator(self.0)
	}

	pub fn invalidate_all_the_instances(&mut self)
	{
		self.0 += 1;
	}
}
