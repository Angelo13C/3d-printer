use embedded_svc::{
	io::ErrorType,
	ota::{Ota, OtaUpdate, OtaUpdateFinished},
};

use crate::printer::components::mock::MockError;

pub struct MockOta;

impl Ota for MockOta
{
	type Update<'a> = MockOtaUpdate;

	fn get_boot_slot(&self) -> Result<embedded_svc::ota::Slot, Self::Error>
	{
		todo!()
	}

	fn get_running_slot(&self) -> Result<embedded_svc::ota::Slot, Self::Error>
	{
		todo!()
	}

	fn get_update_slot(&self) -> Result<embedded_svc::ota::Slot, Self::Error>
	{
		todo!()
	}

	fn is_factory_reset_supported(&self) -> Result<bool, Self::Error>
	{
		todo!()
	}

	fn factory_reset(&mut self) -> Result<(), Self::Error>
	{
		todo!()
	}

	fn initiate_update(&mut self) -> Result<Self::Update<'_>, Self::Error>
	{
		todo!()
	}

	fn mark_running_slot_valid(&mut self) -> Result<(), Self::Error>
	{
		todo!()
	}

	fn mark_running_slot_invalid_and_reboot(&mut self) -> Self::Error
	{
		todo!()
	}
}

impl ErrorType for MockOta
{
	type Error = MockError;
}

pub struct MockOtaUpdate;
impl OtaUpdate for MockOtaUpdate
{
	type OtaUpdateFinished = MockOtaUpdateFinished;

	fn finish(self) -> Result<Self::OtaUpdateFinished, Self::Error>
	{
		todo!()
	}

	fn complete(self) -> Result<(), Self::Error>
	{
		todo!()
	}

	fn abort(self) -> Result<(), Self::Error>
	{
		todo!()
	}
}

impl embedded_io::Write for MockOtaUpdate
{
	fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error>
	{
		todo!()
	}

	fn flush(&mut self) -> Result<(), Self::Error>
	{
		todo!()
	}
}
impl ErrorType for MockOtaUpdate
{
	type Error = MockError;
}

pub struct MockOtaUpdateFinished;
impl OtaUpdateFinished for MockOtaUpdateFinished
{
	fn activate(self) -> Result<(), Self::Error>
	{
		todo!()
	}
}
impl ErrorType for MockOtaUpdateFinished
{
	type Error = MockError;
}
