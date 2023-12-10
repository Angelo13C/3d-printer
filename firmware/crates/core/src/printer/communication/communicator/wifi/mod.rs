mod http_server;

use std::fmt::Debug;

use embedded_svc::wifi::{asynch::Wifi, ClientConfiguration, Configuration};
pub use http_server::*;

use super::Communicator;
use crate::printer::{
	communication::http::{request::HttpRequest, resources::Resources},
	components::Peripherals,
};

/// A way to communicate with the external world using WiFi.
pub struct WifiCommunicator<WifiDriver: Wifi, Server: HttpServer>
{
	driver: WifiDriver,
	server: Server,
}

impl<WifiDriver: Wifi, Server: HttpServer> WifiCommunicator<WifiDriver, Server>
{
	/// Setup the provided `wifi` to work as a client based on the provided `configuration`, and returns a new
	/// [`WifiCommunicator`] when the connection is successfully created.
	///
	/// Returns `Err(CreationError)` if there has been any problem setting the configuration of the WiFi, starting it or
	/// connecting it to the network.
	pub async fn new(
		mut wifi: WifiDriver, server: Server, configuration: CreationConfig,
	) -> Result<Self, CreationError<WifiDriver>>
	{
		wifi.set_configuration(&Configuration::Client(configuration.wifi_client_configuration))
			.await
			.map_err(CreationError::<WifiDriver>::SetConfiguration)?;
		wifi.start().await.map_err(CreationError::<WifiDriver>::Start)?;
		wifi.connect().await.map_err(CreationError::<WifiDriver>::Connect)?;

		Ok(Self { driver: wifi, server })
	}

	pub fn get_driver(&self) -> &WifiDriver
	{
		&self.driver
	}
}

impl<WifiDriver: Wifi, Server: HttpServer> Communicator for WifiCommunicator<WifiDriver, Server>
{
	type Error = Server::Error;

	fn register_request<P: Peripherals + 'static>(
		&mut self, request: HttpRequest, resources: Resources<P>,
	) -> Result<(), Self::Error>
	{
		self.server.register_request(request, resources)
	}
}

/// Configuration required to create a [`WifiCommunicator`].
pub struct CreationConfig
{
	/// Configuration of the WiFi client.
	pub wifi_client_configuration: ClientConfiguration,
}

/// An error returned from [`WifiCommunicator::new`].
pub enum CreationError<WifiDriver: Wifi>
{
	/// A problem occurred when calling `Wifi::set_configuration`.
	SetConfiguration(WifiDriver::Error),
	/// A problem occurred when calling `Wifi::start`.
	Start(WifiDriver::Error),
	/// A problem occurred when calling `Wifi::connect`.
	Connect(WifiDriver::Error),
}

impl<WifiDriver: Wifi> Debug for CreationError<WifiDriver>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Self::SetConfiguration(err) => f.debug_tuple("SetConfiguration").field(err).finish(),
			Self::Start(err) => f.debug_tuple("Start").field(err).finish(),
			Self::Connect(err) => f.debug_tuple("Connect").field(err).finish(),
		}
	}
}
