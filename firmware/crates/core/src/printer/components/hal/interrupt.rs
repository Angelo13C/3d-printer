//! Interrupt module.
//!
//! This module defines a trait for interrupt pins and an enumeration to specify
//! the triggering conditions for interrupts. It allows subscribing to interrupts
//! on specific pins and handling them through callback functions.

use std::fmt::Debug;

/// Trait representing an interrupt-capable pin.
pub trait InterruptPin
{
	/// The type of error that can occur when subscribing to an interrupt.
	type Error: Debug;

	/// Subscribes to an interrupt on this pin with a specified trigger condition.
	///
	/// # Safety
	/// The `callback` will be called in an Interrupt Service Routine (ISR) context.
	///
	/// This means that the callback should be lightweight and should not perform
	/// operations that could block or take a long time. Care must also be taken
	/// to ensure that shared resources are accessed in a thread-safe manner.
	unsafe fn subscribe_to_interrupt(
		&mut self, when_to_trigger: Trigger, callback: impl FnMut() + Send + 'static,
	) -> Result<(), Self::Error>;
}

/// Enumeration representing the conditions under which an interrupt can be triggered.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Trigger
{
	/// Trigger on the positive edge (rising edge) of the signal.
	PositiveEdge,
	/// Trigger on the negative edge (falling edge) of the signal.
	NegativeEdge,
	/// Trigger on any edge (both rising and falling) of the signal.
	AnyEdge,
	/// Trigger when the signal is low.
	LowLevel,
	/// Trigger when the signal is high.
	HighLevel,
}
