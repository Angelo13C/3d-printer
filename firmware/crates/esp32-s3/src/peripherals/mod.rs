pub mod adc;
mod http_server;
pub mod interrupt;
pub mod pwm;
pub mod system_time;
pub mod timer;
pub mod uart;
pub mod watchdog;

use std::{fmt::Debug, net::IpAddr, time::Duration};

use enumset::EnumSet;
use esp_idf_hal::{
	adc::{AdcChannelDriver, AdcDriver, ADC1},
	gpio::*,
	io::EspIOError,
	ledc::{LedcDriver, LedcTimerDriver},
	spi::SpiSingleDeviceDriver,
	task::watchdog::*,
	uart::UartDriver,
};
use esp_idf_svc::{
	eventloop::EspSystemEventLoop, http::server::EspHttpServer, nvs::EspDefaultNvsPartition,
	timer::EspTaskTimerService, wifi::*,
};
use esp_idf_sys::EspError;
use firmware_core::printer::components::{
	drivers::{bl_touch::BLTouch, button::Button, spi_flash_memory::MT29F2G01ABAGDWB},
	motion::kinematics::CoreXYKinematics,
	Peripherals as PeripheralsTrait,
};

use self::{
	adc::{Adc, AdcPin},
	http_server::HttpServer,
	interrupt::InputPin,
	pwm::LedcPwmPin,
	system_time::SystemTime,
	timer::Timer,
	watchdog::WatchdogCreator,
};
use crate::{
	config::components::{ADC_CONFIG, FLASH_SPI_CONFIG, FLASH_SPI_DRIVER_CONFIG},
	peripherals::uart::UARTDriver,
};

pub struct Peripherals
{
	stepper_ticker_timer: Option<<Self as PeripheralsTrait>::StepperTickerTimer>,
	kinematics: <Self as PeripheralsTrait>::Kinematics,

	left_motor_dir_pin: Option<<Self as PeripheralsTrait>::LeftDirPin>,
	left_motor_step_pin: Option<<Self as PeripheralsTrait>::LeftStepPin>,
	right_motor_dir_pin: Option<<Self as PeripheralsTrait>::RightDirPin>,
	right_motor_step_pin: Option<<Self as PeripheralsTrait>::RightStepPin>,
	z_axis_motor_dir_pin: Option<<Self as PeripheralsTrait>::ZAxisDirPin>,
	z_axis_motor_step_pin: Option<<Self as PeripheralsTrait>::ZAxisStepPin>,
	extruder_motor_dir_pin: Option<<Self as PeripheralsTrait>::ExtruderDirPin>,
	extruder_motor_step_pin: Option<<Self as PeripheralsTrait>::ExtruderStepPin>,

	uart_driver: Option<<Self as PeripheralsTrait>::UartDriver>,

	x_axis_endstop: Option<<Self as PeripheralsTrait>::XAxisEndstop>,
	y_axis_endstop: Option<<Self as PeripheralsTrait>::YAxisEndstop>,
	z_axis_endstop: Option<<Self as PeripheralsTrait>::ZAxisEndstop>,

	flash_chip: <Self as PeripheralsTrait>::FlashChip,
	flash_spi: Option<<Self as PeripheralsTrait>::FlashSpi>,

	layer_fan_pin: Option<<Self as PeripheralsTrait>::FanPin>,
	hotend_fan_pin: Option<<Self as PeripheralsTrait>::FanPin>,

	bed_cartridge_heater_pin: Option<<Self as PeripheralsTrait>::HeatedBedHeaterPin>,
	bed_thermistor_pin: Option<<Self as PeripheralsTrait>::HeatedBedAdcPin>,

	hotend_cartridge_heater_pin: Option<<Self as PeripheralsTrait>::CartridgeHeaterPin>,
	hotend_thermistor_pin: Option<<Self as PeripheralsTrait>::HotendAdcPin>,

	adc: Option<<Self as PeripheralsTrait>::Adc>,

	system_time: Option<<Self as PeripheralsTrait>::SystemTime>,

	wifi: Option<<Self as PeripheralsTrait>::WifiDriver>,
	server: Option<
		Box<dyn FnOnce() -> Result<<Self as PeripheralsTrait>::Server, <Self as PeripheralsTrait>::ServerError> + Send>,
	>,

	watchdog_creator: <Self as PeripheralsTrait>::WatchdogCreator,
}

impl PeripheralsTrait for Peripherals
{
	type WatchdogCreator = WatchdogCreator;

	type StepperTickerTimer = Timer;
	type Kinematics = CoreXYKinematics;

	type LeftDirPin = PinDriver<'static, Gpio40, Output>;
	type LeftStepPin = PinDriver<'static, Gpio6, Output>;
	type RightDirPin = PinDriver<'static, Gpio37, Output>;
	type RightStepPin = PinDriver<'static, Gpio38, Output>;
	type ZAxisDirPin = PinDriver<'static, Gpio39, Output>;
	type ZAxisStepPin = PinDriver<'static, Gpio7, Output>;
	type ExtruderDirPin = PinDriver<'static, Gpio41, Output>;
	type ExtruderStepPin = PinDriver<'static, Gpio42, Output>;

	type UartDriver = UARTDriver<'static>;

	type XAxisEndstop = Button<InputPin<'static, Gpio35>>;
	type YAxisEndstop = Button<InputPin<'static, Gpio36>>;
	type ZAxisEndstop = BLTouch<LedcPwmPin<'static>, InputPin<'static, Gpio15>>;

	type FlashChip = MT29F2G01ABAGDWB;
	type FlashSpi = SpiSingleDeviceDriver<'static>;

	type FanPin = LedcPwmPin<'static>;

	type CartridgeHeaterPin = LedcPwmPin<'static>;
	type HotendAdcPin = AdcPin<'static, ADC1, 3, Gpio2>;

	type HeatedBedHeaterPin = LedcPwmPin<'static>;
	type HeatedBedAdcPin = AdcPin<'static, ADC1, 3, Gpio1>;

	type Adc = Adc<'static, ADC1>;

	type SystemTime = SystemTime;

	type WifiDriver = AsyncWifi<EspWifi<'static>>;

	type Server = HttpServer<'static>;
	type ServerError = EspIOError;

	#[cfg(feature = "usb")]
	type UsbSensePin = InputPin<'static, Gpio20>;
	#[cfg(feature = "usb")]
	type UsbBus = firmware_core::printer::components::mock::MockUsbBus;

	fn take_watchdog_creator(&mut self) -> Option<Self::WatchdogCreator>
	{
		Some(self.watchdog_creator.clone())
	}

	fn take_stepper_ticker_timer(&mut self) -> Option<Self::StepperTickerTimer>
	{
		self.stepper_ticker_timer.take()
	}

	fn take_kinematics(&mut self) -> Option<Self::Kinematics>
	{
		Some(self.kinematics.clone())
	}

	fn take_left_motor_dir_pin(&mut self) -> Option<Self::LeftDirPin>
	{
		self.left_motor_dir_pin.take()
	}

	fn take_left_motor_step_pin(&mut self) -> Option<Self::LeftStepPin>
	{
		self.left_motor_step_pin.take()
	}

	fn take_right_motor_dir_pin(&mut self) -> Option<Self::RightDirPin>
	{
		self.right_motor_dir_pin.take()
	}

	fn take_right_motor_step_pin(&mut self) -> Option<Self::RightStepPin>
	{
		self.right_motor_step_pin.take()
	}

	fn take_z_axis_motor_dir_pin(&mut self) -> Option<Self::ZAxisDirPin>
	{
		self.z_axis_motor_dir_pin.take()
	}

	fn take_z_axis_motor_step_pin(&mut self) -> Option<Self::ZAxisStepPin>
	{
		self.z_axis_motor_step_pin.take()
	}

	fn take_extruder_motor_dir_pin(&mut self) -> Option<Self::ExtruderDirPin>
	{
		self.extruder_motor_dir_pin.take()
	}

	fn take_extruder_motor_step_pin(&mut self) -> Option<Self::ExtruderStepPin>
	{
		self.extruder_motor_step_pin.take()
	}

	fn take_uart_driver(&mut self) -> Option<Self::UartDriver>
	{
		self.uart_driver.take()
	}

	fn take_x_axis_endstop(&mut self) -> Option<Self::XAxisEndstop>
	{
		self.x_axis_endstop.take()
	}

	fn take_y_axis_endstop(&mut self) -> Option<Self::YAxisEndstop>
	{
		self.y_axis_endstop.take()
	}

	fn take_z_axis_endstop(&mut self) -> Option<Self::ZAxisEndstop>
	{
		self.z_axis_endstop.take()
	}

	fn take_flash_chip(&mut self) -> Option<Self::FlashChip>
	{
		Some(self.flash_chip.clone())
	}

	fn take_flash_spi(&mut self) -> Option<Self::FlashSpi>
	{
		self.flash_spi.take()
	}

	fn take_layer_fan_pin(&mut self) -> Option<Self::FanPin>
	{
		self.layer_fan_pin.take()
	}

	fn take_hotend_fan_pin(&mut self) -> Option<Self::FanPin>
	{
		self.hotend_fan_pin.take()
	}

	fn take_bed_cartridge_heater_pin(&mut self) -> Option<Self::HeatedBedHeaterPin>
	{
		self.bed_cartridge_heater_pin.take()
	}

	fn take_bed_thermistor_pin(&mut self) -> Option<Self::HeatedBedAdcPin>
	{
		self.bed_thermistor_pin.take()
	}

	fn take_hotend_cartridge_heater_pin(&mut self) -> Option<Self::CartridgeHeaterPin>
	{
		self.hotend_cartridge_heater_pin.take()
	}

	fn take_hotend_thermistor_pin(&mut self) -> Option<Self::HotendAdcPin>
	{
		self.hotend_thermistor_pin.take()
	}

	fn take_adc(&mut self) -> Option<Self::Adc>
	{
		self.adc.take()
	}

	fn take_system_time(&mut self) -> Option<Self::SystemTime>
	{
		self.system_time.clone().take()
	}

	fn take_wifi_driver(&mut self) -> Option<Self::WifiDriver>
	{
		self.wifi.take()
	}

	fn get_ip_address_from_wifi_driver_function() -> fn(&Self::WifiDriver) -> Option<IpAddr>
	{
		|wifi_driver| {
			wifi_driver
				.wifi()
				.sta_netif()
				.get_ip_info()
				.ok()
				.map(|info| IpAddr::V4(info.ip))
		}
	}

	fn take_http_server(&mut self) -> Option<Box<dyn FnOnce() -> Result<Self::Server, Self::ServerError> + Send>>
	{
		self.server.take()
	}

	#[cfg(feature = "usb")]
	fn take_usb_sense_pin(&mut self) -> Option<Self::UsbSensePin>
	{
		todo!()
	}

	#[cfg(feature = "usb")]
	fn take_usb_bus(&mut self) -> Option<Self::UsbBus>
	{
		todo!()
	}
}

impl Peripherals
{
	pub fn from_esp_peripherals(peripherals: esp_idf_hal::peripherals::Peripherals) -> Result<Self, EspError>
	{
		let sys_loop = EspSystemEventLoop::take()?;
		let nvs = EspDefaultNvsPartition::take()?;
		let http_server_config = crate::config::communication::HTTP_SERVER_CONFIG;
		let fans_timer_driver =
			LedcTimerDriver::new(peripherals.ledc.timer0, &crate::config::components::FANS_PWM_TIMER)?;

		Ok(Self {
			watchdog_creator: WatchdogCreator(TWDTDriver::new(
				peripherals.twdt,
				&TWDTConfig {
					duration: Duration::from_secs(10),
					panic_on_trigger: true,
					subscribed_idle_tasks: EnumSet::EMPTY,
				},
			)?),
			system_time: Some(SystemTime::new()?),
			uart_driver: Some(UARTDriver(UartDriver::new(
				peripherals.uart2,
				peripherals.pins.gpio17,
				peripherals.pins.gpio18,
				None as Option<Gpio0>,
				None as Option<Gpio0>,
				&crate::config::components::UART_CONFIGURATION,
			)?)),
			stepper_ticker_timer: Some(Timer::new(
				peripherals.timer00,
				&crate::config::components::STEPPER_TIMER_CONFIG,
			)?),
			kinematics: CoreXYKinematics,
			left_motor_dir_pin: Some(PinDriver::output(peripherals.pins.gpio40)?),
			left_motor_step_pin: Some(PinDriver::output(peripherals.pins.gpio6)?),
			right_motor_dir_pin: Some(PinDriver::output(peripherals.pins.gpio37)?),
			right_motor_step_pin: Some(PinDriver::output(peripherals.pins.gpio38)?),
			z_axis_motor_dir_pin: Some(PinDriver::output(peripherals.pins.gpio39)?),
			z_axis_motor_step_pin: Some(PinDriver::output(peripherals.pins.gpio7)?),
			extruder_motor_dir_pin: Some(PinDriver::output(peripherals.pins.gpio41)?),
			extruder_motor_step_pin: Some(PinDriver::output(peripherals.pins.gpio42)?),
			x_axis_endstop: Some(Button::new(InputPin(PinDriver::input(peripherals.pins.gpio35)?))),
			y_axis_endstop: Some(Button::new(InputPin(PinDriver::input(peripherals.pins.gpio36)?))),
			z_axis_endstop: Some(BLTouch::new(
				LedcPwmPin(LedcDriver::new(
					peripherals.ledc.channel2,
					&LedcTimerDriver::new(
						peripherals.ledc.timer1,
						&crate::config::components::BL_TOUCH_SIGNAL_PWM_TIMER,
					)?,
					peripherals.pins.gpio16,
				)?),
				InputPin(PinDriver::input(peripherals.pins.gpio15)?),
			)?),
			flash_chip: MT29F2G01ABAGDWB,
			flash_spi: Some(SpiSingleDeviceDriver::new_single(
				peripherals.spi2,
				peripherals.pins.gpio12,
				peripherals.pins.gpio11,
				Some(peripherals.pins.gpio13),
				Some(peripherals.pins.gpio10),
				&FLASH_SPI_DRIVER_CONFIG,
				&FLASH_SPI_CONFIG,
			)?),
			hotend_fan_pin: Some(LedcPwmPin(LedcDriver::new(
				peripherals.ledc.channel1,
				&fans_timer_driver,
				peripherals.pins.gpio21,
			)?)),
			layer_fan_pin: Some(LedcPwmPin(LedcDriver::new(
				peripherals.ledc.channel0,
				&fans_timer_driver,
				peripherals.pins.gpio48,
			)?)),
			bed_cartridge_heater_pin: Some(LedcPwmPin(LedcDriver::new(
				peripherals.ledc.channel4,
				&LedcTimerDriver::new(
					peripherals.ledc.timer2,
					&crate::config::components::BED_HEATER_PWM_TIMER,
				)?,
				peripherals.pins.gpio4,
			)?)),
			bed_thermistor_pin: Some(AdcPin::new(AdcChannelDriver::new(peripherals.pins.gpio1)?)),
			hotend_cartridge_heater_pin: Some(LedcPwmPin(LedcDriver::new(
				peripherals.ledc.channel6,
				&LedcTimerDriver::new(
					peripherals.ledc.timer3,
					&crate::config::components::HOTEND_HEATER_PWM_TIMER,
				)?,
				peripherals.pins.gpio5,
			)?)),
			hotend_thermistor_pin: Some(AdcPin::new(AdcChannelDriver::new(peripherals.pins.gpio2)?)),
			adc: Some(Adc(AdcDriver::new(peripherals.adc1, &ADC_CONFIG)?)),
			wifi: Some(AsyncWifi::wrap(
				EspWifi::wrap(WifiDriver::new(peripherals.modem, sys_loop.clone(), Some(nvs))?)?,
				sys_loop,
				EspTaskTimerService::new()?,
			)?),
			server: Some(Box::new(move || {
				Ok(HttpServer(EspHttpServer::new(&http_server_config)?))
			})),
		})
	}
}

impl Debug for Peripherals
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		f.debug_struct("Peripherals").finish()
	}
}
