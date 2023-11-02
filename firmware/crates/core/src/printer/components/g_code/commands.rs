//! A list of all the G-code commands supported by this firmware.
//!
//! Check the [`RepRap documentation`] for informations on what each command does.
//!
//! [`RepRap documentation`]: <https://reprap.org/wiki/G-code>

use super::{
	execute::{GCodeExecuter, PositionMode},
	parameters::{identifier, value::NoValue, Param},
	GCodeCommand, Status,
};
use crate::{
	printer::components::{
		drivers::fan::Fan,
		hal::{
			adc::{Adc, AdcPin},
			pwm::PwmPin,
		},
		motion::axes::Axis,
		temperature::TemperaturePidController,
		Peripherals, Printer3DComponents,
	},
	utils::{
		math::Percentage,
		measurement::{
			distance::{Distance, Units},
			temperature::Temperature,
		},
	},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct G0
{
	pub x: Param<identifier::X, Distance>,
	pub y: Param<identifier::Y, Distance>,
	pub z: Param<identifier::Z, Distance>,
	pub e: Param<identifier::E, Distance>,
	pub feed_rate: Param<identifier::F, f32>,
}
impl<P: Peripherals> GCodeCommand<P> for G0
{
	fn execute(&mut self, printer_components: &mut Printer3DComponents<P>, _: &mut GCodeExecuter<P>) -> Status
	{
		printer_components.motion_controller.mark_last_move_as_ready_to_go();

		Status::Finished
	}

	fn prepare(
		&mut self, printer_components: &mut Printer3DComponents<P>, g_code_executer: &mut GCodeExecuter<P>,
	) -> Status
	{
		g_code_executer.calculate_position_based_on_mode(
			printer_components,
			&mut self.x.value,
			&mut self.y.value,
			&mut self.z.value,
			&mut self.e.value,
		);
		match printer_components.motion_controller.plan_move(
			self.x.value,
			self.y.value,
			self.z.value,
			self.e.value,
			self.feed_rate.value,
		)
		{
			Ok(_) => Status::Finished,
			Err(_) => Status::Working,
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct G1
{
	pub x: Param<identifier::X, Distance>,
	pub y: Param<identifier::Y, Distance>,
	pub z: Param<identifier::Z, Distance>,
	pub e: Param<identifier::E, Distance>,
	pub feed_rate: Param<identifier::F, f32>,
}
impl<P: Peripherals> GCodeCommand<P> for G1
{
	fn execute(
		&mut self, printer_components: &mut Printer3DComponents<P>, g_code_executer: &mut GCodeExecuter<P>,
	) -> Status
	{
		G0 {
			x: self.x,
			y: self.y,
			z: self.z,
			e: self.e,
			feed_rate: self.feed_rate,
		}
		.execute(printer_components, g_code_executer)
	}

	fn prepare(
		&mut self, printer_components: &mut Printer3DComponents<P>, g_code_executer: &mut GCodeExecuter<P>,
	) -> Status
	{
		G0 {
			x: self.x,
			y: self.y,
			z: self.z,
			e: self.e,
			feed_rate: self.feed_rate,
		}
		.prepare(printer_components, g_code_executer)
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct G20;
impl<P: Peripherals> GCodeCommand<P> for G20
{
	fn execute(&mut self, _: &mut Printer3DComponents<P>, g_code_executer: &mut GCodeExecuter<P>) -> Status
	{
		g_code_executer.set_units(Units::Inches);

		Status::Finished
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct G21;
impl<P: Peripherals> GCodeCommand<P> for G21
{
	fn execute(&mut self, _: &mut Printer3DComponents<P>, g_code_executer: &mut GCodeExecuter<P>) -> Status
	{
		g_code_executer.set_units(Units::Millimeters);

		Status::Finished
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct G28
{
	pub x: Param<identifier::X, NoValue>,
	pub y: Param<identifier::Y, NoValue>,
	pub z: Param<identifier::Z, NoValue>,
}
impl<P: Peripherals> GCodeCommand<P> for G28
{
	fn prepare(&mut self, printer_components: &mut Printer3DComponents<P>, _: &mut GCodeExecuter<P>) -> Status
	{
		match printer_components.motion_controller.start_homing()
		{
			Ok(_) => Status::Finished,
			Err(_) => Status::Working,
		}
	}
}

const DEFAULT_MEMORY_SLOT: usize = 0;
#[derive(Clone, Copy, Debug, PartialEq)]
/// TODO: Find a way to get the current position (because now I can't save it since I don't know how to retrieve it).
pub struct G60
{
	pub slot: Param<identifier::S, usize>,
}
impl<P: Peripherals> GCodeCommand<P> for G60
{
	fn execute(&mut self, _: &mut Printer3DComponents<P>, g_code_executer: &mut GCodeExecuter<P>) -> Status
	{
		let current_position = todo!("Find a way to get the current position");
		match g_code_executer.save_position(current_position, self.slot.value.unwrap_or(DEFAULT_MEMORY_SLOT))
		{
			Ok(_) => Status::Finished,
			Err(_) => Status::Error("Invalid position slot".to_string()),
		}
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct G61
{
	pub axes:
		Param<identifier::AnyWithoutSpaces<(identifier::X, identifier::Y, identifier::Z, identifier::E)>, NoValue>,
	pub feed_rate: Param<identifier::F, f32>,
	pub slot: Param<identifier::S, usize>,
}
impl<P: Peripherals> GCodeCommand<P> for G61
{
	fn execute(
		&mut self, printer_components: &mut Printer3DComponents<P>, g_code_executer: &mut GCodeExecuter<P>,
	) -> Status
	{
		let mut inner = |x: bool, y: bool, z: bool, e: bool| match g_code_executer
			.get_saved_position(self.slot.value.unwrap_or(DEFAULT_MEMORY_SLOT))
		{
			Ok(position) =>
			{
				let x = x.then_some(position[Axis::X as usize]);
				let y = y.then_some(position[Axis::Y as usize]);
				let z = z.then_some(position[Axis::Z as usize]);
				let e = e.then_some(position[Axis::E as usize]);

				match printer_components
					.motion_controller
					.plan_move(x, y, z, e, self.feed_rate.value)
				{
					Ok(_) => Status::Finished,
					Err(_) => Status::Working,
				}
			},
			Err(_) => Status::Error(format!(
				"The position slot {} is invalid (it must be within >= 0 and less than {})",
				self.slot.value.unwrap_or(DEFAULT_MEMORY_SLOT),
				super::execute::SAVED_POSITIONS_COUNT
			)),
		};
		if self.axes.identifier.is_any_present()
		{
			let x = self.axes.identifier.is_identifier_present::<identifier::X>();
			let y = self.axes.identifier.is_identifier_present::<identifier::Y>();
			let z = self.axes.identifier.is_identifier_present::<identifier::Z>();
			let e = self.axes.identifier.is_identifier_present::<identifier::E>();
			inner(x, y, z, e)
		}
		else
		{
			inner(true, true, true, true)
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct G90;
impl<P: Peripherals> GCodeCommand<P> for G90
{
	fn prepare(&mut self, _: &mut Printer3DComponents<P>, g_code_executer: &mut GCodeExecuter<P>) -> Status
	{
		g_code_executer.set_position_mode(PositionMode::Absolute);

		Status::Finished
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct G91;
impl<P: Peripherals> GCodeCommand<P> for G91
{
	fn prepare(&mut self, _: &mut Printer3DComponents<P>, g_code_executer: &mut GCodeExecuter<P>) -> Status
	{
		g_code_executer.set_position_mode(PositionMode::Relative);

		Status::Finished
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct G92
{
	pub x: Param<identifier::X, Distance>,
	pub y: Param<identifier::Y, Distance>,
	pub z: Param<identifier::Z, Distance>,
	pub e: Param<identifier::E, Distance>,
}
impl<P: Peripherals> GCodeCommand<P> for G92
{
	fn execute(&mut self, _: &mut Printer3DComponents<P>, _: &mut GCodeExecuter<P>) -> Status
	{
		Status::Finished
	}

	fn prepare(&mut self, printer_components: &mut Printer3DComponents<P>, _: &mut GCodeExecuter<P>) -> Status
	{
		printer_components
			.motion_controller
			.set_position(self.x.value, self.y.value, self.z.value, self.e.value);

		Status::Finished
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct M82;
impl<P: Peripherals> GCodeCommand<P> for M82
{
	fn execute(&mut self, _: &mut Printer3DComponents<P>, g_code_executer: &mut GCodeExecuter<P>) -> Status
	{
		g_code_executer.set_extruder_position_mode(PositionMode::Absolute);

		Status::Finished
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct M83;
impl<P: Peripherals> GCodeCommand<P> for M83
{
	fn execute(&mut self, _: &mut Printer3DComponents<P>, g_code_executer: &mut GCodeExecuter<P>) -> Status
	{
		g_code_executer.set_extruder_position_mode(PositionMode::Relative);

		Status::Finished
	}
}

fn set_target_temperature<CHP: PwmPin, TADC: Adc, TP: AdcPin<TADC>>(
	temperature: &Option<u16>, pid_controller: &mut TemperaturePidController<CHP, TADC, TP>,
) -> Status
{
	if let Some(target_temperature) = temperature
	{
		let target_temperature = Temperature::from_celsius(*target_temperature as f32);
		pid_controller.set_target_temperature(target_temperature);
	}

	Status::Finished
}

fn wait_for_target_temperature<CHP: PwmPin, TADC: Adc, TP: AdcPin<TADC>>(
	pid_controller: &mut TemperaturePidController<CHP, TADC, TP>, adc: &mut TADC,
	target_temperature_cooling_and_heating: &Param<identifier::R, u16>,
) -> Status
{
	const ACCEPTABLE_TEMPERATURE_RANGE: Temperature = Temperature::from_kelvin(3.);

	match pid_controller.get_current_temperature(adc)
	{
		Ok(current_temperature) =>
		{
			let target_temperature = pid_controller.get_target_temperature();
			if current_temperature < target_temperature
				|| (target_temperature_cooling_and_heating.value.is_some()
					&& current_temperature > target_temperature + ACCEPTABLE_TEMPERATURE_RANGE)
			{
				Status::Working
			}
			else
			{
				Status::Finished
			}
		},
		Err(error) => Status::Error(format!("Read temperature error: {:#?}", error)),
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct M104
{
	pub target_temperature: Param<identifier::S, u16>,
}
impl<P: Peripherals> GCodeCommand<P> for M104
{
	fn execute(&mut self, printer_components: &mut Printer3DComponents<P>, _: &mut GCodeExecuter<P>) -> Status
	{
		set_target_temperature(
			&self.target_temperature.value,
			&mut printer_components.hotend_pid_controller,
		)
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct M109
{
	pub target_temperature_heating: Param<identifier::S, u16>,
	pub target_temperature_cooling_and_heating: Param<identifier::R, u16>,
}
impl<P: Peripherals> GCodeCommand<P> for M109
{
	fn execute(
		&mut self, printer_components: &mut Printer3DComponents<P>, g_code_executer: &mut GCodeExecuter<P>,
	) -> Status
	{
		M104 {
			target_temperature: self
				.target_temperature_heating
				.value
				.or(self.target_temperature_cooling_and_heating.value)
				.into(),
		}
		.execute(printer_components, g_code_executer);

		wait_for_target_temperature(
			&mut printer_components.hotend_pid_controller,
			&mut printer_components.adc,
			&self.target_temperature_cooling_and_heating,
		)
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct M140
{
	pub target_temperature: Param<identifier::S, u16>,
}
impl<P: Peripherals> GCodeCommand<P> for M140
{
	fn execute(&mut self, printer_components: &mut Printer3DComponents<P>, _: &mut GCodeExecuter<P>) -> Status
	{
		set_target_temperature(
			&self.target_temperature.value,
			&mut printer_components.heated_bed_pid_controller,
		)
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct M190
{
	pub target_temperature_heating: Param<identifier::S, u16>,
	pub target_temperature_cooling_and_heating: Param<identifier::R, u16>,
}
impl<P: Peripherals> GCodeCommand<P> for M190
{
	fn execute(
		&mut self, printer_components: &mut Printer3DComponents<P>, g_code_executer: &mut GCodeExecuter<P>,
	) -> Status
	{
		M140 {
			target_temperature: self
				.target_temperature_heating
				.value
				.or(self.target_temperature_cooling_and_heating.value)
				.into(),
		}
		.execute(printer_components, g_code_executer);

		wait_for_target_temperature(
			&mut printer_components.heated_bed_pid_controller,
			&mut printer_components.adc,
			&self.target_temperature_cooling_and_heating,
		)
	}
}

const DEFAULT_FAN_INDEX: u8 = 0;
fn get_fan_with_index<P: Peripherals>(
	fan_index: Param<identifier::P, u8>, printer_components: &mut Printer3DComponents<P>,
) -> Result<&mut Fan<P::FanPin>, Status>
{
	match fan_index.value.unwrap_or(DEFAULT_FAN_INDEX)
	{
		0 => Ok(&mut printer_components.layer_fan),
		1 => Ok(&mut printer_components.hotend_fan),
		_ => Err(Status::Error(format!(
			"The fan with index {} is not supported by this firmware",
			fan_index.value.unwrap_or(DEFAULT_FAN_INDEX)
		))),
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct M106
{
	pub fan_index: Param<identifier::P, u8>,
	pub fan_speed: Param<identifier::S, u8>,
}
impl<P: Peripherals> GCodeCommand<P> for M106
{
	fn execute(&mut self, printer_components: &mut Printer3DComponents<P>, _: &mut GCodeExecuter<P>) -> Status
	{
		let fan = match get_fan_with_index(self.fan_index, printer_components)
		{
			Ok(fan) => fan,
			Err(error) => return error,
		};

		let fan_speed = self.fan_speed.value.unwrap_or(255);

		match fan.set_speed(Percentage::from_0_to_1(fan_speed as f32 / 255.).unwrap())
		{
			Ok(_) => Status::Finished,
			Err(_) => Status::Error(format!(
				"Couldn't set the speed of the fan with index {}",
				self.fan_index.value.unwrap_or(DEFAULT_FAN_INDEX)
			)),
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct M107
{
	pub fan_index: Param<identifier::P, u8>,
}
impl<P: Peripherals> GCodeCommand<P> for M107
{
	fn execute(&mut self, printer_components: &mut Printer3DComponents<P>, _: &mut GCodeExecuter<P>) -> Status
	{
		let fan = match get_fan_with_index(self.fan_index, printer_components)
		{
			Ok(fan) => fan,
			Err(error) => return error,
		};

		match fan.set_speed(Percentage::ZERO)
		{
			Ok(_) => Status::Finished,
			Err(_) => Status::Error(format!(
				"Couldn't turn off the fan with index {}",
				self.fan_index.value.unwrap_or(DEFAULT_FAN_INDEX)
			)),
		}
	}
}
