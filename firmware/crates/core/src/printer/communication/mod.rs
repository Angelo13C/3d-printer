//! This module contains everything related to making the 3D printer able to communicate with the external world.
//! Communication methods may include:
//! - WiFi
//! - USB (not supported yet)
//! - Bluetooth (not supported yet)
//!
//! The data exchange with the external world is used to monitor the printer's status and execute commands
//! (like starting the printing of a 3D model).

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

// Module components that facilitate communication.
use super::components::{
	file_system::{self, regions::RegionsConfig, FileSystem},
	print_process::{self, PrintProcessError},
	Peripherals,
};

/// A struct for managing communication with external entities.
pub struct Communication<P: Peripherals + 'static>
{
	/// WiFi communicator instance for managing WiFi communications.
	wifi: WifiCommunicator<P::WifiDriver, P::Server>,
	#[cfg(feature = "usb")]
	/// USB communicator instance for managing USB communications (not yet implemented).
	usb: UsbCommunicator<P::UsbSensePin, P::UsbBus>,

	/// Resources required for HTTP communication and OTA updates.
	resources: Resources<P>,
}

impl<P: Peripherals + 'static> Communication<P>
{
	/// Creates a new `Communication` instance with the specified peripherals and configurations.
	///
	/// # Arguments
	///
	/// * `peripherals` - The peripherals required for communication.
	/// * `configuration` - The configuration settings for communication.
	/// * `command_sender` - A sender for commands to be communicated to the main thread.
	///
	/// # Returns
	///
	/// A result containing the newly created `Communication` instance or an error if creation fails.
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

		log::info!("Register all the HTTP URI handlers");

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

	/// Executes periodic tasks for the communication module.
	///
	/// This method performs tasks like checking if the print process requires new lines to be read from the file system,
	/// handling OTA updates...
	///
	/// # Returns
	///
	/// A result indicating success or failure of the tick operation.
	pub fn tick(&mut self) -> Result<(), TickError<P>>
	{
		if let Some(mut resources) = self.resources.try_lock()
		{
			let (file_system, print_process) = resources.get_file_system_and_print_process();
			match print_process.tick(file_system, print_process::get_commands_in_buffer_count())
			{
				Ok(result) =>
				{
					if let Some(read_lines) = result.read_lines
					{
						resources
							.command_sender
							.send_command(Command::AddGCodeCommandsToBuffer(result.read_commands))
							.map_err(TickError::Send)?;

						resources.g_code_history.add_read_lines(read_lines);
					}
				},
				Err(error) => return Err(TickError::PrintProcessTick(error)),
			}
		}

		Ok(())
	}
}

/// Errors that may occur during the creation of the `Communication` struct.
pub enum CreationError<WifiDriver: Wifi, WifiCommunicator: Communicator, Spi: SpiDevice<u8>, ServerError: Debug>
{
	/// A peripheral from the provided ones is missing (`name` is the name of the peripheral that's missing).
	/// This means that `peripherals.take_...()` returned `None` instead of `Some`.
	PeripheralMissing
	{
		name: &'static str
	},

	/// An error related to security configuration.
	Security(security::CreationError),
	/// An error related to the WiFi communicator.
	Wifi(communicator::wifi::CreationError<WifiDriver>),
	/// An error occurred while registering WiFi requests.
	WifiRegisterRequests(WifiCommunicator::Error),
	/// An error related to the HTTP server.
	HttpServer(ServerError),
	/// An error related to the file system.
	FileSystem(file_system::CreationError<Spi>),
}

/// Errors that may occur during the tick operation of the `Communication` struct.
pub enum TickError<P: Peripherals>
{
	/// An error occurred while sending a command.
	Send(SendError<Command<P>>),
	/// An error occurred during the print process tick operation.
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

/// Configuration settings for the communication module.
pub struct CommunicationConfig
{
	/// Configuration settings for the WiFi connection.
	pub wifi: WifiCreationConfig,
	#[cfg(feature = "usb")]
	/// Configuration settings for USB communication (not yet implemented).
	pub usb: UsbCreationConfig,

	/// Configuration settings for the file system.
	pub file_system: RegionsConfig,
	/// Security configuration settings.
	pub security: security::Configuration,

	/// Maximum number of commands that can be buffered before needing to read new ones.
	pub max_commands_in_buffer_before_reading_new: u16,
	/// Delay duration between communication ticks.
	pub delay_between_ticks: Duration,
}
