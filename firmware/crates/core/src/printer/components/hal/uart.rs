use crate::utils::measurement::duration::SmallDuration;

/// A type that can be used to communicate using the [`UART protocol`].
///
/// [`UART protocol`]: <https://it.wikipedia.org/wiki/UART>
pub trait Uart
{
	type Error;

	/// Try to read `buf.len()` bytes from the interface and write them in `buf`.
	/// If there are not enough readable bytes and the `timeout` expires, the function will return
	/// without having read all the `buf.len()` bytes.
	///
	/// Returns `Ok(read_bytes_count)` if `read_bytes_count` were successfully read from the interface,
	/// otherwise returns `Err(Self::Error)`.
	fn read(&mut self, buf: &mut [u8], timeout: SmallDuration) -> Result<usize, Self::Error>;

	/// Try to write `buf` in the interface.
	///
	/// Returns `Ok(written_bytes_count)` if `written_bytes_count` were successfully written in the interface,
	/// otherwise returns `Err(Self::Error)`;
	fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error>;

	/// Discards all the bytes in the read buffer.
	///
	/// Returns `Ok(())` if the operation was succesful, otherwise returns `Err(Self::Error)`.
	fn flush_read(&mut self) -> Result<(), Self::Error>;
}
