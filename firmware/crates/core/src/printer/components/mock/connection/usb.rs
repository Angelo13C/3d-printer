use usb_device::class_prelude::UsbBus;

pub struct MockUsbBus;
impl UsbBus for MockUsbBus
{
	fn alloc_ep(
		&mut self, ep_dir: usb_device::UsbDirection, ep_addr: Option<usb_device::endpoint::EndpointAddress>,
		ep_type: usb_device::endpoint::EndpointType, max_packet_size: u16, interval: u8,
	) -> usb_device::Result<usb_device::endpoint::EndpointAddress>
	{
		todo!()
	}

	fn enable(&mut self)
	{
		todo!()
	}

	fn reset(&self)
	{
		todo!()
	}

	fn set_device_address(&self, addr: u8)
	{
		todo!()
	}

	fn write(&self, ep_addr: usb_device::endpoint::EndpointAddress, buf: &[u8]) -> usb_device::Result<usize>
	{
		todo!()
	}

	fn read(&self, ep_addr: usb_device::endpoint::EndpointAddress, buf: &mut [u8]) -> usb_device::Result<usize>
	{
		todo!()
	}

	fn set_stalled(&self, ep_addr: usb_device::endpoint::EndpointAddress, stalled: bool)
	{
		todo!()
	}

	fn is_stalled(&self, ep_addr: usb_device::endpoint::EndpointAddress) -> bool
	{
		todo!()
	}

	fn suspend(&self)
	{
		todo!()
	}

	fn resume(&self)
	{
		todo!()
	}

	fn poll(&self) -> usb_device::bus::PollResult
	{
		todo!()
	}
}
