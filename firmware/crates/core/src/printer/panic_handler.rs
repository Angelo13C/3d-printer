/// This struct contains a [`function pointer`] that will be called when the firmware [`panics`].
///
/// The function pointer should try to shutdown all the heaters, even in an unsafe way since it will be called only during a panic
/// and so the state of the program is already unrecovarable.
///
/// [`function pointer`]: fn
/// [`panics`]: panic
pub struct PanicHandler(pub unsafe fn());

pub(super) fn register_panic_handler(panic_handler: PanicHandler)
{
	std::panic::set_hook(Box::new(move |info| {
		unsafe { (panic_handler.0)() }

		println!("PANIC: {}", info);
	}))
}
