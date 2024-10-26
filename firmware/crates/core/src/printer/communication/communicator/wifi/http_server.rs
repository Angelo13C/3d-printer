use std::fmt::Debug;

use crate::printer::{
	communication::http::{request::HttpRequest, resources::Resources},
	components::Peripherals,
};

/// A trait that defines the behavior of an HTTP server in the microcontroller.
///
/// This trait allows for registering HTTP requests that the server can handle. Any type that implements
/// this trait can serve as an HTTP server, processing incoming requests and responding appropriately.
pub trait HttpServer
{
	/// A type that represents any error that can occur while handling requests.
	type Error: Debug;

	/// Registers a request with the server.
	///
	/// This method takes an [`HttpRequest`] and a set of resources, allowing the server to handle the request
	/// appropriately. It returns `Ok(())` if the registration is successful, or an error of type `Self::Error`
	/// if something goes wrong.
	///
	/// [`HttpRequest`]: crate::printer::communication::http::request::HttpRequest
	fn register_request<P: Peripherals + 'static>(
		&mut self, request: HttpRequest, resources: Resources<P>,
	) -> Result<(), Self::Error>;
}
