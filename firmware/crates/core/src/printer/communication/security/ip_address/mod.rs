//! This module provides functionality related to IP address handling for security mechanisms.
//! It includes traits and structures for managing both blacklisted and whitelisted IP addresses.
//! The main purpose is to facilitate the retrieval of IP addresses from incoming requests,
//! which can be used for security checks.

mod black_list;
mod white_list;

use embedded_svc::http::server::{Connection, Request};
use spin::MutexGuard;

use super::ProtectionInput;
use crate::printer::{communication::http::resources::ResourcesImpl, components::Peripherals};

/// A trait for retrieving the IP address from a request.
pub trait GetIpAddress
{
	/// Returns the IP address associated with the current request, if available.
	fn get_ip_address(&self) -> Option<IpAddress>;
}

/// Represents an IP address as a 32-bit unsigned integer.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct IpAddress(u32);

impl<'a> ProtectionInput<'a> for IpAddress
{
	/// Generates an `IpAddress` from an HTTP request and the associated resources.
	///
	/// This function is currently not implemented and contains a placeholder for future functionality.
	fn generate<C: Connection, P: Peripherals>(
		_: &'a mut Request<C>, _: &mut MutexGuard<'_, ResourcesImpl<P>>,
	) -> Result<Self, ()>
	{
		todo!(
			"For ESP-IDF I could do something like:
		```
		let handle = request.connection().handle();
		let sockfd = esp_idf_sys::httpd_req_to_sockfd(handle);
		let mut address = esp_idf_sys::sockaddr::default();
		let mut address_size = core::mem::size_of_val(&address);
		if esp_idf_sys::lwip_getpeername(sockfd, &mut address as *mut sockaddr, &mut address_size as *mut socklen_t) < 0
		{{
			return Err();
		}}

		todo!(\"Convert `address` to an IpAddress somehow\");
		```"
		)
	}
}
