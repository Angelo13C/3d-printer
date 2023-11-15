use embedded_hal::spi::SpiDevice;
use embedded_svc::{
	http::{
		server::{Connection, HandlerResult, Request},
		Headers, Query,
	},
	io::{ErrorType, Read, Write},
	wifi::asynch::Wifi,
};

use crate::printer::{
	communication::{
		communicator::wifi::HttpServer,
		http::{request::HttpRequest, resources::Resources},
	},
	components::{drivers::spi_flash_memory::FlashMemoryChip, mock::MockError, Peripherals},
};

pub struct MockWifiDriver;
impl Wifi for MockWifiDriver
{
	type Error = MockError;

	async fn get_capabilities(&self) -> Result<enumset::EnumSet<embedded_svc::wifi::Capability>, Self::Error>
	{
		todo!()
	}

	async fn get_configuration(&self) -> Result<embedded_svc::wifi::Configuration, Self::Error>
	{
		todo!()
	}

	async fn set_configuration(&mut self, conf: &embedded_svc::wifi::Configuration) -> Result<(), Self::Error>
	{
		todo!()
	}

	async fn start(&mut self) -> Result<(), Self::Error>
	{
		todo!()
	}

	async fn stop(&mut self) -> Result<(), Self::Error>
	{
		todo!()
	}

	async fn connect(&mut self) -> Result<(), Self::Error>
	{
		todo!()
	}

	async fn disconnect(&mut self) -> Result<(), Self::Error>
	{
		todo!()
	}

	async fn is_started(&self) -> Result<bool, Self::Error>
	{
		todo!()
	}

	async fn is_connected(&self) -> Result<bool, Self::Error>
	{
		todo!()
	}

	async fn scan_n<const N: usize>(
		&mut self,
	) -> Result<
		(
			serde_json_core::heapless::Vec<embedded_svc::wifi::AccessPointInfo, N>,
			usize,
		),
		Self::Error,
	>
	{
		todo!()
	}

	async fn scan(&mut self) -> Result<alloc::vec::Vec<embedded_svc::wifi::AccessPointInfo>, Self::Error>
	{
		todo!()
	}
}

pub struct MockHttpServer;
impl HttpServer for MockHttpServer
{
	type Error = MockError;

	fn register_request<P: Peripherals>(
		&mut self, request: HttpRequest, resources: Resources<P>,
	) -> Result<(), Self::Error>
	{
		todo!()
	}
}

pub struct MockHttpConnection;
impl Connection for MockHttpConnection
{
	type Headers = MockHeaders;

	type Read = Self;

	type RawConnectionError = MockError;

	type RawConnection = Self;

	fn split(&mut self) -> (&Self::Headers, &mut Self::Read)
	{
		todo!()
	}

	fn initiate_response<'a>(
		&'a mut self, status: u16, message: Option<&'a str>, headers: &'a [(&'a str, &'a str)],
	) -> Result<(), Self::Error>
	{
		todo!()
	}

	fn is_response_initiated(&self) -> bool
	{
		todo!()
	}

	fn raw_connection(&mut self) -> Result<&mut Self::RawConnection, Self::Error>
	{
		todo!()
	}
}
impl Headers for MockHttpConnection
{
	fn header(&self, name: &str) -> Option<&'_ str>
	{
		todo!()
	}
}
impl Query for MockHttpConnection
{
	fn uri(&self) -> &'_ str
	{
		todo!()
	}

	fn method(&self) -> embedded_svc::http::Method
	{
		todo!()
	}
}
impl Read for MockHttpConnection
{
	fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>
	{
		todo!()
	}
}
impl Write for MockHttpConnection
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
impl ErrorType for MockHttpConnection
{
	type Error = MockError;
}

pub struct MockHeaders;
impl Headers for MockHeaders
{
	fn header(&self, name: &str) -> Option<&'_ str>
	{
		todo!()
	}
}
impl Query for MockHeaders
{
	fn uri(&self) -> &'_ str
	{
		todo!()
	}

	fn method(&self) -> embedded_svc::http::Method
	{
		todo!()
	}
}
