#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
/// Amount of [`microsteps`] the motor will require to do a full single step.
///
/// [`microsteps`]: <https://www.trinamic.com/technology/motor-control-technology/microstepping/>
pub enum MicrostepsPerStep
{
	/// `1` microstep corresponds to a step.
	FULLSTEP = 0,
	/// `2` microsteps correspond to a step.
	M2,
	/// `4` microsteps correspond to a step.
	M4,
	/// `8` microsteps correspond to a step.
	M8,
	/// `16` microsteps correspond to a step.
	M16,
	/// `32` microsteps correspond to a step.
	M32,
	/// `64` microsteps correspond to a step.
	M64,
	/// `128` microsteps correspond to a step.
	M128,
	/// `256` microsteps correspond to a step.
	M256,
}

impl MicrostepsPerStep
{
	/// Maximum microstep resolution achievable by the TMC2209 chip.
	/// ```
	/// # use firmware_core::printer::components::drivers::stepper_motor::tmc2209::*;
	/// #
	/// assert_eq!(MicrostepsPerStep::MAX_RESOLUTION, MicrostepsPerStep::M256);
	/// ```
	pub const MAX_RESOLUTION: Self = Self::M256;

	pub const fn as_exponent_of_2(&self) -> u8
	{
		*self as u8
	}

	/// Returns the number of microsteps per step this instance represents.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::printer::components::drivers::stepper_motor::tmc2209::*;
	/// #
	/// assert_eq!(MicrostepsPerStep::FULLSTEP.as_value(), 1);
	/// assert_eq!(MicrostepsPerStep::M2.as_value(), 2);
	/// assert_eq!(MicrostepsPerStep::M4.as_value(), 4);
	/// assert_eq!(MicrostepsPerStep::M8.as_value(), 8);
	/// assert_eq!(MicrostepsPerStep::M16.as_value(), 16);
	/// assert_eq!(MicrostepsPerStep::M32.as_value(), 32);
	/// assert_eq!(MicrostepsPerStep::M64.as_value(), 64);
	/// assert_eq!(MicrostepsPerStep::M128.as_value(), 128);
	/// assert_eq!(MicrostepsPerStep::M256.as_value(), 256);
	/// ```
	pub const fn as_value(&self) -> u16
	{
		1 << (self.as_exponent_of_2() as u16)
	}

	/// Returns the number of microsteps with the highest resolution this instance represents.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::printer::components::drivers::stepper_motor::tmc2209::*;
	/// #
	/// assert_eq!(MicrostepsPerStep::FULLSTEP.as_max_resolution_microsteps_count(), 256);
	/// assert_eq!(MicrostepsPerStep::M2.as_max_resolution_microsteps_count(), 128);
	/// assert_eq!(MicrostepsPerStep::M4.as_max_resolution_microsteps_count(), 64);
	/// assert_eq!(MicrostepsPerStep::M8.as_max_resolution_microsteps_count(), 32);
	/// assert_eq!(MicrostepsPerStep::M16.as_max_resolution_microsteps_count(), 16);
	/// assert_eq!(MicrostepsPerStep::M32.as_max_resolution_microsteps_count(), 8);
	/// assert_eq!(MicrostepsPerStep::M64.as_max_resolution_microsteps_count(), 4);
	/// assert_eq!(MicrostepsPerStep::M128.as_max_resolution_microsteps_count(), 2);
	/// assert_eq!(MicrostepsPerStep::M256.as_max_resolution_microsteps_count(), 1);
	/// ```
	pub const fn as_max_resolution_microsteps_count(&self) -> u16
	{
		Self::MAX_RESOLUTION.as_value() / self.as_value()
	}

	pub(super) fn as_value_of_register(&self) -> u8
	{
		Self::MAX_RESOLUTION as u8 - *self as u8
	}

	pub(super) fn from_value_of_register(register: u8) -> Option<Self>
	{
		if register == 0b0000_0000
		{
			Some(MicrostepsPerStep::M256)
		}
		else if register == 0b0000_0001
		{
			Some(MicrostepsPerStep::M128)
		}
		else if register == 0b0000_0010
		{
			Some(MicrostepsPerStep::M64)
		}
		else if register == 0b0000_0011
		{
			Some(MicrostepsPerStep::M32)
		}
		else if register == 0b0000_0100
		{
			Some(MicrostepsPerStep::M16)
		}
		else if register == 0b0000_0101
		{
			Some(MicrostepsPerStep::M8)
		}
		else if register == 0b0000_0110
		{
			Some(MicrostepsPerStep::M4)
		}
		else if register == 0b0000_0111
		{
			Some(MicrostepsPerStep::M2)
		}
		else if register == 0b0000_1000
		{
			Some(MicrostepsPerStep::FULLSTEP)
		}
		else
		{
			None
		}
	}
}
