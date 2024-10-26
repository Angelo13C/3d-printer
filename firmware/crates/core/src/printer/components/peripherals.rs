use std::{fmt::Debug, net::IpAddr};

#[cfg(feature = "usb")]
use embedded_hal::digital::InputPin;
use embedded_hal::{digital::OutputPin, spi::SpiDevice};
use embedded_svc::{ota::Ota, wifi::asynch::Wifi};
#[cfg(feature = "usb")]
use usb_device::class_prelude::UsbBus;

use super::{
	drivers::spi_flash_memory::FlashMemoryChip,
	hal::{
		adc::{Adc, AdcPin},
		pwm::PwmPin,
		timer::Timer,
		uart::Uart,
		watchdog::WatchdogCreator,
	},
	motion::{bed_leveling::ZAxisProbe, homing::endstop::Endstop, kinematics::Kinematics},
	time::SystemTime,
};
use crate::printer::communication::communicator::wifi::HttpServer;

/// A type that implements this trait contains all the peripherals required for the 3D printer's operation.
///
/// You must implement this trait for a struct of the specific microcontroller you want to use with this firmware.
pub trait Peripherals
{
	/// A type implementing a watchdog timer creator.
	type WatchdogCreator: WatchdogCreator;

	/// A type representing the kinematic model of the printer.
	type Kinematics: Kinematics;

	/// A timer used for controlling stepper motors.
	type StepperTickerTimer: Timer;

	/// The output pin used to control the left motor's direction.
	type LeftDirPin: OutputPin + Send + 'static;

	/// The output pin used to control the left motor's stepping.
	type LeftStepPin: OutputPin + Send + 'static;

	/// The output pin used to control the right motor's direction.
	type RightDirPin: OutputPin + Send + 'static;

	/// The output pin used to control the right motor's stepping.
	type RightStepPin: OutputPin + Send + 'static;

	/// The output pin used to control the Z-axis motor's direction.
	type ZAxisDirPin: OutputPin + Send + 'static;

	/// The output pin used to control the Z-axis motor's stepping.
	type ZAxisStepPin: OutputPin + Send + 'static;

	/// The output pin used to control the extruder motor's direction.
	type ExtruderDirPin: OutputPin + Send + 'static;

	/// The output pin used to control the extruder motor's stepping.
	type ExtruderStepPin: OutputPin + Send + 'static;

	/// A type for handling UART communication.
	type UartDriver: Uart;

	/// The type for the X-axis endstop.
	type XAxisEndstop: Endstop + Send + 'static;

	/// The type for the Y-axis endstop.
	type YAxisEndstop: Endstop + Send + 'static;

	/// The type for the Z-axis endstop.
	type ZAxisEndstop: ZAxisProbe + 'static;

	/// A type representing a flash memory chip.
	type FlashChip: FlashMemoryChip + Send + 'static;

	/// A type for interfacing with a SPI flash memory chip.
	type FlashSpi: SpiDevice<u8> + Send + 'static;

	/// A type representing the PWM pin for the cartridge heater.
	type CartridgeHeaterPin: PwmPin;

	/// A type for reading the hotend thermistor value.
	type HotendAdcPin: AdcPin<Self::Adc>;

	/// A type representing the PWM pin for the heated bed.
	type HeatedBedHeaterPin: PwmPin;

	/// A type for reading the heated bed thermistor value.
	type HeatedBedAdcPin: AdcPin<Self::Adc>;

	/// A type for handling analog-to-digital conversion.
	type Adc: Adc;

	/// A type representing the PWM pin for the fan.
	type FanPin: PwmPin;

	/// A type for system time management.
	type SystemTime: SystemTime + Send;

	/// A type for handling WiFi connections.
	type WifiDriver: Wifi + Send + 'static;

	/// A type for managing an HTTP server.
	type Server: HttpServer + 'static;

	/// A type for representing errors from the server.
	type ServerError: Debug;

	/// A type for handling over-the-air updates.
	type Ota: Ota + Send + 'static;

	#[cfg(feature = "usb")]
	/// A type for the USB sense pin.
	type UsbSensePin: InputPin + Send + 'static;

	#[cfg(feature = "usb")]
	/// A type for the USB bus.
	type UsbBus: UsbBus + Send + 'static;

	/// Attempts to take the watchdog creator peripheral.
	/// Returns `None` if the peripheral is not available.
	fn take_watchdog_creator(&mut self) -> Option<Self::WatchdogCreator>;

	/// Attempts to take the kinematics peripheral.
	/// Returns `None` if the peripheral is not available.
	fn take_kinematics(&mut self) -> Option<Self::Kinematics>;

	/// Attempts to take the stepper ticker timer peripheral.
	/// Returns `None` if the peripheral is not available.
	fn take_stepper_ticker_timer(&mut self) -> Option<Self::StepperTickerTimer>;

	/// Attempts to take the left motor direction pin.
	/// Returns `None` if the peripheral is not available.
	fn take_left_motor_dir_pin(&mut self) -> Option<Self::LeftDirPin>;

	/// Attempts to take the left motor step pin.
	/// Returns `None` if the peripheral is not available.
	fn take_left_motor_step_pin(&mut self) -> Option<Self::LeftStepPin>;

	/// Attempts to take the right motor direction pin.
	/// Returns `None` if the peripheral is not available.
	fn take_right_motor_dir_pin(&mut self) -> Option<Self::RightDirPin>;

	/// Attempts to take the right motor step pin.
	/// Returns `None` if the peripheral is not available.
	fn take_right_motor_step_pin(&mut self) -> Option<Self::RightStepPin>;

	/// Attempts to take the Z-axis motor direction pin.
	/// Returns `None` if the peripheral is not available.
	fn take_z_axis_motor_dir_pin(&mut self) -> Option<Self::ZAxisDirPin>;

	/// Attempts to take the Z-axis motor step pin.
	/// Returns `None` if the peripheral is not available.
	fn take_z_axis_motor_step_pin(&mut self) -> Option<Self::ZAxisStepPin>;

	/// Attempts to take the extruder motor direction pin.
	/// Returns `None` if the peripheral is not available.
	fn take_extruder_motor_dir_pin(&mut self) -> Option<Self::ExtruderDirPin>;

	/// Attempts to take the extruder motor step pin.
	/// Returns `None` if the peripheral is not available.
	fn take_extruder_motor_step_pin(&mut self) -> Option<Self::ExtruderStepPin>;

	/// Attempts to take the UART driver.
	/// Returns `None` if the peripheral is not available.
	fn take_uart_driver(&mut self) -> Option<Self::UartDriver>;

	/// Attempts to take the X-axis endstop.
	/// Returns `None` if the peripheral is not available.
	fn take_x_axis_endstop(&mut self) -> Option<Self::XAxisEndstop>;

	/// Attempts to take the Y-axis endstop.
	/// Returns `None` if the peripheral is not available.
	fn take_y_axis_endstop(&mut self) -> Option<Self::YAxisEndstop>;

	/// Attempts to take the Z-axis endstop.
	/// Returns `None` if the peripheral is not available.
	fn take_z_axis_endstop(&mut self) -> Option<Self::ZAxisEndstop>;

	/// Attempts to take the flash chip peripheral.
	/// Returns `None` if the peripheral is not available.
	fn take_flash_chip(&mut self) -> Option<Self::FlashChip>;

	/// Attempts to take the SPI flash peripheral.
	/// Returns `None` if the peripheral is not available.
	fn take_flash_spi(&mut self) -> Option<Self::FlashSpi>;

	/// Attempts to take the ADC peripheral.
	/// Returns `None` if the peripheral is not available.
	fn take_adc(&mut self) -> Option<Self::Adc>;

	/// Attempts to take the heated bed thermistor pin.
	/// Returns `None` if the peripheral is not available.
	fn take_bed_thermistor_pin(&mut self) -> Option<Self::HeatedBedAdcPin>;

	/// Attempts to take the heated bed cartridge heater pin.
	/// Returns `None` if the peripheral is not available.
	fn take_bed_cartridge_heater_pin(&mut self) -> Option<Self::HeatedBedHeaterPin>;

	/// Attempts to take the hotend thermistor pin.
	/// Returns `None` if the peripheral is not available.
	fn take_hotend_thermistor_pin(&mut self) -> Option<Self::HotendAdcPin>;

	/// Attempts to take the hotend cartridge heater pin.
	/// Returns `None` if the peripheral is not available.
	fn take_hotend_cartridge_heater_pin(&mut self) -> Option<Self::CartridgeHeaterPin>;

	/// Attempts to take the layer fan pin.
	/// Returns `None` if the peripheral is not available.
	fn take_layer_fan_pin(&mut self) -> Option<Self::FanPin>;

	/// Attempts to take the hotend fan pin.
	/// Returns `None` if the peripheral is not available.
	fn take_hotend_fan_pin(&mut self) -> Option<Self::FanPin>;

	/// Attempts to take the system time peripheral.
	/// Returns `None` if the peripheral is not available.
	fn take_system_time(&mut self) -> Option<Self::SystemTime>;

	/// Attempts to take the WiFi driver.
	/// Returns `None` if the peripheral is not available.
	fn take_wifi_driver(&mut self) -> Option<Self::WifiDriver>;

	/// A function pointer to retrieve the IP address from the WiFi driver.
	fn get_ip_address_from_wifi_driver_function() -> fn(&Self::WifiDriver) -> Option<IpAddr>;

	/// Attempts to take the HTTP server.
	/// Returns `None` if the server is not available.
	fn take_http_server(&mut self) -> Option<Box<dyn FnOnce() -> Result<Self::Server, Self::ServerError> + Send>>;

	/// Attempts to take the OTA update handler.
	/// Returns `None` if the peripheral is not available.
	fn take_ota(&mut self) -> Option<Self::Ota>;

	/// Returns a function pointer that reboots the microcontroller.
	fn reboot_fn() -> fn();

	#[cfg(feature = "usb")]
	/// Attempts to take the USB sense pin.
	/// Returns `None` if the peripheral is not available.
	fn take_usb_sense_pin(&mut self) -> Option<Self::UsbSensePin>;

	#[cfg(feature = "usb")]
	/// Attempts to take the USB bus.
	/// Returns `None` if the peripheral is not available.
	fn take_usb_bus(&mut self) -> Option<Self::UsbBus>;
}
