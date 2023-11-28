use std::ops::{Range, RangeInclusive};

use embedded_hal::spi::{ErrorType, SpiDevice};

use self::{
	address::{ColumnAddress, RowAddress},
	commands::Command,
};
use crate::utils::math::NumberExt;

mod address;
mod chip;
mod commands;
mod features;

pub use chip::*;
pub use features::*;

/// A flash memory connected to the microcontroller through a SPI interface.
///
/// # Warning
/// It's really important to understand how a [flash memory works] before you use any of the method in this struct,
/// or you might have unexpected results and/or wear out the memory.
///
/// [flash memory works]: <https://flashdba.com/2014/06/20/understanding-flash-blocks-pages-and-program-erases/>
pub struct SpiFlashMemory<Chip: FlashMemoryChip, Spi: SpiDevice<u8>>
{
	spi: Spi,
	chip: Chip,
}

impl<Chip: FlashMemoryChip, Spi: SpiDevice<u8>> SpiFlashMemory<Chip, Spi>
{
	pub fn new(spi: Spi, chip: Chip) -> Self
	{
		Self { spi, chip }
	}

	/// Program the provided `data` in the flash memory starting from the provided `address`,
	/// writing to `data.len() / Chip::PAGE_SIZE` sequential pages.
	///
	/// Returns `Ok(())` if all the bytes have been written successfully, otherwise returns `Err(...)`.
	///
	/// # Note
	/// The program will only affect the data area of a page, not the ECC one.
	///
	/// # Warning
	/// You can't update some data in the memory you programmed before without erasing it first.
	///
	/// Check [`struct's documentation`](Self#warning).
	pub fn program(&mut self, data: &[u8], address: u32) -> Result<(), <Spi as ErrorType>::Error>
	{
		if data.len() == 0
		{
			return Ok(());
		}

		Command::<Chip>::WriteEnable.execute(&mut self.spi)?;

		Self::cycle_pages(address, data.len() as u32, |parameters| {
			Command::ProgramLoadRandomData::<Chip> {
				column_address: parameters.column_address,
				input: &data[parameters.data_range.clone()],
			}
			.execute(&mut self.spi)?;

			self.wait_for_operation_to_finish()?;

			Command::ProgramExecute::<Chip> {
				row_address: parameters.row_address,
			}
			.execute(&mut self.spi)?;

			Ok(())
		})?;

		Command::<Chip>::WriteDisable.execute(&mut self.spi)?;

		Ok(())
	}

	/// Reads [`data.len()`] bytes from the data areas of the pages starting from the specified `address`.
	///
	/// Returns `Ok(())` if all the bytes have been read successfully, otherwise returns `Err(...)`.
	///
	/// # Note
	/// The read will only affect the data area of a page, not the ECC one.
	pub fn read(&mut self, address: u32, data: &mut [u8]) -> Result<(), <Spi as ErrorType>::Error>
	{
		if data.len() == 0
		{
			return Ok(());
		}

		Self::cycle_pages(address, data.len() as u32, |parameters| {
			self.read_internal(
				parameters.row_address,
				parameters.column_address,
				&mut data[parameters.data_range],
			)
		})?;

		Ok(())
	}

	/// Reads [`data.len()`] bytes from the ECC area of the page identified by the provided `row_address`.
	///
	/// Returns `Ok(())` if all the bytes have been read successfully, otherwise returns `Err(...)`.
	///
	/// # Note
	/// The read will only affect the ECC area of a page, not the data one.
	///
	/// # Panics
	/// Panics if `data.len() > Chip::PAGE_ECC_SIZE`.
	pub fn read_ecc(&mut self, row_address: RowAddress<Chip>, data: &mut [u8])
		-> Result<(), <Spi as ErrorType>::Error>
	{
		assert!(data.len() as u32 <= Chip::PAGE_ECC_SIZE);

		self.read_internal(
			row_address,
			ColumnAddress::new(Chip::PAGE_SIZE as u16, row_address.get_plane_index()),
			data,
		)
	}

	/// Moves the bytes in the `from` address range to the address range of the same size that starts at
	/// `to_start_address`.
	///
	/// It's an internal move because bytes aren't copied in the microcontroller and then programmed
	/// at the right location, but everything is done in the flash memory chip.
	///
	/// # Warning
	/// Check [`struct's documentation`](Self#warning).
	pub fn internal_data_move(
		&mut self, from: RangeInclusive<u32>, to_start_address: u32,
	) -> Result<(), <Spi as ErrorType>::Error>
	{
		Command::<Chip>::WriteEnable.execute(&mut self.spi)?;

		Self::cycle_pages(*from.start(), from.end() - from.start(), |parameters| {
			Command::PageRead::<Chip> {
				row_address: parameters.row_address,
			}
			.execute(&mut self.spi)?;

			Command::ProgramLoadRandomData::<Chip> {
				column_address: parameters.column_address,
				input: &[],
			}
			.execute(&mut self.spi)?;

			let row_address = RowAddress::from_memory_address(to_start_address + parameters.data_range.start as u32);
			Command::ProgramExecute::<Chip> { row_address }.execute(&mut self.spi)?;

			self.wait_for_operation_to_finish()?;

			Ok(())
		})?;

		Command::<Chip>::WriteDisable.execute(&mut self.spi)?;

		Ok(())
	}

	/// Puts the flash memory in a known condition.
	///
	/// # Warning
	/// Calling this function can block the microcontroller for some milliseconds.
	pub fn reset(&mut self) -> Result<(), <Spi as ErrorType>::Error>
	{
		Command::Reset::<Chip>.execute(&mut self.spi)
	}

	/// Erases all the blocks in the provided `block_indices_to_erase` range by resetting all the bits
	/// of those blocks to 1.
	///
	/// Returns `Ok(())` if all the blocks have been reset successfully, otherwise returns `Err(...)`.
	///
	/// # Warning
	/// Check [`struct's documentation`](Self#warning).
	pub fn erase_blocks(&mut self, block_indices_to_erase: RangeInclusive<u16>)
		-> Result<(), <Spi as ErrorType>::Error>
	{
		for block_index in block_indices_to_erase
		{
			Command::WriteEnable::<Chip>.execute(&mut self.spi)?;
	
			let row_address = RowAddress::from_memory_address(Chip::get_address_of_block_index(block_index));
			Command::BlockErase::<Chip> { row_address }.execute(&mut self.spi)?;

			self.wait_for_operation_to_finish()?;
		}

		Command::WriteDisable::<Chip>.execute(&mut self.spi)?;

		Ok(())
	}

	/// Validate that the IDs (`ManufacturerID` and `DeviceID`) of the connected flash memory chip are
	/// the same as the ones written in the [`FlashMemoryChip`] trait.
	///
	/// Returns `Ok(())` if the microcontroller was able to communicate with the flash memory and
	/// the returned IDs are correct, otherwise returns `Err(ValidateIdError)`.
	pub fn validate_id(&mut self) -> Result<(), ValidateIdError<Spi>>
	{
		let mut manufacturer_id = 0;
		let mut device_id = 0;
		Command::ReadId::<Chip> {
			manufacturer_id: &mut manufacturer_id,
			device_id: &mut device_id,
		}
		.execute(&mut self.spi)
		.map_err(ValidateIdError::CoudlntReadSpi)?;

		match (manufacturer_id == Chip::MANUFACTURER_ID, device_id == Chip::DEVICE_ID)
		{
			(true, true) => Ok(()),
			(true, false) => Err(ValidateIdError::DeviceIdDoesntMatch),
			(false, true) => Err(ValidateIdError::ManufacturerIdDoesntMatch),
			(false, false) => Err(ValidateIdError::BothIdsDontMatch),
		}
	}

	/// Returns the value stored in the provided `features` register of the flash memory chip.
	///
	/// Check the datasheet of the chip to understand what each feature does.
	pub fn get_features(&mut self, features: FeatureRegister) -> Result<u8, <Spi as ErrorType>::Error>
	{
		let mut value = 0;
		Command::GetFeatures::<Chip> {
			features_address: features.address(),
			features_value: &mut value,
		}
		.execute(&mut self.spi)?;

		Ok(value)
	}

	/// Sets the value stored in the provided `features` register of the flash memory chip to `features_value`.
	///
	/// Check the datasheet of the chip to understand what each feature does.
	pub fn set_features(
		&mut self, features: FeatureRegister, features_value: u8,
	) -> Result<(), <Spi as ErrorType>::Error>
	{
		Command::SetFeatures::<Chip> {
			features_address: features.address(),
			features_value,
		}
		.execute(&mut self.spi)?;

		Ok(())
	}

	/// Returns a mutable reference to the underlying chip.
	pub fn get_chip_mut(&mut self) -> &mut Chip
	{
		&mut self.chip
	}

	fn is_operation_in_progress(&mut self) -> Result<bool, <Spi as ErrorType>::Error>
	{
		let status = self.get_features(FeatureRegister::Status)?;
		let is_in_progress = (status & 0b0000_0001) == 1;
		Ok(is_in_progress)
	}

	fn wait_for_operation_to_finish(&mut self) -> Result<(), <Spi as ErrorType>::Error>
	{
		while self.is_operation_in_progress()?
		{}
		Ok(())
	}

	fn read_internal(
		&mut self, row_address: RowAddress<Chip>, column_address: ColumnAddress, output: &mut [u8],
	) -> Result<(), <Spi as ErrorType>::Error>
	{
		Command::PageRead::<Chip> { row_address }.execute(&mut self.spi)?;

		self.wait_for_operation_to_finish()?;

		Command::ReadFromCache::<Chip> { column_address, output }.execute(&mut self.spi)?;

		Ok(())
	}

	/// Check the test module below for some examples.
	fn cycle_pages(
		address: u32, data_length: u32,
		mut callback: impl FnMut(CyclePageParameters<Chip>) -> Result<(), <Spi as ErrorType>::Error>,
	) -> Result<(), <Spi as ErrorType>::Error>
	{
		let start_page_index = address / Chip::PAGE_SIZE;
		let mut column_address = (address - start_page_index * Chip::PAGE_SIZE) as u16;
		let mut data_range_length = Chip::PAGE_SIZE as usize - column_address as usize;
		let loops_to_do = data_length.ceil_div(Chip::PAGE_SIZE);
		for i in 0..loops_to_do
		{
			let row_address = RowAddress::<Chip>::from_page_index(start_page_index + i);
			let plane_index = row_address.get_plane_index();
			let data_range_start = (i * Chip::PAGE_SIZE) as usize;

			if i == loops_to_do - 1
			{
				data_range_length = data_length as usize - data_range_start;
			}

			(callback)(CyclePageParameters {
				data_range: data_range_start..(data_range_start + data_range_length),
				column_address: ColumnAddress::new(column_address, plane_index),
				row_address,
			})?;

			column_address = 0;
			data_range_length = Chip::PAGE_SIZE as usize;
		}

		Ok(())
	}
}

struct CyclePageParameters<Chip: FlashMemoryChip>
{
	data_range: Range<usize>,
	row_address: RowAddress<Chip>,
	column_address: ColumnAddress,
}

/// An error returned from [`SpiFlashMemory::validate_id`].
pub enum ValidateIdError<Spi: SpiDevice<u8>>
{
	/// Only the `Manufacturer ID` read from the SPI device doesn't match the one assigned in the
	/// [`FlashMemoryChip`] trait.
	ManufacturerIdDoesntMatch,
	/// Only the `Device ID` read from the SPI device doesn't match the one assigned in the
	/// [`FlashMemoryChip`] trait.
	DeviceIdDoesntMatch,
	/// Both the `Manufacturer ID` and the `Device ID` read from the SPI device don't match the ones
	/// assigned in the [`FlashMemoryChip`] trait.
	BothIdsDontMatch,
	/// It has been impossible to communicate via SPI and so no validation has been performed.
	CoudlntReadSpi(<Spi as ErrorType>::Error),
}

#[cfg(test)]
mod tests
{
	use super::{chip::MT29F2G01ABAGDWB, *};
	use crate::printer::components::mock::{MockError, MockSpi};

	#[test]
	fn cycle_zero_pages()
	{
		for address in 0..10_000
		{
			SpiFlashMemory::<MT29F2G01ABAGDWB, MockSpi>::cycle_pages(address, 0, |_| panic!()).unwrap();
		}
	}

	#[test]
	fn cycle_single_pages()
	{
		const LOOPS_COUNT: usize = 10_000;
		const DATA_LENGTH: u32 = MT29F2G01ABAGDWB::PAGE_SIZE;
		let mut cycled_pages = [false; LOOPS_COUNT];
		for address in 0..LOOPS_COUNT
		{
			SpiFlashMemory::<MT29F2G01ABAGDWB, MockSpi>::cycle_pages(
				DATA_LENGTH * address as u32,
				DATA_LENGTH,
				|parameters| {
					let row_address = (parameters.row_address.get_page_index()) as usize;
					if cycled_pages[row_address]
					{
						Err(MockError)
					}
					else
					{
						cycled_pages[row_address] = true;
						Ok(())
					}
				},
			)
			.unwrap();
		}
	}
}
