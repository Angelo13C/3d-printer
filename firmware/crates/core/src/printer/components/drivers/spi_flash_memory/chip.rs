use embedded_hal::spi::{ErrorType, Mode, SpiDevice, MODE_0};

use super::{address::RowAddress, SpiFlashMemory};
use crate::utils::measurement::frequency::Frequency;

/// A type that represents a [`flash memory chip`](https://en.wikipedia.org/wiki/Flash_memory).
pub trait FlashMemoryChip
{
	/// Number of LUNs contained in the chip.
	const LUNS_PER_DEVICE: u32;
	/// Number of planes contained in a LUN of the chip.
	const PLANES_PER_LUN: u32;
	/// Number of blocks contained in a plane of the chip.
	const BLOCKS_PER_PLANE: u32;
	/// Number of pages contained in a block of the chip.
	const PAGES_PER_BLOCK: u32;
	/// Size of the data area of a page.
	const PAGE_SIZE: u32;
	/// Size of the ECC area of a page.
	const PAGE_ECC_SIZE: u32;

	/// Preferred [`SPI mode`](https://en.wikipedia.org/wiki/Serial_Peripheral_Interface#Mode_numbers).
	const SPI_MODE: Mode;
	/// Max supported frequency of the SPI clock.
	const MAX_CLOCK_FREQUENCY: Frequency;

	const MANUFACTURER_ID: u8;
	const DEVICE_ID: u8;

	/// Check if the block at the provided `block_index` in the provided `spi_flash_memory` contains a bad block mark.
	///
	/// Returns `Err(...)` if there is a problem in reading from the flash memory and it has been impossible to check if
	/// the mark is there. Otherwise returns `Ok(contains_bad_block_mark)`.
	fn contains_bad_block_mark<Spi: SpiDevice<u8>>(
		block_index: u16, spi_flash_memory: &mut SpiFlashMemory<Self, Spi>,
	) -> Result<bool, <Spi as ErrorType>::Error>
	where Self: Sized;
}

/// Extra functionality provided automatically to every type that implements [`FlashMemoryChip`].
pub trait FlashMemoryChipExt: FlashMemoryChip
{
	/// Size of the data area of a block.
	const BLOCK_SIZE: u32 = Self::PAGES_PER_BLOCK * Self::PAGE_SIZE;
	/// Size of the data area of a plane.
	const PLANE_SIZE: u32 = Self::BLOCKS_PER_PLANE * Self::BLOCK_SIZE;
	/// Size of the data area of a LUN.
	const LUN_SIZE: u32 = Self::PLANES_PER_LUN * Self::PLANE_SIZE;
	/// Size of the data area of the chip.
	const MEMORY_SIZE: u32 = Self::LUNS_PER_DEVICE * Self::LUN_SIZE;

	/// Returns the memory address of the first byte of the first page of the block at the
	/// provided `block_index`.
	fn get_address_of_block_index(block_index: u16) -> u32
	{
		block_index as u32 * Self::BLOCK_SIZE
	}

	/// Returns the index of the block that contains the provided `address`.
	fn get_block_index_of_address(address: u32) -> u16
	{
		(address / Self::BLOCK_SIZE) as u16
	}
}
impl<Chip: FlashMemoryChip> FlashMemoryChipExt for Chip {}

/// 2Gbit 3.3V NAND SPI flash memory chip ([datasheet]).
///
/// [datasheet]: <https://datasheet.lcsc.com/lcsc/1912111437_Micron-Tech-MT29F2G01ABAGDWB-IT-G_C410863.pdf>
pub struct MT29F2G01ABAGDWB;
impl FlashMemoryChip for MT29F2G01ABAGDWB
{
	const LUNS_PER_DEVICE: u32 = 1;
	const PLANES_PER_LUN: u32 = 2;
	const BLOCKS_PER_PLANE: u32 = 1024;
	const PAGES_PER_BLOCK: u32 = 64;
	const PAGE_SIZE: u32 = 2048;
	const PAGE_ECC_SIZE: u32 = 128;

	const SPI_MODE: Mode = MODE_0;
	const MAX_CLOCK_FREQUENCY: Frequency = Frequency::from_hertz(133_000_000);

	const MANUFACTURER_ID: u8 = 0x2C;
	const DEVICE_ID: u8 = 0x24;

	fn contains_bad_block_mark<Spi: SpiDevice<u8>>(
		block_index: u16, spi_flash_memory: &mut SpiFlashMemory<Self, Spi>,
	) -> Result<bool, <Spi as ErrorType>::Error>
	where Self: Sized
	{
		let mut data = 0;
		let row_address = RowAddress::from_memory_address(block_index as u32 * Self::BLOCK_SIZE);
		spi_flash_memory.read_ecc(row_address, core::slice::from_mut(&mut data))?;

		Ok(data != 0)
	}
}
