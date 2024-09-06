pub mod communication;
pub mod components;
pub mod panic_handler;

use std::fmt::Debug;

use communication::CommunicationConfig;
use components::{config::ComponentsConfig, Peripherals, Printer3DComponents};

use self::{
	communication::{MultiThreadCommunication, SendablePeripherals},
	components::hal::watchdog::{Watchdog, WatchdogCreator},
	panic_handler::PanicHandler,
};

pub struct Printer3D<P: Peripherals + 'static>
{
	components: Printer3DComponents<SendablePeripherals<P>>,
	communication: MultiThreadCommunication<P>,
	watchdog: Option<<P::WatchdogCreator as WatchdogCreator>::Watchdog>,
}

impl<P: Peripherals + 'static> Printer3D<P>
{
	pub fn new(
		mut peripherals: P, components_config: ComponentsConfig, communication_config: CommunicationConfig,
		panic_handler: PanicHandler,
	) -> Result<Self, CreationError<P>>
	{
		panic_handler::register_panic_handler(panic_handler);

		Ok(Self {
			components: Printer3DComponents::new(
				&mut SendablePeripherals::of_components_thread(&mut peripherals),
				components_config,
			)
			.map_err(CreationError::Components)?,
			communication: MultiThreadCommunication::new(&mut peripherals, communication_config)
				.map_err(CreationError::Communication)?,
			watchdog: peripherals
				.take_watchdog_creator()
				.map(|watchdog_creator| watchdog_creator.watch_current_thread())
				.flatten(),
		})
	}

	pub fn tick(&mut self) -> Result<(), TickError<P>>
	{
		if let Some(watchdog) = self.watchdog.as_mut()
		{
			watchdog.feed().map_err(TickError::WatchdogReset)?;
		}

		self.components.tick().map_err(TickError::Components)?;
		self.communication.tick(&mut self.components);

		//crate::utils::log_in_isr::print_logs_from_isr();

		Ok(())
	}
}

#[derive(Debug)]
pub enum CreationError<P: Peripherals>
{
	Components(components::CreationError<P::StepperTickerTimer, P::ZAxisEndstop, P::UartDriver>),
	Communication(std::io::Error),
}

pub enum TickError<P: Peripherals>
{
	WatchdogReset(<<P::WatchdogCreator as WatchdogCreator>::Watchdog as Watchdog>::Error),
	Components(components::TickError<P::ZAxisEndstop, P::StepperTickerTimer>),
}

impl<P: Peripherals> Debug for TickError<P>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Self::WatchdogReset(arg0) => f.debug_tuple("WatchdogReset").field(arg0).finish(),
			Self::Components(arg0) => f.debug_tuple("Components").field(arg0).finish(),
		}
	}
}
