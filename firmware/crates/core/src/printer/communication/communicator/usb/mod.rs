use embedded_hal::digital::{ErrorType, InputPin};
use usb_device::{
	bus::UsbBusAllocator,
	class_prelude::UsbBus as UsbBusTrait,
	device::UsbDeviceBuilder,
	prelude::{UsbDevice, UsbVidPid},
	UsbError,
};

pub struct UsbCommunicator<SensePin: InputPin, UsbBus: UsbBusTrait + 'static>
{
	sense_pin: SensePin,
	usb_device: UsbDevice<'static, UsbBus>,
	usb_serial: usbd_serial::SerialPort<'static, UsbBus>,
	read_buffer: Vec<u8>,
	write_buffer: Vec<u8>,
}

impl<SensePin: InputPin, UsbBus: UsbBusTrait> UsbCommunicator<SensePin, UsbBus>
{
	pub fn new(configuration: CreationConfig, usb_bus: UsbBus, usb_sense_pin: SensePin) -> Self
	{
		todo!();
	}

	fn is_host_connected(&self) -> Result<bool, <SensePin as ErrorType>::Error>
	{
		self.sense_pin.is_high()
	}
}

pub struct Settings
{
	pub vid_pid: UsbVidPid,
	pub read_buffer_size: usize,
	pub write_buffer_size: usize,
}

pub struct CreationConfig
{
	pub settings: Settings,
}
