use std::fmt::Debug;

use crate::printer::{
	communication::http::{request::HttpRequest, resources::Resources},
	components::Peripherals,
};

pub trait HttpServer
{
	type Error: Debug;

	fn register_request<P: Peripherals + 'static>(
		&mut self, request: HttpRequest, resources: Resources<P>,
	) -> Result<(), Self::Error>;
}
