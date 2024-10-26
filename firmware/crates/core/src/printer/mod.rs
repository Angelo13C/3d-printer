//! A module for managing a 3D printer's components, communication, and operations.
//!
//! This module encapsulates the functionality of a 3D printer, including
//! its components and communication methods, as well as managing
//! the watchdog timer for safety. It provides the main interface to
//! interact with the printer, including initialization and periodic
//! updates via the `tick` method.
//!
//! # Structure
//!
//! - [`Printer3D`]: The main structure representing the 3D printer.
//! - [`CreationError`]: Errors that may occur during the creation of a printer instance.
//! - [`TickError`]: Errors that may occur during the ticking process.

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

/// A structure representing a 3D printer.
///
/// This struct manages the printer's components, communication protocols,
/// and the watchdog timer for monitoring the printer's operation.
/// It provides methods to initialize the printer and perform periodic updates.
pub struct Printer3D<P: Peripherals + 'static>
{
	components: Printer3DComponents<SendablePeripherals<P>>,
	communication: MultiThreadCommunication<P>,
	watchdog: Option<<P::WatchdogCreator as WatchdogCreator>::Watchdog>,
}

impl<P: Peripherals + 'static> Printer3D<P>
{
	/// Creates a new instance of `Printer3D`.
	///
	/// This method initializes the printer's components and
	/// communication systems, setting up any necessary peripherals.
	///
	/// # Arguments
	///
	/// * `peripherals` - The hardware peripherals for the printer.
	/// * `components_config` - Configuration settings for the printer's components.
	/// * `communication_config` - Configuration settings for the communication system.
	/// * `panic_handler` - A handler to register for panic events.
	///
	/// # Returns
	///
	/// A `Result` containing the initialized `Printer3D` instance or a `CreationError`.
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

	/// Updates the printer state, feeding the watchdog if present.
	///
	/// This method should be called periodically to ensure the printer
	/// operates correctly and to maintain communication with its components.
	///
	/// # Returns
	///
	/// A `Result` indicating success or a `TickError` if an error occurs.
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

/// Errors that may occur during the creation of a `Printer3D`.
#[derive(Debug)]
pub enum CreationError<P: Peripherals>
{
	Components(components::CreationError<P::StepperTickerTimer, P::ZAxisEndstop, P::UartDriver>),
	Communication(std::io::Error),
}

/// Errors that may occur during the ticking process of a `Printer3D`.
pub enum TickError<P: Peripherals>
{
	WatchdogReset(<<P::WatchdogCreator as WatchdogCreator>::Watchdog as Watchdog>::Error),
	Components(components::TickError<P::ZAxisEndstop, P::UartDriver, P::StepperTickerTimer>),
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
