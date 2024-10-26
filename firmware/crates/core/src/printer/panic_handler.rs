//! A module for handling panic events in the firmware.
//!
//! This module defines a [`PanicHandler`] struct that encapsulates a function pointer.
//! When a panic occurs, this function will be called to handle the situation,
//! such as safely shutting down heaters to prevent hazards.
//!
//! The provided function should be safe to call even in an unsafe state since
//! it will be executed when the program is in a panic state, and recovery
//! is not possible.

/// A struct containing a function pointer that will be called when the firmware panics.
///
/// The function pointer should attempt to shut down all heaters, even in an unsafe way,
/// as it will be executed during a panic, indicating that the state of the program
/// is already unrecoverable.
pub struct PanicHandler(pub unsafe fn());

/// Registers a panic handler to be called when a panic occurs.
///
/// This function sets the panic hook to call the provided `PanicHandler`'s function
/// pointer, allowing for cleanup and safe shutdown of critical components before
/// the panic message is printed.
///
/// # Arguments
///
/// * `panic_handler` - The `PanicHandler` instance containing the function to call on panic.
pub(super) fn register_panic_handler(panic_handler: PanicHandler)
{
	std::panic::set_hook(Box::new(move |info| {
		unsafe { (panic_handler.0)() }

		println!("PANIC: {}", info);
	}))
}
