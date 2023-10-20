use std::{
	cell::UnsafeCell,
	ops::{Deref, DerefMut},
	sync::atomic::{AtomicBool, Ordering},
};

const LOCKED: bool = true;
const UNLOCKED: bool = !LOCKED;

/// Simple implementation of a [`mutual exclusion`] primitive to protect shared data.
///
/// [`mutual exclusion`]: https://en.wikipedia.org/wiki/Mutual_exclusion
pub struct Mutex<T>
{
	data: UnsafeCell<T>,
	locked: AtomicBool,
}

impl<T> Mutex<T>
{
	/// Returns a new mutex in an unlocked state.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::mutex::*;
	/// #
	/// let value = 5;
	/// let mutex = Mutex::new(value);
	/// assert_eq!(mutex.try_lock().map(|v| *v), Some(value));
	/// ```
	pub const fn new(value: T) -> Self
	{
		Self {
			data: UnsafeCell::new(value),
			locked: AtomicBool::new(UNLOCKED),
		}
	}

	/// If the mutex is unlocked returns `Some(MutexGuard<T>)` with the protected value and locks the mutex.
	/// Otherwise returns `None`.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::mutex::*;
	/// #
	/// let value = 5;
	/// let mutex = Mutex::new(value);
	///
	/// {
	///     let protected_value = mutex.try_lock();
	///     assert!(mutex.try_lock().is_none());
	/// }
	///
	/// assert!(mutex.try_lock().is_some());
	/// ```
	pub fn try_lock(&self) -> Option<MutexGuard<'_, T>>
	{
		if self.locked.swap(LOCKED, Ordering::Relaxed) == UNLOCKED
		{
			Some(MutexGuard(self))
		}
		else
		{
			None
		}
	}

	/// Blocks the thread until the mutex is unlocked and returns a [`MutexGuard`] with the protected value.
	///
	/// Locks the mutex.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::utils::mutex::*;
	/// #
	/// let mutex = Mutex::new(7);
	/// {
	///     let mut protected_value = mutex.lock();
	///     assert_eq!(*protected_value, 7);
	///
	///     *protected_value = 4;
	/// }
	/// assert_eq!(*mutex.lock(), 4);
	/// ```
	pub fn lock(&self) -> MutexGuard<'_, T>
	{
		loop
		{
			if let Some(guard) = self.try_lock()
			{
				return guard;
			}
		}
	}
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

/// Check [`Mutex`].
///
/// When this struct is dropped the original mutex is unlocked.
pub struct MutexGuard<'a, T>(&'a Mutex<T>);
impl<T> Deref for MutexGuard<'_, T>
{
	type Target = T;

	fn deref(&self) -> &T
	{
		unsafe { &*self.0.data.get() }
	}
}

impl<T> DerefMut for MutexGuard<'_, T>
{
	fn deref_mut(&mut self) -> &mut T
	{
		unsafe { &mut *self.0.data.get() }
	}
}

impl<'a, T> Drop for MutexGuard<'a, T>
{
	fn drop(&mut self)
	{
		self.0.locked.store(UNLOCKED, Ordering::Relaxed);
	}
}
