use std::{fmt::Debug, net::IpAddr};

#[cfg(feature = "usb")]
use embedded_hal::digital::InputPin;
use embedded_hal::{digital::OutputPin, spi::SpiDevice};
use embedded_svc::wifi::asynch::Wifi;
#[cfg(feature = "usb")]
use usb_device::class_prelude::UsbBus;

use super::{
	drivers::spi_flash_memory::FlashMemoryChip,
	hal::{
		adc::{Adc, AdcPin},
		pwm::PwmPin,
		timer::Timer,
		uart::Uart,
		watchdog::{Watchdog, WatchdogCreator},
	},
	motion::{bed_leveling::ZAxisProbe, homing::endstop::Endstop, kinematics::Kinematics},
	time::SystemTime,
};
use crate::printer::communication::communicator::wifi::HttpServer;

pub trait Peripherals
{
	type WatchdogCreator: WatchdogCreator;

	type Kinematics: Kinematics;
	type StepperTickerTimer: Timer;

	type LeftDirPin: OutputPin + Send + 'static;
	type LeftStepPin: OutputPin + Send + 'static;
	type RightDirPin: OutputPin + Send + 'static;
	type RightStepPin: OutputPin + Send + 'static;
	type ZAxisDirPin: OutputPin + Send + 'static;
	type ZAxisStepPin: OutputPin + Send + 'static;
	type ExtruderDirPin: OutputPin + Send + 'static;
	type ExtruderStepPin: OutputPin + Send + 'static;

	type UartDriver: Uart;

	type XAxisEndstop: Endstop + Send + 'static;
	type YAxisEndstop: Endstop + Send + 'static;
	type ZAxisEndstop: ZAxisProbe + 'static;

	type FlashChip: FlashMemoryChip + Send + 'static;
	type FlashSpi: SpiDevice<u8> + Send + 'static;

	type CartridgeHeaterPin: PwmPin;
	type HotendAdcPin: AdcPin<Self::Adc>;

	type HeatedBedHeaterPin: PwmPin;
	type HeatedBedAdcPin: AdcPin<Self::Adc>;

	type Adc: Adc;

	type FanPin: PwmPin;

	type SystemTime: SystemTime + Send;

	type WifiDriver: Wifi + Send + 'static;
	type Server: HttpServer + 'static;
	type ServerError: Debug;

	#[cfg(feature = "usb")]
	type UsbSensePin: InputPin + Send + 'static;
	#[cfg(feature = "usb")]
	type UsbBus: UsbBus + Send + 'static;

	fn take_watchdog_creator(&mut self) -> Option<Self::WatchdogCreator>;

	fn take_kinematics(&mut self) -> Option<Self::Kinematics>;
	fn take_stepper_ticker_timer(&mut self) -> Option<Self::StepperTickerTimer>;

	fn take_left_motor_dir_pin(&mut self) -> Option<Self::LeftDirPin>;
	fn take_left_motor_step_pin(&mut self) -> Option<Self::LeftStepPin>;
	fn take_right_motor_dir_pin(&mut self) -> Option<Self::RightDirPin>;
	fn take_right_motor_step_pin(&mut self) -> Option<Self::RightStepPin>;
	fn take_z_axis_motor_dir_pin(&mut self) -> Option<Self::ZAxisDirPin>;
	fn take_z_axis_motor_step_pin(&mut self) -> Option<Self::ZAxisStepPin>;
	fn take_extruder_motor_dir_pin(&mut self) -> Option<Self::ExtruderDirPin>;
	fn take_extruder_motor_step_pin(&mut self) -> Option<Self::ExtruderStepPin>;

	fn take_uart_driver(&mut self) -> Option<Self::UartDriver>;

	fn take_x_axis_endstop(&mut self) -> Option<Self::XAxisEndstop>;
	fn take_y_axis_endstop(&mut self) -> Option<Self::YAxisEndstop>;
	fn take_z_axis_endstop(&mut self) -> Option<Self::ZAxisEndstop>;

	fn take_flash_chip(&mut self) -> Option<Self::FlashChip>;
	fn take_flash_spi(&mut self) -> Option<Self::FlashSpi>;

	fn take_adc(&mut self) -> Option<Self::Adc>;

	fn take_bed_thermistor_pin(&mut self) -> Option<Self::HeatedBedAdcPin>;
	fn take_bed_cartridge_heater_pin(&mut self) -> Option<Self::HeatedBedHeaterPin>;
	fn take_hotend_thermistor_pin(&mut self) -> Option<Self::HotendAdcPin>;
	fn take_hotend_cartridge_heater_pin(&mut self) -> Option<Self::CartridgeHeaterPin>;

	fn take_layer_fan_pin(&mut self) -> Option<Self::FanPin>;
	fn take_hotend_fan_pin(&mut self) -> Option<Self::FanPin>;

	fn take_system_time(&mut self) -> Option<Self::SystemTime>;

	fn take_wifi_driver(&mut self) -> Option<Self::WifiDriver>;
	fn get_ip_address_from_wifi_driver_function() -> fn(&Self::WifiDriver) -> Option<IpAddr>;
	fn take_http_server(&mut self) -> Option<Box<dyn FnOnce() -> Result<Self::Server, Self::ServerError> + Send>>;

	#[cfg(feature = "usb")]
	fn take_usb_sense_pin(&mut self) -> Option<Self::UsbSensePin>;
	#[cfg(feature = "usb")]
	fn take_usb_bus(&mut self) -> Option<Self::UsbBus>;
}
