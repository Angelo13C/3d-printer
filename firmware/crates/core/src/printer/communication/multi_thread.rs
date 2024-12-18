use std::thread::JoinHandle;

#[cfg(feature = "usb")]
use usb_device::class_prelude::UsbBus as UsbBusTrait;

use super::{
	http::command::{CommandsReceiver, CommandsSender},
	Communication, CommunicationConfig,
};
use crate::printer::components::{
	hal::watchdog::{Watchdog, WatchdogCreator},
	time::SystemTime,
	Peripherals, Printer3DComponents,
};

/// Manages multi-threaded communication for the 3D printer.
///
/// This struct handles the communication in a separate thread, allowing the main
/// application to interact with the printer without blocking. It receives commands
/// and executes them in the context of the printer's components.
pub struct MultiThreadCommunication<P: Peripherals + 'static>
{
	/// The handle for the spawned thread running the communication loop.
	join_handle: JoinHandle<()>,
	/// The receiver for commands sent from the communication thread.
	command_receiver: CommandsReceiver<SendablePeripherals<P>>,
}

impl<P: Peripherals + 'static> MultiThreadCommunication<P>
{
	/// Starts a new thread for communication and executes the `tick` method in a loop.
	///
	/// # Arguments
	///
	/// * `peripherals` - Mutable reference to the peripherals required for communication.
	/// * `configuration` - Configuration settings for the communication.
	///
	/// # Returns
	///
	/// A result containing the new `MultiThreadCommunication` instance or an error if creation fails.
	pub fn new(peripherals: &mut P, configuration: CommunicationConfig) -> Result<Self, std::io::Error>
	{
		let (command_sender, command_receiver) = CommandsSender::new();

		let mut sendable_peripherals = SendablePeripherals::<P>::of_communication_thread(peripherals);

		let system_time = peripherals.take_system_time();
		let delay_between_ticks = configuration.delay_between_ticks;

		let join_handle = std::thread::Builder::new()
			.stack_size(15_000)
			.name("Communication".to_string())
			.spawn(move || {
				let mut communication =
					Communication::new(&mut sendable_peripherals, configuration, command_sender).unwrap();

				let mut watchdog = sendable_peripherals
					.take_watchdog_creator()
					.map(|watchdog_creator| watchdog_creator.watch_current_thread())
					.flatten();

				loop
				{
					if let Some(watchdog) = watchdog.as_mut()
					{
						if let Err(error) = watchdog.feed()
						{
							log::error!("Watchdog feed error: {error:#?}");
						}
					}

					if let Err(error) = communication.tick()
					{
						log::error!("Tick error: {error:#?}");
					}

					if let Some(system_time) = system_time.as_ref()
					{
						system_time.delay(delay_between_ticks);
					}
				}
			})?;

		Ok(Self {
			join_handle,
			command_receiver,
		})
	}

	/// Retrieves a reference to the join handle of the communication thread.
	///
	/// # Returns
	///
	/// A reference to the `JoinHandle` of the communication thread.
	pub fn get_join_handle(&self) -> &JoinHandle<()>
	{
		&self.join_handle
	}

	/// Executes all commands received from the communication thread.
	///
	/// # Arguments
	///
	/// * `components` - Mutable reference to the printer's components where commands will be executed.
	pub fn tick(&self, components: &mut Printer3DComponents<SendablePeripherals<P>>)
	{
		for command in self.command_receiver.iterate_received_commands()
		{
			command.execute(components);
		}
	}
}

/// Represents peripherals that can be sent between threads.
///
/// This enum allows for safe transmission of peripheral data across thread boundaries
/// in the context of communication and components management.
pub enum SendablePeripherals<P: Peripherals>
{
	/// Contains peripherals required for the components thread.
	ComponentsThread
	{
		watchdog_creator: Option<P::WatchdogCreator>,

		stepper_ticker_timer: Option<P::StepperTickerTimer>,

		kinematics: Option<P::Kinematics>,

		/// Motor control pins for various axes.
		left_motor_dir_pin: Option<P::LeftDirPin>,
		left_motor_step_pin: Option<P::LeftStepPin>,
		right_motor_dir_pin: Option<P::RightDirPin>,
		right_motor_step_pin: Option<P::RightStepPin>,
		z_axis_motor_dir_pin: Option<P::ZAxisDirPin>,
		z_axis_motor_step_pin: Option<P::ZAxisStepPin>,
		extruder_motor_dir_pin: Option<P::ExtruderDirPin>,
		extruder_motor_step_pin: Option<P::ExtruderStepPin>,

		/// UART driver for serial communication.
		uart_driver: Option<P::UartDriver>,

		/// Endstop sensors for the axes.
		x_axis_endstop: Option<P::XAxisEndstop>,
		y_axis_endstop: Option<P::YAxisEndstop>,
		z_axis_endstop: Option<P::ZAxisEndstop>,

		/// Fan control pins.
		layer_fan_pin: Option<P::FanPin>,
		hotend_fan_pin: Option<P::FanPin>,

		/// Heater and thermistor pins for the bed and hotend.
		bed_cartridge_heater_pin: Option<P::HeatedBedHeaterPin>,
		bed_thermistor_pin: Option<P::HeatedBedAdcPin>,

		hotend_cartridge_heater_pin: Option<P::CartridgeHeaterPin>,
		hotend_thermistor_pin: Option<P::HotendAdcPin>,

		/// ADC for reading sensor values.
		adc: Option<P::Adc>,

		/// System time for managing delays.
		system_time: Option<P::SystemTime>,
	},
	/// Contains peripherals required for the communication thread.
	CommunicationThread
	{
		watchdog_creator: Option<P::WatchdogCreator>,

		system_time: Option<P::SystemTime>,
		/// Flash chip for memory operations.
		flash_chip: Option<P::FlashChip>,
		/// SPI interface for the flash chip.
		flash_spi: Option<P::FlashSpi>,
		/// WiFi driver for connectivity.
		wifi_driver: Option<P::WifiDriver>,
		/// HTTP server for handling requests.
		server: Option<Box<dyn FnOnce() -> Result<P::Server, P::ServerError> + Send>>,
		/// OTA (Over-the-Air) update interface.
		ota: Option<P::Ota>,

		#[cfg(feature = "usb")]
		/// USB bus for USB communication.
		usb_bus: Option<P::UsbBus>,
		#[cfg(feature = "usb")]
		/// USB sense pin for detecting USB connections.
		usb_sense_pin: Option<P::UsbSensePin>,
	},
}

/// Safety trait implementation for sending `SendablePeripherals` across threads.
unsafe impl<P: Peripherals> Send for SendablePeripherals<P> {}

impl<P: Peripherals> SendablePeripherals<P>
{
	/// Constructs a `SendablePeripherals` instance for the communication thread.
	///
	/// # Arguments
	///
	/// * `peripherals` - Mutable reference to the peripherals from which data will be taken.
	///
	/// # Returns
	///
	/// A `SendablePeripherals` instance configured for the communication thread.
	pub fn of_communication_thread(peripherals: &mut P) -> Self
	{
		Self::CommunicationThread {
			watchdog_creator: peripherals.take_watchdog_creator(),
			system_time: peripherals.take_system_time(),
			flash_chip: peripherals.take_flash_chip(),
			flash_spi: peripherals.take_flash_spi(),
			wifi_driver: peripherals.take_wifi_driver(),
			server: peripherals.take_http_server(),
			ota: peripherals.take_ota(),
			#[cfg(feature = "usb")]
			usb_bus: peripherals.take_usb_bus(),
			#[cfg(feature = "usb")]
			usb_sense_pin: peripherals.take_usb_sense_pin(),
		}
	}

	/// Constructs a `SendablePeripherals` instance for the components thread.
	///
	/// # Arguments
	///
	/// * `peripherals` - Mutable reference to the peripherals from which data will be taken.
	///
	/// # Returns
	///
	/// A `SendablePeripherals` instance configured for the components thread.
	pub fn of_components_thread(peripherals: &mut P) -> Self
	{
		Self::ComponentsThread {
			watchdog_creator: peripherals.take_watchdog_creator(),
			stepper_ticker_timer: peripherals.take_stepper_ticker_timer(),
			kinematics: peripherals.take_kinematics(),
			left_motor_dir_pin: peripherals.take_left_motor_dir_pin(),
			left_motor_step_pin: peripherals.take_left_motor_step_pin(),
			right_motor_dir_pin: peripherals.take_right_motor_dir_pin(),
			right_motor_step_pin: peripherals.take_right_motor_step_pin(),
			z_axis_motor_dir_pin: peripherals.take_z_axis_motor_dir_pin(),
			z_axis_motor_step_pin: peripherals.take_z_axis_motor_step_pin(),
			extruder_motor_dir_pin: peripherals.take_extruder_motor_dir_pin(),
			extruder_motor_step_pin: peripherals.take_extruder_motor_step_pin(),
			uart_driver: peripherals.take_uart_driver(),
			x_axis_endstop: peripherals.take_x_axis_endstop(),
			y_axis_endstop: peripherals.take_y_axis_endstop(),
			z_axis_endstop: peripherals.take_z_axis_endstop(),
			layer_fan_pin: peripherals.take_layer_fan_pin(),
			hotend_fan_pin: peripherals.take_hotend_fan_pin(),
			bed_cartridge_heater_pin: peripherals.take_bed_cartridge_heater_pin(),
			bed_thermistor_pin: peripherals.take_bed_thermistor_pin(),
			hotend_cartridge_heater_pin: peripherals.take_hotend_cartridge_heater_pin(),
			hotend_thermistor_pin: peripherals.take_hotend_thermistor_pin(),
			adc: peripherals.take_adc(),
			system_time: peripherals.take_system_time(),
		}
	}
}

#[allow(unused_variables)]
impl<P: Peripherals> Peripherals for SendablePeripherals<P>
{
	type WatchdogCreator = P::WatchdogCreator;

	type Kinematics = P::Kinematics;

	type StepperTickerTimer = P::StepperTickerTimer;

	type LeftDirPin = P::LeftDirPin;
	type LeftStepPin = P::LeftStepPin;
	type RightDirPin = P::RightDirPin;
	type RightStepPin = P::RightStepPin;
	type ZAxisDirPin = P::ZAxisDirPin;
	type ZAxisStepPin = P::ZAxisStepPin;
	type ExtruderDirPin = P::ExtruderDirPin;
	type ExtruderStepPin = P::ExtruderStepPin;

	type UartDriver = P::UartDriver;

	type XAxisEndstop = P::XAxisEndstop;
	type YAxisEndstop = P::YAxisEndstop;
	type ZAxisEndstop = P::ZAxisEndstop;

	type CartridgeHeaterPin = P::CartridgeHeaterPin;
	type HotendAdcPin = P::HotendAdcPin;

	type HeatedBedHeaterPin = P::HeatedBedHeaterPin;
	type HeatedBedAdcPin = P::HeatedBedAdcPin;

	type FlashChip = P::FlashChip;
	type FlashSpi = P::FlashSpi;

	type Adc = P::Adc;

	type FanPin = P::FanPin;

	type SystemTime = P::SystemTime;

	type WifiDriver = P::WifiDriver;

	type Server = P::Server;
	type ServerError = P::ServerError;

	type Ota = P::Ota;

	#[cfg(feature = "usb")]
	type UsbSensePin = P::UsbSensePin;

	#[cfg(feature = "usb")]
	type UsbBus = P::UsbBus;

	fn take_watchdog_creator(&mut self) -> Option<Self::WatchdogCreator>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => watchdog_creator.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => watchdog_creator.take(),
		}
	}

	fn take_kinematics(&mut self) -> Option<Self::Kinematics>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => kinematics.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => None,
		}
	}

	fn take_stepper_ticker_timer(&mut self) -> Option<Self::StepperTickerTimer>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => stepper_ticker_timer.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => None,
		}
	}

	fn take_left_motor_dir_pin(&mut self) -> Option<Self::LeftDirPin>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => left_motor_dir_pin.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => None,
		}
	}

	fn take_left_motor_step_pin(&mut self) -> Option<Self::LeftStepPin>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => left_motor_step_pin.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => None,
		}
	}

	fn take_right_motor_dir_pin(&mut self) -> Option<Self::RightDirPin>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => right_motor_dir_pin.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => None,
		}
	}

	fn take_right_motor_step_pin(&mut self) -> Option<Self::RightStepPin>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => right_motor_step_pin.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => None,
		}
	}

	fn take_z_axis_motor_dir_pin(&mut self) -> Option<Self::ZAxisDirPin>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => z_axis_motor_dir_pin.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => None,
		}
	}

	fn take_z_axis_motor_step_pin(&mut self) -> Option<Self::ZAxisStepPin>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => z_axis_motor_step_pin.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => None,
		}
	}

	fn take_extruder_motor_dir_pin(&mut self) -> Option<Self::ExtruderDirPin>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => extruder_motor_dir_pin.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => None,
		}
	}

	fn take_extruder_motor_step_pin(&mut self) -> Option<Self::ExtruderStepPin>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => extruder_motor_step_pin.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => None,
		}
	}

	fn take_uart_driver(&mut self) -> Option<Self::UartDriver>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => uart_driver.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => None,
		}
	}

	fn take_x_axis_endstop(&mut self) -> Option<Self::XAxisEndstop>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => x_axis_endstop.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => None,
		}
	}

	fn take_y_axis_endstop(&mut self) -> Option<Self::YAxisEndstop>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => y_axis_endstop.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => None,
		}
	}

	fn take_z_axis_endstop(&mut self) -> Option<Self::ZAxisEndstop>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => z_axis_endstop.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => None,
		}
	}

	fn take_flash_chip(&mut self) -> Option<Self::FlashChip>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => None,
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => flash_chip.take(),
		}
	}

	fn take_flash_spi(&mut self) -> Option<Self::FlashSpi>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => None,
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => flash_spi.take(),
		}
	}

	fn take_adc(&mut self) -> Option<Self::Adc>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => adc.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => None,
		}
	}

	fn take_bed_thermistor_pin(&mut self) -> Option<Self::HeatedBedAdcPin>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => bed_thermistor_pin.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => None,
		}
	}

	fn take_bed_cartridge_heater_pin(&mut self) -> Option<Self::HeatedBedHeaterPin>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => bed_cartridge_heater_pin.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => None,
		}
	}

	fn take_hotend_thermistor_pin(&mut self) -> Option<Self::HotendAdcPin>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => hotend_thermistor_pin.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => None,
		}
	}

	fn take_hotend_cartridge_heater_pin(&mut self) -> Option<Self::CartridgeHeaterPin>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => hotend_cartridge_heater_pin.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => None,
		}
	}

	fn take_layer_fan_pin(&mut self) -> Option<Self::FanPin>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => layer_fan_pin.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => None,
		}
	}

	fn take_hotend_fan_pin(&mut self) -> Option<Self::FanPin>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => hotend_fan_pin.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => None,
		}
	}

	fn take_system_time(&mut self) -> Option<Self::SystemTime>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => system_time.take(),
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => system_time.take(),
		}
	}

	fn take_wifi_driver(&mut self) -> Option<Self::WifiDriver>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => None,
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => wifi_driver.take(),
		}
	}

	fn take_http_server(&mut self) -> Option<Box<dyn FnOnce() -> Result<Self::Server, Self::ServerError> + Send>>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => None,
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => server.take(),
		}
	}

	fn take_ota(&mut self) -> Option<Self::Ota>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => None,
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => ota.take(),
		}
	}

	fn reboot_fn() -> fn()
	{
		P::reboot_fn()
	}

	#[cfg(feature = "usb")]
	fn take_usb_sense_pin(&mut self) -> Option<Self::UsbSensePin>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => None,
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => usb_sense_pin.take(),
		}
	}

	#[cfg(feature = "usb")]
	fn take_usb_bus(&mut self) -> Option<Self::UsbBus>
	{
		match self
		{
			SendablePeripherals::ComponentsThread {
				watchdog_creator,
				stepper_ticker_timer,
				kinematics,
				left_motor_dir_pin,
				left_motor_step_pin,
				right_motor_dir_pin,
				right_motor_step_pin,
				z_axis_motor_dir_pin,
				z_axis_motor_step_pin,
				extruder_motor_dir_pin,
				extruder_motor_step_pin,
				uart_driver,
				x_axis_endstop,
				y_axis_endstop,
				z_axis_endstop,
				layer_fan_pin,
				hotend_fan_pin,
				bed_cartridge_heater_pin,
				bed_thermistor_pin,
				hotend_cartridge_heater_pin,
				hotend_thermistor_pin,
				adc,
				system_time,
			} => None,
			SendablePeripherals::CommunicationThread {
				watchdog_creator,
				system_time,
				flash_chip,
				flash_spi,
				wifi_driver,
				server,
				ota,
				#[cfg(feature = "usb")]
				usb_bus,
				#[cfg(feature = "usb")]
				usb_sense_pin,
			} => usb_bus.take(),
		}
	}

	fn get_ip_address_from_wifi_driver_function() -> fn(&Self::WifiDriver) -> Option<std::net::IpAddr>
	{
		P::get_ip_address_from_wifi_driver_function()
	}
}
