pub mod components;

use components::{config::ComponentsConfig, Peripherals, Printer3DComponents};

pub struct Printer3D<P: Peripherals>
{
	components: Printer3DComponents<P>,
}

impl<P: Peripherals> Printer3D<P>
{
	pub fn new(mut peripherals: P, components_config: ComponentsConfig) -> Result<Self, CreationError>
	{
		Ok(Self {
			components: Printer3DComponents::new(&mut peripherals, components_config)
				.map_err(CreationError::Components)?,
		})
	}

	pub fn tick(&mut self)
	{
		self.components.tick();
	}
}

#[derive(Debug)]
pub enum CreationError
{
	Components(components::CreationError),
}
