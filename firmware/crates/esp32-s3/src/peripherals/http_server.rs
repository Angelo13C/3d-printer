use embedded_svc::http::server::Request;
use esp_idf_sys::EspError;
use firmware_core::printer::{
	communication::{
		communicator::wifi::HttpServer as HttpServerTrait,
		http::{request::HttpRequest, resources::Resources},
	},
	components::Peripherals,
};

pub struct HttpServer<'d>(pub esp_idf_svc::http::server::EspHttpServer<'d>);
impl HttpServerTrait for HttpServer<'static>
{
	type Error = EspError;

	fn register_request<P: Peripherals + 'static>(
		&mut self, request: HttpRequest, resources: Resources<P>,
	) -> Result<(), Self::Error>
	{
		let resources_clone = resources.clone();
		let callback = move |arrived_request: Request<&mut esp_idf_svc::http::server::EspHttpConnection>| {
			(request.get_callback())(arrived_request, resources_clone.clone())
		};
		self.0
			.fn_handler(request.get_uri(), request.get_method(), callback)
			.map(|_| ())
	}
}
