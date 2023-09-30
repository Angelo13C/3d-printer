use embedded_hal::spi::{ErrorType, Operation, SpiDevice};

use super::{
	address::{ColumnAddress, RowAddress},
	chip::FlashMemoryChip,
};

#[derive(Debug)]
/// A command to send to a flash memory over SPI.
pub enum Command<'a, Chip: FlashMemoryChip>
{
	Reset,
	WriteEnable,
	WriteDisable,
	PageRead
	{
		row_address: RowAddress<Chip>,
	},
	ReadFromCache
	{
		column_address: ColumnAddress,
		output: &'a mut [u8],
	},
	BlockErase
	{
		row_address: RowAddress<Chip>,
	},
	ProgramLoad
	{
		column_address: ColumnAddress,
		input: &'a [u8],
	},
	ProgramLoadRandomData
	{
		column_address: ColumnAddress,
		input: &'a [u8],
	},
	ProgramExecute
	{
		row_address: RowAddress<Chip>,
	},
	ReadId
	{
		manufacturer_id: &'a mut u8,
		device_id: &'a mut u8,
	},
	GetFeatures
	{
		features_address: u8,
		features_value: &'a mut u8,
	},
	SetFeatures
	{
		features_address: u8,
		features_value: u8,
	},
}

impl<'a, Chip: FlashMemoryChip> Command<'a, Chip>
{
	/// Send this command to the `spi_device` flash memory.
	///
	/// Returns `Ok(())` if the command has been sent succesfully, otherwise returns `Err(...)`.
	pub fn execute<Spi: SpiDevice<u8>>(self, spi_device: &mut Spi) -> Result<(), <Spi as ErrorType>::Error>
	{
		let op_code = [self.op_code()];
		let op_code_operation = Operation::Write(&op_code);

		match self
		{
			Command::PageRead { row_address } =>
			{
				spi_device.transaction(&mut [op_code_operation, Operation::Write(row_address.as_bytes())])?;
				// Here I can read the status register
			},
			Command::ReadFromCache { column_address, output } =>
			{
				spi_device.transaction(&mut [
					op_code_operation,
					Operation::Write(column_address.as_bytes()),
					Operation::Write(&[0]), // Dummy byte
					Operation::Read(output),
				])?;
			},
			Command::BlockErase { row_address } =>
			{
				spi_device.transaction(&mut [op_code_operation, Operation::Write(row_address.as_bytes())])?;
			},
			Command::ProgramLoad { column_address, input } =>
			{
				spi_device.transaction(&mut [
					op_code_operation,
					Operation::Write(column_address.as_bytes()),
					Operation::Write(input),
				])?;
			},
			Command::ProgramLoadRandomData { column_address, input } =>
			{
				spi_device.transaction(&mut [
					op_code_operation,
					Operation::Write(column_address.as_bytes()),
					Operation::Write(input),
				])?;
			},
			Command::ProgramExecute { row_address } =>
			{
				spi_device.transaction(&mut [op_code_operation, Operation::Write(row_address.as_bytes())])?;
			},
			Command::Reset | Command::WriteEnable | Command::WriteDisable =>
			{
				spi_device.transaction(&mut [op_code_operation])?;
			},
			Command::ReadId {
				manufacturer_id,
				device_id,
			} =>
			{
				spi_device.transaction(&mut [
					op_code_operation,
					Operation::Write(&[0]), // Dummy byte
					Operation::Read(std::slice::from_mut(manufacturer_id)),
					Operation::Read(std::slice::from_mut(device_id)),
				])?;
			},
			Command::GetFeatures {
				features_address: feature_address,
				features_value: feature_value,
			} =>
			{
				spi_device.transaction(&mut [
					op_code_operation,
					Operation::Write(&[feature_address]),
					Operation::Read(std::slice::from_mut(feature_value)),
				])?;
			},
			Command::SetFeatures {
				features_address: feature_address,
				features_value: feature_value,
			} =>
			{
				spi_device.transaction(&mut [
					op_code_operation,
					Operation::Write(&[feature_address]),
					Operation::Write(&[feature_value]),
				])?;
			},
		}

		Ok(())
	}

	fn op_code(&self) -> u8
	{
		match self
		{
			Command::Reset => 0xFF,
			Command::WriteEnable => 0x06,
			Command::WriteDisable => 0x04,
			Command::PageRead { row_address: _ } => 0x13,
			Command::ReadFromCache {
				column_address: _,
				output: _,
			} => 0x03,
			Command::BlockErase { row_address: _ } => 0xD8,
			Command::ProgramLoad {
				column_address: _,
				input: _,
			} => 0x02,
			Command::ProgramLoadRandomData {
				column_address: _,
				input: _,
			} => 0x84,
			Command::ProgramExecute { row_address: _ } => 0x10,
			Command::ReadId {
				manufacturer_id: _,
				device_id: _,
			} => 0x9F,
			Command::GetFeatures {
				features_address: _,
				features_value: _,
			} => 0x0F,
			Command::SetFeatures {
				features_address: _,
				features_value: _,
			} => 0x1F,
		}
	}
}
