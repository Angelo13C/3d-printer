mod black_list;
mod white_list;

use embedded_svc::http::server::{Connection, Request};
use spin::MutexGuard;

use super::ProtectionInput;
use crate::printer::{communication::http::resources::ResourcesImpl, components::Peripherals};

pub trait GetIpAddress
{
	fn get_ip_address(&self) -> Option<IpAddress>;
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct IpAddress(u32);
impl<'a> ProtectionInput<'a> for IpAddress
{
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
