pub mod components;

use components::{Peripherals, Printer3DComponents, Config, CreationError};

pub struct Printer3D<P: Peripherals>
{
    components: Printer3DComponents<P>
}

impl<P: Peripherals> Printer3D<P>
{
    pub fn new(mut peripherals: P, components_config: Config) -> Result<Self, CreationError>
    {
        Ok(Self
        {
            components: Printer3DComponents::new(&mut peripherals, components_config)?
        })
    }

    pub fn tick(&mut self)
    {
        self.components.tick();
    }
}