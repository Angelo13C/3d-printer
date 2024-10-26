//! UART module.
//!
//! This module provides a trait for communication using the [`UART protocol`].
//!
//! [`UART protocol`]: <https://en.wikipedia.org/wiki/UART>

use std::fmt::Debug;

use crate::utils::measurement::duration::SmallDuration;

/// A type that can be used to communicate using the [`UART protocol`].
///
/// [`UART protocol`]: <https://en.wikipedia.org/wiki/UART>
pub trait Uart
{
	/// The type of error that can occur while using the UART interface.
	type Error: Debug;

	/// Tries to read `buf.len()` bytes from the interface and writes them into `buf`.
	/// If there are not enough readable bytes and the `timeout` expires, the function will return
	/// without having read all the `buf.len()` bytes.
	///
	/// Returns `Ok(read_bytes_count)` if `read_bytes_count` were successfully read from the interface,
	/// otherwise returns `Err(Self::Error)`.
	fn read(&mut self, buf: &mut [u8], timeout: SmallDuration) -> Result<usize, Self::Error>;

	/// Tries to write `buf` to the interface.
	///
	/// Returns `Ok(written_bytes_count)` if `written_bytes_count` were successfully written to the interface,
	/// otherwise returns `Err(Self::Error)`.
	fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error>;

	/// Discards all the bytes in the read buffer.
	///
	/// Returns `Ok(())` if the operation was successful, otherwise returns `Err(Self::Error)`.
	fn flush_read(&mut self) -> Result<(), Self::Error>;
}
