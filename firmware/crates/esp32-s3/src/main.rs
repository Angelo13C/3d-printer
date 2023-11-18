pub mod config;
pub mod emergency;
pub mod peripherals;

use esp_idf_hal::{cpu::Core, peripherals::Peripherals as EspPeripherals};
use esp_idf_sys::{self as _, EspError}; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use firmware_core::printer::{panic_handler::PanicHandler, Printer3D};
use peripherals::Peripherals;

fn main()
{
	// It is necessary to call this function once. Otherwise some patches to the runtime
	// implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
	esp_idf_sys::link_patches();
	// Bind the log crate to the ESP Logging facilities
	esp_idf_svc::log::EspLogger::initialize_default();

	let mut printer_3d = create_printer().unwrap();

	loop
	{
		printer_3d.tick().unwrap();
	}
}

fn create_printer() -> Result<Printer3D<Peripherals>, CreatePrinterError>
{
	esp_idf_hal::task::thread::ThreadSpawnConfiguration {
		name: Some(b"Communication thread"),
		stack_size: 10_000,
		priority: 10,
		inherit: false,
		pin_to_core: Some(Core::Core1),
	}
	.set()
	.map_err(CreatePrinterError::ThreadSpawnConfiguration)?;

	let peripherals =
		Peripherals::from_esp_peripherals(EspPeripherals::take().map_err(CreatePrinterError::CantTakeEspPeripherals)?)
			.map_err(CreatePrinterError::CantCreatePeripherals)?;
	Printer3D::new(
		peripherals,
		config::components::configuration(),
		config::communication::configuration(),
		PanicHandler(emergency::disable_all_pins_function),
	)
	.map_err(CreatePrinterError::Printer3DCreation)
}

#[derive(Debug)]
enum CreatePrinterError
{
	ThreadSpawnConfiguration(EspError),
	CantTakeEspPeripherals(EspError),
	CantCreatePeripherals(EspError),
	Printer3DCreation(firmware_core::printer::CreationError<Peripherals>),
}
