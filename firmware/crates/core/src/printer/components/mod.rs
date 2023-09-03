pub mod drivers;
pub mod hal;
mod peripherals;

pub use peripherals::*;

use self::drivers::fan::Fan;
use crate::utils::math::Percentage;

pub struct Printer3DComponents<P: Peripherals>
{
	pub layer_fan: Fan<P::FanPin>,
	pub hotend_fan: Fan<P::FanPin>,
}

impl<P: Peripherals> Printer3DComponents<P>
{
	pub fn new(peripherals: &mut P, config: Config) -> Result<Self, CreationError>
	{
		Ok(Self {
			layer_fan: Fan::new(
				peripherals
					.take_layer_fan_pin()
					.ok_or(CreationError::PeripheralMissing { name: "Layer fan" })?,
				config.layer_fan_min_duty_cycle_to_move,
			),
			hotend_fan: Fan::new(
				peripherals
					.take_hotend_fan_pin()
					.ok_or(CreationError::PeripheralMissing { name: "Hotend fan" })?,
				config.hotend_fan_min_duty_cycle_to_move,
			),
		})
	}

	pub fn tick(&mut self)
	{
		todo!()
	}
}

pub struct Config
{
	pub layer_fan_min_duty_cycle_to_move: Percentage,
	pub hotend_fan_min_duty_cycle_to_move: Percentage,
}

#[derive(Debug)]
pub enum CreationError
{
	PeripheralMissing
	{
		name: &'static str
	},
}
