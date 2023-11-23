use preferences::{AppInfo, Preferences};

const APP_INFO: AppInfo = AppInfo {
	name: "firmware-tools",
	author: "Angelo Cipriani",
};

/// A preference saved in the computer (it can be used in multiple runs of the program).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Preference
{
	FlashOtaKey,
}

impl Preference
{
	/// Saves the `value` of this preference in the computer.
	pub fn save(&self, value: impl Preferences) -> Result<(), preferences::PreferencesError>
	{
		value.save(&APP_INFO, self.as_key())
	}

	/// Loads the `value` of this preference you provided the last time you called [`Self::save`].
	pub fn load<P: Preferences>(&self) -> Result<P, preferences::PreferencesError>
	{
		P::load(&APP_INFO, self.as_key())
	}

	const fn as_key(&self) -> &str
	{
		match self
		{
			Preference::FlashOtaKey => "flash/ota/key",
		}
	}
}
