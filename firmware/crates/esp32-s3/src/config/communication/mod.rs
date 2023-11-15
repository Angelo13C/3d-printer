mod http_server;

use std::time::Duration;

use embedded_svc::wifi::{AuthMethod, ClientConfiguration};
use firmware_core::printer::{
	communication::{
		communicator::wifi::CreationConfig as WifiCreationConfig,
		security::{self, PasswordConfiguration},
		CommunicationConfig,
	},
	components::{file_system::regions::RegionsConfig, Peripherals as PeripheralsTrait},
};
pub use http_server::*;

use crate::peripherals::Peripherals;

pub fn configuration() -> CommunicationConfig
{
	CommunicationConfig {
		wifi: WifiCreationConfig {
			wifi_client_configuration: ClientConfiguration {
				ssid: include_str!("../../../../../../private/Secrets/WiFi/SSID.txt").into(),
				bssid: None,
				auth_method: AuthMethod::WPA2Personal,
				password: include_str!("../../../../../../private/Secrets/WiFi/Password.txt").into(),
				channel: None,
			},
		},
		#[cfg(feature = "usb")]
		usb: todo!(),
		security: security::Configuration {
			password: PasswordConfiguration::PasswordAndBruteforce {
				password: include_str!("../../../../../../private/Secrets/Password/Password.txt"),
				delays_and_wrong_attempts_count_for_it: vec![(3, Duration::from_secs(1))],
			},
		},
		file_system: RegionsConfig::default::<<Peripherals as PeripheralsTrait>::FlashChip>(),
		max_commands_in_buffer_before_reading_new: 20,
	}
}
