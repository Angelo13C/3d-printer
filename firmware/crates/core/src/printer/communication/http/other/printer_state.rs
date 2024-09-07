use serde::{ser::SerializeStruct, Serialize};
use spin::Mutex;

use crate::{
	printer::components::{temperature::TemperaturePidController, Peripherals},
	utils::measurement::temperature::Temperature,
};

static PRINTER_STATE: Mutex<PrinterState> = Mutex::new(PrinterState::EMPTY);

/// Returns the static [`PrinterState`] instance that contains the state of the machine when you last called the [`tick`]
/// function.
pub fn get_current_state() -> PrinterState
{
	PRINTER_STATE.lock().clone()
}

/// Updates the "screenshot" of the state of some components of the machine based on the current and target temperatures
/// of the two PID controllers (saves the result in the static instance of [`PrinterState`] so that you can later retrieve
/// the state using [`get_current_state`]).
pub fn tick<P: Peripherals>(
	hotend_pid_controller: &TemperaturePidController<P::CartridgeHeaterPin, P::Adc, P::HotendAdcPin>,
	bed_pid_controller: &TemperaturePidController<P::HeatedBedHeaterPin, P::Adc, P::HeatedBedAdcPin>,
)
{
	PRINTER_STATE
		.lock()
		.tick::<P>(hotend_pid_controller, bed_pid_controller)
}

#[derive(Clone)]
/// A "screenshot" of the state of some components of the machine.
pub struct PrinterState
{
	hotend_current_temperature: Option<Temperature>,
	hotend_target_temperature: Option<Temperature>,
	bed_current_temperature: Option<Temperature>,
	bed_target_temperature: Option<Temperature>,
}

impl PrinterState
{
	const EMPTY: Self = Self {
		hotend_current_temperature: None,
		hotend_target_temperature: None,
		bed_current_temperature: None,
		bed_target_temperature: None,
	};

	/// Updates the "screenshot" of the state of some components of the machine based on the current and target temperatures
	/// of the two PID controllers.
	fn tick<P: Peripherals>(
		&mut self, hotend_pid_controller: &TemperaturePidController<P::CartridgeHeaterPin, P::Adc, P::HotendAdcPin>,
		bed_pid_controller: &TemperaturePidController<P::HeatedBedHeaterPin, P::Adc, P::HeatedBedAdcPin>,
	)
	{
		if let Some(hotend_current_temperature) = hotend_pid_controller.get_last_sample_of_current_temperature()
		{
			self.hotend_current_temperature = Some(hotend_current_temperature);
		}
		self.hotend_target_temperature = Some(
			hotend_pid_controller
				.get_target_temperature()
				.unwrap_or(Temperature::from_kelvin(0.)),
		);

		if let Some(bed_current_temperature) = bed_pid_controller.get_last_sample_of_current_temperature()
		{
			self.bed_current_temperature = Some(bed_current_temperature);
		}
		self.bed_target_temperature = Some(
			bed_pid_controller
				.get_target_temperature()
				.unwrap_or(Temperature::from_kelvin(0.)),
		);
	}
}

impl Serialize for PrinterState
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: serde::Serializer
	{
		let mut state = serializer.serialize_struct("PrinterState", 4)?;
		let mut serialize_field = |field_name, field_value: Option<Temperature>| {
			state.serialize_field(
				field_name,
				&field_value.map(|temperature| temperature.as_kelvin()).unwrap_or(-1.),
			)
		};

		(serialize_field)("hotendCurrentTemperature", self.hotend_current_temperature)?;
		(serialize_field)("hotendTargetTemperature", self.hotend_target_temperature)?;
		(serialize_field)("bedCurrentTemperature", self.bed_current_temperature)?;
		(serialize_field)("bedTargetTemperature", self.bed_target_temperature)?;

		state.end()
	}
}
