use std::fmt::Debug;

use strum::IntoEnumIterator;

use super::http::{request::HttpRequest, resources::Resources};
use crate::printer::components::Peripherals;

#[cfg(feature = "usb")]
pub mod usb;
pub mod wifi;

/// A type that implements this trait can be used to make the microcontroller communicate with the external
/// world.
pub trait Communicator
{
	type Error: Debug;

	/// Make the [Communicator] register the handle all the possible [`requests`] supported by this firmware.
	///
	/// [`requests`]: HttpRequest
	fn register_all_requests<P: Peripherals + 'static>(&mut self, resources: Resources<P>) -> Result<(), Self::Error>
	{
		for possible_request in HttpRequest::iter()
		{
			self.register_request(possible_request, resources.clone())?;
		}

		Ok(())
	}

	/// Make the [Communicator] register the handle of the provided `request`.
	fn register_request<P: Peripherals + 'static>(
		&mut self, request: HttpRequest, resources: Resources<P>,
	) -> Result<(), Self::Error>;
}
