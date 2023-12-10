//! This module contains everything related to making the 3D printer able to communicate with the external world.
//! This could be thanks to:
//! - WiFi
//! - USB (not supported yet)
//! - Bluetooth (not supported yet)
//!
//! The data exchange with the external world is used to monitor the printer's status, make it execute commands
//! (like start printing a 3D model)...

use std::{fmt::Debug, sync::mpsc::SendError, time::Duration};

use cassette::{pin_mut, Cassette};
use embedded_hal::spi::SpiDevice;
use embedded_svc::wifi::asynch::Wifi;
#[cfg(feature = "usb")]
use usb_device::class_prelude::UsbBus as UsbBusTrait;

#[cfg(feature = "usb")]
use self::communicator::usb::{CreationConfig as UsbCreationConfig, UsbCommunicator};
use self::{
	communicator::wifi::{CreationConfig as WifiCreationConfig, WifiCommunicator},
	http::{
		command::{Command, CommandsSender},
		resources::Resources,
	},
	security::Security,
};
use crate::printer::{
	communication::{communicator::Communicator, ota::OverTheAirUpdater},
	components::{drivers::spi_flash_memory::SpiFlashMemory, print_process::PrintProcess},
};

pub mod communicator;
pub mod http;
mod multi_thread;
pub mod ota;
pub mod security;

pub use multi_thread::*;

use super::components::{
	file_system::{self, regions::RegionsConfig, FileSystem},
	print_process::{self, PrintProcessError},
	Peripherals,
};

pub struct Communication<P: Peripherals + 'static>
{
	wifi: WifiCommunicator<P::WifiDriver, P::Server>,
	#[cfg(feature = "usb")]
	usb: UsbCommunicator<P::UsbSensePin, P::UsbBus>,

	resources: Resources<P>,
}

impl<P: Peripherals + 'static> Communication<P>
{
	pub fn new(
		peripherals: &mut P, configuration: CommunicationConfig, command_sender: CommandsSender<P>,
	) -> Result<
		Self,
		CreationError<P::WifiDriver, WifiCommunicator<P::WifiDriver, P::Server>, P::FlashSpi, P::ServerError>,
	>
	{
		let wifi_driver = peripherals
			.take_wifi_driver()
			.ok_or(CreationError::PeripheralMissing { name: "Wifi driver" })?;
		let http_server = peripherals
			.take_http_server()
			.ok_or(CreationError::PeripheralMissing { name: "Http server" })?;
		let wifi_communication = WifiCommunicator::new(
			wifi_driver,
			((http_server)()).map_err(CreationError::HttpServer)?,
			configuration.wifi,
		);
		pin_mut!(wifi_communication);
		let mut wifi_cassette = Cassette::new(wifi_communication);

		let mut wifi_communication_option = None;

		while wifi_communication_option.is_none()
		{
			if let Some(wifi_communication) = wifi_cassette.poll_on()
			{
				wifi_communication_option = Some(wifi_communication.map_err(CreationError::Wifi)?);
			}
		}

		let mut wifi = wifi_communication_option.unwrap();

		let file_system = FileSystem::new(
			SpiFlashMemory::new(
				peripherals
					.take_flash_spi()
					.ok_or(CreationError::PeripheralMissing { name: "Flash SPI" })?,
				peripherals
					.take_flash_chip()
					.ok_or(CreationError::PeripheralMissing { name: "Flash chip" })?,
			),
			configuration.file_system,
		)
		.map_err(CreationError::FileSystem)?;
		let http_handler_resources = http::resources::Resources::<P>::new(
			peripherals.take_system_time(),
			file_system,
			OverTheAirUpdater::new(
				peripherals
					.take_ota()
					.ok_or(CreationError::PeripheralMissing { name: "OTA" })?,
				P::reboot_fn(),
			),
			Security::new(configuration.security).map_err(CreationError::Security)?,
			command_sender,
			PrintProcess::new(configuration.max_commands_in_buffer_before_reading_new),
		);

		log::info!("Register all the HTTP uri handlers");

		wifi.register_all_requests(http_handler_resources.clone())
			.map_err(CreationError::WifiRegisterRequests)?;

		log::info!("Finished creating the Communication struct");

		Ok(Self {
			wifi,
			#[cfg(feature = "usb")]
			usb: todo!(),
			resources: http_handler_resources,
		})
	}

	pub fn tick(&mut self) -> Result<(), TickError<P>>
	{
		if let Some(mut resources) = self.resources.try_lock()
		{
			let (file_system, print_process) = resources.get_file_system_and_print_process();
			match print_process.tick(file_system, print_process::get_commands_in_buffer_count())
			{
				Ok(result) =>
				{
					resources
						.command_sender
						.send_command(Command::AddGCodeCommandsToBuffer(result.read_commands))
						.map_err(TickError::Send)?;

					if let Some(read_lines) = result.read_lines
					{
						resources.g_code_history.add_read_lines(read_lines);
					}
				},
				Err(error) => return Err(TickError::PrintProcessTick(error)),
			}
		}

		Ok(())
	}
}

pub enum CreationError<WifiDriver: Wifi, WifiCommunicator: Communicator, Spi: SpiDevice<u8>, ServerError: Debug>
{
	/// A peripheral from the provided ones is missing (`name` is the name of the peripheral that's missing).
	/// This means that `peripherals.take_...()` returned `None` instead of `Some`.
	PeripheralMissing
	{
		name: &'static str,
	},

	Security(security::CreationError),
	Wifi(communicator::wifi::CreationError<WifiDriver>),
	WifiRegisterRequests(WifiCommunicator::Error),
	HttpServer(ServerError),
	FileSystem(file_system::CreationError<Spi>),
}

pub enum TickError<P: Peripherals>
{
	Send(SendError<Command<P>>),
	PrintProcessTick(PrintProcessError<P::FlashSpi>),
}

impl<P: Peripherals> Debug for TickError<P>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			TickError::Send(error) => f.debug_tuple("Send").field(error).finish(),
			TickError::PrintProcessTick(error) => f.debug_tuple("PrintProcessTick").field(error).finish(),
		}
	}
}

impl<WifiDriver: Wifi, WifiCommunicator: Communicator, Spi: SpiDevice<u8>, ServerError: Debug> Debug
	for CreationError<WifiDriver, WifiCommunicator, Spi, ServerError>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			CreationError::Wifi(err) => f.debug_tuple("Wifi").field(err).finish(),
			CreationError::PeripheralMissing { name } =>
			{
				f.debug_struct("PeripheralMissing").field("name", name).finish()
			},
			CreationError::FileSystem(err) => f.debug_tuple("FileSystem").field(err).finish(),
			CreationError::HttpServer(err) => f.debug_tuple("HttpServer").field(err).finish(),
			CreationError::WifiRegisterRequests(err) => f.debug_tuple("WifiRegisterRequests").field(err).finish(),
			CreationError::Security(err) => f.debug_tuple("Security").field(err).finish(),
		}
	}
}

pub struct CommunicationConfig
{
	pub wifi: WifiCreationConfig,
	#[cfg(feature = "usb")]
	pub usb: UsbCreationConfig,

	pub file_system: RegionsConfig,
	pub security: security::Configuration,

	pub max_commands_in_buffer_before_reading_new: u16,
	pub delay_between_ticks: Duration,
}
