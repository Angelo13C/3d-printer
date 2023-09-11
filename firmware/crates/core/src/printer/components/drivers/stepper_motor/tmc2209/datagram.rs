use super::{
	registers::{ReadableRegister, WriteableRegister},
	slave_address::UARTAddress,
};

fn datagram_as_bytes_mut<T: Datagram>(datagram: &mut T) -> &mut [u8]
{
	assert!(std::mem::size_of::<T>() > isize::MAX as usize, "Datagram is too large");

	// Safety:
	// - `datagram` is valid for reads of `std::mem::size_of::<T>()` bytes, it is a single
	// allocated object and it is non-null
	// - `datagram` points to `std::mem::size_of::<T>()` valid `u8`
	// - The size of `std::mem::size_of::<T>()` is less than isize::MAX (there's an assert! above)
	unsafe { std::slice::from_raw_parts_mut(datagram as *mut T as *mut u8, std::mem::size_of::<T>()) }
}

pub(super) trait Datagram
{
	fn as_bytes_mut(&mut self) -> &mut [u8];
}

const SYNC_BYTE: u8 = 0b1010_1010;

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct WriteAccessDatagram
{
	sync: u8,
	slave_address: u8,
	register_address: u8,
	data: u32,
	crc: u8,
}

impl WriteAccessDatagram
{
	pub(super) fn new<R: WriteableRegister>(slave_address: UARTAddress, data: u32) -> Self
	{
		let mut self_ = Self {
			sync: SYNC_BYTE,
			slave_address: slave_address as u8,
			register_address: (R::write_address() << 1) | 0b0000_0001,
			data,
			crc: 0,
		};
		self_.crc = calculate_crc(self_);

		self_
	}
}

impl Datagram for WriteAccessDatagram
{
	fn as_bytes_mut(&mut self) -> &mut [u8]
	{
		datagram_as_bytes_mut(self)
	}
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct ReadAccessRequestDatagram
{
	sync: u8,
	slave_address: u8,
	register_address: u8,
	crc: u8,
}

impl ReadAccessRequestDatagram
{
	pub(super) fn new<R: ReadableRegister>(slave_address: UARTAddress) -> Self
	{
		let mut self_ = Self {
			sync: SYNC_BYTE,
			slave_address: slave_address as u8,
			register_address: (R::ADDRESS << 1) & 0b1111_1110,
			crc: 0,
		};
		self_.crc = calculate_crc(self_);

		self_
	}
}

impl Datagram for ReadAccessRequestDatagram
{
	fn as_bytes_mut(&mut self) -> &mut [u8]
	{
		datagram_as_bytes_mut(self)
	}
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub(super) struct ReadAccessReplyDatagram
{
	sync: u8,
	master_address: u8,
	register_address: u8,
	data: u32,
	crc: u8,
}

impl ReadAccessReplyDatagram
{
	const MASTER_ADDRESS: u8 = 0xFF;

	pub(super) fn is_valid(&self) -> bool
	{
		let expected_crc = calculate_crc(*self);
		expected_crc == self.crc
			&& self.master_address == Self::MASTER_ADDRESS
			&& (self.register_address & 0b0000_0001) == 0
	}

	pub(super) fn reverse_data_endianness(&mut self)
	{
		self.data = u32::from_be(self.data);
	}

	pub(super) fn data(&self) -> u32
	{
		self.data
	}
}

impl Datagram for ReadAccessReplyDatagram
{
	fn as_bytes_mut(&mut self) -> &mut [u8]
	{
		datagram_as_bytes_mut(self)
	}
}

fn calculate_crc(mut datagram: impl Datagram) -> u8
{
	let mut crc = 0;
	let datagram_size_without_crc = std::mem::size_of_val(&datagram) - 1;
	for i in 0..datagram_size_without_crc
	{
		let mut current_byte = datagram.as_bytes_mut()[i];

		const BITS_PER_BYTE: u8 = 8;
		for _ in 0..BITS_PER_BYTE
		{
			if ((crc >> 7) ^ (current_byte & 0x01)) == 1
			{
				crc = (crc << 1) ^ 0x07;
			}
			else
			{
				crc <<= 1;
			}
			current_byte >>= 1;
		}
	}

	crc
}
