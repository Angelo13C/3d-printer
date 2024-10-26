//! This module handles the communication between the microcontroller and the external world.
//!
//! It provides the [`Communicator`] trait that defines methods for registering HTTP request handlers
//! and managing communication protocols such as WiFi and USB.
//!
//! The communication is essential for:
//! - Receiving commands from external sources
//! - Sending status updates
//! - Handling various types of requests defined in the firmware

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
	/// The type of error that can occur during communication.
	type Error: Debug;

	/// Registers handlers for all possible HTTP requests supported by this firmware.
	///
	/// This method iterates through all defined `HttpRequest` types and registers
	/// each one with the provided resources.
	///
	/// # Arguments
	///
	/// * `resources` - Resources to be used when handling requests.
	///
	/// # Returns
	///
	/// A result indicating success or failure.
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

	/// Registers a handler for a specific HTTP request.
	///
	/// This method allows for individual request handling to be defined.
	///
	/// # Arguments
	///
	/// * `request` - The specific HTTP request to register.
	/// * `resources` - Resources to be used when handling the request.
	///
	/// # Returns
	///
	/// A result indicating success or failure.
	fn register_request<P: Peripherals + 'static>(
		&mut self, request: HttpRequest, resources: Resources<P>,
	) -> Result<(), Self::Error>;
}
