use std::fmt::Debug;

pub trait InterruptPin
{
	type Error: Debug;

	/// # Safety
	/// The `callback` will be called in an ISR context.
	unsafe fn subscribe_to_interrupt(
		&mut self, when_to_trigger: Trigger, callback: impl FnMut() + 'static,
	) -> Result<(), Self::Error>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Trigger
{
	PositiveEdge,
	NegativeEdge,
	AnyEdge,
	LowLevel,
	HighLevel,
}
