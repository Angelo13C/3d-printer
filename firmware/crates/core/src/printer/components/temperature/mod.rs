//! A module for managing temperature control in the 3D printer.
//!
//! This module includes components for temperature control, including PID (Proportional-Integral-Derivative)
//! controllers and safety mechanisms to ensure safe operating temperatures. It provides the ability to adjust
//! temperature settings, monitor temperature readings, and handle temperature-related errors.

mod pid;
pub mod safety;

pub use pid::{
	PidController as TemperaturePidController, PidGains as TemperaturePidGains, TickError as PidUpdateError,
};
