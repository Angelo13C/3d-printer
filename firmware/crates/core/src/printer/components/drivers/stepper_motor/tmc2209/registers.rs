#![allow(dead_code)]
#![allow(clippy::upper_case_acronyms)]

use std::ops::{BitAndAssign, BitOrAssign, Not, RangeInclusive, ShlAssign, ShrAssign};

/// TMC2209 internal registers' values cached in this microcontroller.
pub(super) struct Registers
{
	pub gconf: u16,
	pub chopconf: u32,
	pub pwm_conf: u32,
	pub ihold_irun: u32,
}

impl Default for Registers
{
	/// Sets the values of the registers of the TMC2209 chip cached in the microcontroller to the default
	/// ones that the TMC2209 sets when it's powered on.
	fn default() -> Self
	{
		let mut self_ = Self {
			gconf: 0b01_0000_0001,
			chopconf: 0x10000053,
			pwm_conf: 0xC10D0024,
			ihold_irun: 0,
		};

		write_field_of_register(IHOLD_IRUN::IHOLD, &mut self_.ihold_irun, 16);
		write_field_of_register(IHOLD_IRUN::IRUN, &mut self_.ihold_irun, 31);
		write_field_of_register(IHOLD_IRUN::IHOLDDELAY, &mut self_.ihold_irun, 31);

		self_
	}
}

pub(super) trait Register
{
	const ADDRESS: u8;
}
pub(super) trait ReadableRegister: Register {}
pub(super) trait WriteableRegister: Register + Sized
{
	// As stated in the TMC2209 datasheet, you can add 0x80 to an address of a register to get its write address
	fn write_address() -> u8
	{
		Self::ADDRESS + 0x80
	}
}
pub(super) trait RegisterWithFields: Register + Sized
{
	fn get_bits_range(self) -> RangeInclusive<u8>;
}

pub(super) trait RegisterValue:
	Not<Output = Self> + ShlAssign<u8> + BitOrAssign<Self> + BitAndAssign<Self> + ShrAssign<u8>
where Self: Sized
{
	fn from_u32(value: u32) -> Self;
}
macro_rules! impl_register_value {
	($type: ty) => {
		impl RegisterValue for $type
		{
			fn from_u32(value: u32) -> Self
			{
				value as Self
			}
		}
	};
}
impl_register_value!(u8);
impl_register_value!(u16);
impl_register_value!(u32);
impl_register_value!(i32);

/// Write some bits to the specified field of a register
///
/// # Examples
/// ```ignore
/// # use firmware_core::printer::components::drivers::tmc2209::*;
/// #
/// let mut register_value = 0;
/// write_field_of_register(GCONF::pdn_disable, &mut register_value, 1);
///
/// // pdn_disable is the bit 6 of the GCONF register
/// assert_eq!(register_value, 1 << 6);
/// ```
pub(super) fn write_field_of_register<V: RegisterValue>(
	register_field: impl RegisterWithFields + WriteableRegister, register_value: &mut V, mut value_to_write: V,
)
{
	let register_bits = register_field.get_bits_range();
	let mut mask = V::from_u32((1 << register_bits.len()) - 1_u32);
	mask <<= *register_bits.start();
	value_to_write <<= *register_bits.start();

	*register_value &= !mask;
	*register_value |= value_to_write;
}

/// Returns the value stored in the specified field of a register
pub(super) fn read_field_of_register<V: RegisterValue>(
	register_field: impl RegisterWithFields, mut register_value: V,
) -> V
{
	let register_bits = register_field.get_bits_range();
	let mask = V::from_u32((1 << register_bits.len()) - 1_u32);
	register_value >>= *register_bits.start();
	register_value &= mask;
	register_value
}

macro_rules! impl_register_with_fields_internal {
    (core, $register_name: ident, $register_address: expr, $($register_bit: ident),*) => {
        #[allow(non_camel_case_types)]
        #[repr(u8)]
        pub enum $register_name
        {
            $($register_bit,)*
        }

        impl Register for $register_name
        {
            const ADDRESS: u8 = $register_address;
        }
    };
    (simple_range, $register_name: ident, $register_address: expr, $($register_bit: ident),*) => {
        impl RegisterWithFields for $register_name
        {
            fn get_bits_range(self) -> RangeInclusive<u8>
            {
                let start_bit = self as u8;
                start_bit..=start_bit
            }
        }
    };
    (complex_range, $register_name: ident, $register_address: expr, $($register_bit: ident = $range: expr),*) => {
        impl RegisterWithFields for $register_name
        {
            fn get_bits_range(self) -> RangeInclusive<u8>
            {
                match self
                {
                    $(Self::$register_bit => $range),*
                }
            }
        }
    };
    (read, $register_name: ident, $register_address: expr, $($register_bit: ident),*) => {
        impl ReadableRegister for $register_name
        {

        }
    };
    (write, $register_name: ident, $register_address: expr, $($register_bit: ident),*) => {
        impl WriteableRegister for $register_name
        {

        }
    };
}
macro_rules! impl_register_with_fields {
    (R, $register_name: ident, $register_address: expr, $($register_bit: ident),*) => {
        impl_register_with_fields_internal!(core, $register_name, $register_address, $($register_bit),*);
        impl_register_with_fields_internal!(simple_range, $register_name, $register_address, $($register_bit),*);
        impl_register_with_fields_internal!(read, $register_name, $register_address, $($register_bit),*);
    };
    (W, $register_name: ident, $register_address: expr, $($register_bit: ident),*) => {
        impl_register_with_fields_internal!(core, $register_name, $register_address, $($register_bit),*);
        impl_register_with_fields_internal!(simple_range, $register_name, $register_address, $($register_bit),*);
        impl_register_with_fields_internal!(write, $register_name, $register_address, $($register_bit),*);
    };
    (RW, $register_name: ident, $register_address: expr, $($register_bit: ident),*) => {
        impl_register_with_fields_internal!(core, $register_name, $register_address, $($register_bit),*);
        impl_register_with_fields_internal!(simple_range, $register_name, $register_address, $($register_bit),*);
        impl_register_with_fields_internal!(read, $register_name, $register_address, $($register_bit),*);
        impl_register_with_fields_internal!(write, $register_name, $register_address, $($register_bit),*);
    };

    (R, $register_name: ident, $register_address: expr, $($register_bit: ident = $range: expr),*) => {
        impl_register_with_fields_internal!(core, $register_name, $register_address, $($register_bit),*);
        impl_register_with_fields_internal!(complex_range, $register_name, $register_address, $($register_bit = $range),*);
        impl_register_with_fields_internal!(read, $register_name, $register_address, $($register_bit),*);
    };
    (W, $register_name: ident, $register_address: expr, $($register_bit: ident = $range: expr),*) => {
        impl_register_with_fields_internal!(core, $register_name, $register_address, $($register_bit),*);
        impl_register_with_fields_internal!(complex_range, $register_name, $register_address, $($register_bit = $range),*);
        impl_register_with_fields_internal!(write, $register_name, $register_address, $($register_bit),*);
    };
    (RW, $register_name: ident, $register_address: expr, $($register_bit: ident = $range: expr),*) => {
        impl_register_with_fields_internal!(core, $register_name, $register_address, $($register_bit),*);
        impl_register_with_fields_internal!(complex_range, $register_name, $register_address, $($register_bit = $range),*);
        impl_register_with_fields_internal!(read, $register_name, $register_address, $($register_bit),*);
        impl_register_with_fields_internal!(write, $register_name, $register_address, $($register_bit),*);
    };
}

impl_register_with_fields!(
	RW,
	GCONF,
	0x00,
	I_scale_analog,
	internal_Rsense,
	en_SpreadCycle,
	shaft,
	index_otpw,
	index_step,
	pdn_disable,
	mstep_reg_select,
	multistep_filt,
	test_mode
);
impl_register_with_fields!(RW, GSTAT, 0x01, reset, drv_err, uv_cp);

impl_register_with_fields!(W, SLAVECONF, 0x03, SENDDELAY = 8..=11);
impl_register_with_fields!(
	R,
	IOIN,
	0x06,
	ENN = 0..=0,
	MS1 = 2..=2,
	MS2 = 3..=3,
	DIAG = 4..=4,
	PDN_UART = 6..=6,
	STEP = 7..=7,
	SPREAD_EN = 8..=8,
	DIR = 9..=9,
	VERSION = 24..=31
);
impl_register_with_fields!(W, IHOLD_IRUN, 0x10, IHOLD = 0..=4, IRUN = 8..=12, IHOLDDELAY = 16..=19);

impl_register_with_fields!(
	RW,
	CHOPCONF,
	0x6C,
	TOFF = 0..=3,
	HSTRT = 4..=6,
	HEND = 7..=10,
	TBL = 15..=16,
	VSENSE = 17..=17,
	MRES = 24..=27,
	INTPOL = 28..=28,
	DEDGE = 29..=29,
	DISS2G = 30..=30,
	DISS2VS = 31..=31
);

impl_register_with_fields!(
	RW,
	PWMCONF,
	0x70,
	PWM_OFS = 0..=7,
	PWM_GRAD = 8..=15,
	PWM_FREQ0 = 16..=16,
	PWM_FREQ1 = 17..=17,
	PWM_AUTOSCALE = 18..=18,
	PWM_AUTOGRAD = 19..=19,
	FREEWHEEL = 20..=21,
	PWM_REG = 24..=27,
	PWM_LIM = 28..=31
);

macro_rules! impl_register {
	(core, $register_name: ident, $register_address: expr) => {
		#[allow(non_camel_case_types)]
		pub struct $register_name;

		impl Register for $register_name
		{
			const ADDRESS: u8 = $register_address;
		}
	};
	(R, $register_name: ident, $register_address: expr) => {
		impl_register!(core, $register_name, $register_address);

		impl ReadableRegister for $register_name {}
	};
	(W, $register_name: ident, $register_address: expr) => {
		impl_register!(core, $register_name, $register_address);

		impl WriteableRegister for $register_name {}
	};
	(RW, $register_name: ident, $register_address: expr) => {
		impl_register!(R, $register_name, $register_address);

		impl WriteableRegister for $register_name {}
	};
}

impl_register!(W, VACTUAL, 0x22);
