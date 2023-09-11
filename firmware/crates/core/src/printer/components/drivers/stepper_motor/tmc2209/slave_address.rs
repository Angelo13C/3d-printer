#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Serial address of a [`TMC2209`] chip.
///
/// [`TMC2209`]: `super::TMC2209`
pub enum UARTAddress
{
	A0 = 0b00,
	A1 = 0b01,
	A2 = 0b10,
	A3 = 0b11,
}

impl UARTAddress
{
	/// Returns a [`UARTAddress`] from the state of the `MS1` and `MS2` pins (`true` means the pin
	/// is shorted to `Vccio`, `false` means the pin is shorted to `GND`).
	pub fn from_ms_pins_state(ms1: bool, ms2: bool) -> Self
	{
		if ms1 && ms2
		{
			Self::A3
		}
		else if !ms1 && !ms2
		{
			Self::A0
		}
		else if ms1 && !ms2
		{
			Self::A1
		}
		else
		{
			Self::A2
		}
	}
}
