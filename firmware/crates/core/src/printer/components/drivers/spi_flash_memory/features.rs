#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
/// The register of a feature in the [`MT29F2G01ABAGDWB`] chip.
///
/// [`MT29F2G01ABAGDWB`]: super::MT29F2G01ABAGDWB
pub enum FeatureRegister
{
	BlockLock,
	Configuration,
	Status,
	DieSelect,
}

impl FeatureRegister
{
	/// Returns the address of the register.
	pub fn address(&self) -> u8
	{
		match self
		{
			FeatureRegister::BlockLock => 0xA0,
			FeatureRegister::Configuration => 0xB0,
			FeatureRegister::Status => 0xC0,
			FeatureRegister::DieSelect => 0xD0,
		}
	}
}
