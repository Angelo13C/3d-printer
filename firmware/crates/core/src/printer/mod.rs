pub mod communication;
pub mod components;
pub mod panic_handler;

use communication::CommunicationConfig;
use components::{config::ComponentsConfig, Peripherals, Printer3DComponents};

use self::{
	communication::{MultiThreadCommunication, SendablePeripherals},
	panic_handler::PanicHandler,
};

pub struct Printer3D<P: Peripherals + 'static>
{
	components: Printer3DComponents<SendablePeripherals<P>>,
	communication: MultiThreadCommunication<P>,
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
		})
	}

	pub fn tick(&mut self) -> Result<(), components::TickError<P::ZAxisEndstop, P::StepperTickerTimer>>
	{
		self.components.tick()?;
		self.communication.tick(&mut self.components);

		Ok(())
	}
}

#[derive(Debug)]
pub enum CreationError<P: Peripherals>
{
	Components(components::CreationError<P::StepperTickerTimer, P::ZAxisEndstop, P::UartDriver>),
	Communication(std::io::Error),
}
