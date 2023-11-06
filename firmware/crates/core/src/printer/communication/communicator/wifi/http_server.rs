use std::fmt::Debug;

use embedded_hal::spi::SpiDevice;
use embedded_svc::http::{
	server::{Connection, HandlerResult, Request},
	Method,
};

use crate::printer::{
	communication::http::{request::HttpRequest, resources::Resources},
	components::{drivers::spi_flash_memory::FlashMemoryChip, Peripherals},
};

pub trait HttpServer
{
	type Error: Debug;

	fn register_request<P: Peripherals + 'static>(
		&mut self, request: HttpRequest, resources: Resources<P>,
	) -> Result<(), Self::Error>;
}
