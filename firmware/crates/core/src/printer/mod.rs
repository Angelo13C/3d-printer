pub mod components;

use components::{config::ComponentsConfig, Peripherals, Printer3DComponents};

pub struct Printer3D<P: Peripherals>
{
	components: Printer3DComponents<P>,
}

impl<P: Peripherals> Printer3D<P>
{
	pub fn new(
		mut peripherals: P,
		components_config: ComponentsConfig<
			P::StepperTickerTimer,
			P::Kinematics,
			P::LeftDirPin,
			P::LeftStepPin,
			P::RightDirPin,
			P::RightStepPin,
			P::ZAxisDirPin,
			P::ZAxisStepPin,
			P::ExtruderDirPin,
			P::ExtruderStepPin,
			P::XAxisEndstop,
			P::YAxisEndstop,
			P::ZAxisEndstop,
		>,
	) -> Result<Self, CreationError<P>>
	{
		Ok(Self {
			components: Printer3DComponents::new(&mut peripherals, components_config)
				.map_err(CreationError::Components)?,
		})
	}

	pub fn tick(&mut self) -> Result<(), components::TickError<P::ZAxisEndstop>>
	{
		self.components.tick()?;

		Ok(())
	}
}

#[derive(Debug)]
pub enum CreationError<P: Peripherals>
{
	Components(components::CreationError<P::StepperTickerTimer, P::ZAxisEndstop>),
}
