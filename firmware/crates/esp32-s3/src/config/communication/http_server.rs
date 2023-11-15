use std::time::Duration;

use esp_idf_svc::http::server::Configuration;

pub const HTTP_SERVER_CONFIG: Configuration = Configuration {
	http_port: 80,
	https_port: 443,
	max_sessions: 2,
	session_timeout: Duration::from_secs(20 * 60),
	#[cfg(not(esp_idf_esp_https_server_enable))]
	stack_size: 6144,
	#[cfg(esp_idf_esp_https_server_enable)]
	stack_size: 10240,
	max_open_sockets: 2,
	max_uri_handlers: firmware_core::printer::communication::http::request::http_request_handlers_count(),
	max_resp_handlers: 8,
	lru_purge_enable: true,
	uri_match_wildcard: false,
	#[cfg(esp_idf_esp_https_server_enable)]
	server_certificate: None,
	#[cfg(esp_idf_esp_https_server_enable)]
	private_key: None,
};
