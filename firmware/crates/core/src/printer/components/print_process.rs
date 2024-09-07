//! Check [`PrintProcess`].

use std::{
	fmt::Debug,
	string::FromUtf8Error,
	sync::atomic::{AtomicU16, Ordering},
	time::Duration,
};

use embedded_hal::spi::SpiDevice;

use super::{
	drivers::spi_flash_memory::FlashMemoryChip,
	file_system::{
		regions::{
			data::{FileReader, ReadError},
			metadata::FileId,
		},
		FileSystem,
	},
	g_code::{
		parser::{GCodeLine, GCodeParser},
		GCodeCommand,
	},
	Peripherals,
};

static COMMANDS_IN_BUFFER: AtomicU16 = AtomicU16::new(0);
pub fn set_commands_in_buffer_count(commands_count: u16)
{
	COMMANDS_IN_BUFFER.store(commands_count, Ordering::Relaxed);
}
pub fn get_commands_in_buffer_count() -> u16
{
	COMMANDS_IN_BUFFER.load(Ordering::Relaxed)
}

/// This struct controls the process of printing a file, by parsing the content of the
/// file to [`G-code commmands`].
///
/// [`G-code commmands`]: super::g_code::commands
pub struct PrintProcess<P: Peripherals>
{
	g_code_parser: GCodeParser,

	file_id_to_print: Option<FileId>,
	file_to_print_reader: Option<FileReader<P::FlashChip, P::FlashSpi>>,

	max_commands_in_buffer_before_reading_new: u16,

	g_code_to_execute: String,
	// This is taken from the GCode file
	estimated_duration_in_seconds: Option<u32>,
	print_start_time: Option<Duration>,
}

impl<P: Peripherals> PrintProcess<P>
{
	const ESTIMATED_TIME_PREFIX: &'static str = "TIME:";

	/// Returns an empty [`PrintProcess`].
	///
	/// Use [`Self::print_file`] to start the process of printing.
	pub fn new(max_commands_in_buffer_before_reading_new: u16) -> Self
	{
		log::info!(
			"Create the print process with this configuration: {}",
			max_commands_in_buffer_before_reading_new
		);
		Self {
			g_code_parser: GCodeParser::default(),
			file_id_to_print: None,
			file_to_print_reader: None,
			max_commands_in_buffer_before_reading_new,
			estimated_duration_in_seconds: None,
			print_start_time: None,
			g_code_to_execute: String::with_capacity(P::FlashChip::PAGE_SIZE as usize),
		}
	}

	/// Starts printing the file with the provided `file_id_to_print` file id.
	///
	/// # Warning
	/// You must call [`Self::tick`] to effectively make the print process progress.
	pub fn print_file(&mut self, file_id_to_print: FileId, current_time: Option<Duration>)
	{
		self.file_id_to_print = Some(file_id_to_print);
		self.print_start_time = current_time;
	}

	/// If a file is currently [`being printed`], calling this function will try to read new G-code commands
	/// from the file system that will be executed by the [`GCodeExecuter`].
	///
	/// If instead no file is being printed, calling this function will do nothing.
	///
	/// [`being printed`]: Self::print_file
	/// [`GCodeExecuter`]: super::g_code::execute::GCodeExecuter
	pub fn tick(
		&mut self, file_system: &mut FileSystem<P::FlashChip, P::FlashSpi>, commands_in_buffer: u16,
	) -> Result<PrintProcessOk<P>, PrintProcessError<P::FlashSpi>>
	{
		if let Some(file_id_to_print) = self.file_id_to_print
		{
			if self.file_to_print_reader.is_none()
			{
				self.file_to_print_reader = Some(
					file_system
						.read_file(file_id_to_print)
						.map_err(|_| PrintProcessError::CouldntOpenFileForRead)?,
				);
			}

			if commands_in_buffer < self.max_commands_in_buffer_before_reading_new
			{
				let start = self.g_code_to_execute.len();
				let mut read_lines = Vec::with_capacity(start + P::FlashChip::PAGE_SIZE as usize);
				read_lines.extend_from_slice(self.g_code_to_execute.as_bytes());
				read_lines.extend(core::iter::repeat(0).take(P::FlashChip::PAGE_SIZE as usize));

				let read_bytes_count = self
					.file_to_print_reader
					.as_mut()
					.unwrap()
					.read_data(file_system, &mut read_lines[start..])
					.map_err(PrintProcessError::SPIError)?;

				read_lines.truncate(start + read_bytes_count as usize);

				let read_lines =
					String::from_utf8(read_lines).map_err(|err| PrintProcessError::FileContainsInvalidUtf8(err))?;

				let is_last_line_finished =
					read_lines.ends_with("\n") || self.file_to_print_reader.as_mut().unwrap().has_reached_end_of_file();
				let mut read_commands = Vec::with_capacity(read_lines.len() / 25);
				let mut read_lines_iterator = read_lines.lines().enumerate().peekable();
				let mut read_lines_iterator_cloned = read_lines_iterator.clone();
				while let Some((line_number, line)) = read_lines_iterator.next()
				{
					if read_lines_iterator.peek().is_none()
					{
						if !is_last_line_finished
						{
							self.g_code_to_execute = line.to_string();
							break;
						}
					}
					match self.parse_line_to_execute(&line)
					{
						Ok(result) =>
						{
							if let Some(command) = result.command
							{
								read_commands.push(command);
							}
						},
						Err(_) =>
						{
							self.g_code_to_execute.clear();

							read_lines_iterator_cloned.for_each(|(line_number_cloned, line)| {
								if line_number != line_number_cloned
								{
									self.g_code_to_execute.push_str(line);
									self.g_code_to_execute.push('\n');
								}
							});
							// Remove the last new line if there wasn't one before
							if !is_last_line_finished
							{
								let _ = self.g_code_to_execute.pop();
							}

							return Err(PrintProcessError::CouldntParseLine(line.to_string()));
						},
					}
				}

				if self.file_to_print_reader.as_ref().unwrap().has_reached_end_of_file()
				{
					self.file_id_to_print = None;
					self.file_to_print_reader = None;
				}

				Ok(PrintProcessOk {
					read_lines: Some(read_lines),
					read_commands,
				})
			}
			else
			{
				Ok(PrintProcessOk {
					read_lines: None,
					read_commands: Vec::new(),
				})
			}
		}
		else
		{
			Ok(PrintProcessOk {
				read_lines: None,
				read_commands: Vec::new(),
			})
		}
	}

	/// Returns `Some(FileId)` if a file is currently being printed (which means you called [`Self::print_file`]
	/// and the file has not been completely read yet), otherwise returns `None`.
	pub fn get_file_being_printed(&self) -> Option<FileId>
	{
		self.file_id_to_print.clone()
	}

	/// Returns `Some(duration_in_secs)` if a file is currently being printed (which means you called [`Self::print_file`]
	/// and the file has not been completely read yet) and in the file there's a line containing `;TIME: {value}` where
	/// `{value}` is a number.
	/// Otherwise returns `None`.
	pub fn get_print_estimated_duration_in_seconds(&self) -> Option<u32>
	{
		self.estimated_duration_in_seconds.clone()
	}

	/// Returns `Some(print_start_time)` if a file is currently being printed (which means you called [`Self::print_file`]
	/// and the file has not been completely read yet) and you provided a start time for the print.
	/// Otherwise returns `None`.
	pub fn get_print_start_time(&self) -> Option<Duration>
	{
		self.print_start_time.clone()
	}

	/// Returns the command and the comment present in the provided `line` (if they are present),
	/// or `Err(())` if the line is a [`GCodeLine::Error`].
	pub fn parse_line_to_execute<'a>(&mut self, line: &'a str) -> Result<LineToExecuteParsed<'a, P>, ()>
	{
		match self.g_code_parser.parse_line(&line)
		{
			GCodeLine::Empty => Ok(LineToExecuteParsed {
				comment: None,
				command: None,
			}),
			GCodeLine::Command(command) => Ok(LineToExecuteParsed {
				comment: None,
				command: Some(command),
			}),
			GCodeLine::Comment(comment) =>
			{
				if self.estimated_duration_in_seconds.is_none() && comment.starts_with(Self::ESTIMATED_TIME_PREFIX)
				{
					let duration_string = &comment[Self::ESTIMATED_TIME_PREFIX.len()..];
					self.estimated_duration_in_seconds = duration_string.parse::<u32>().ok();
				}

				Ok(LineToExecuteParsed {
					comment: Some(comment),
					command: None,
				})
			},
			GCodeLine::CommandAndComment(command, comment) => Ok(LineToExecuteParsed {
				comment: Some(comment),
				command: Some(command),
			}),
			GCodeLine::Error => Err(()),
		}
	}
}

/// The call to [`PrintProcess::tick`] has been successful, and this struct contains the string
/// that has been read from the flash memory ([`Self::read_lines`]) and also the result of parsing
/// that string to GCodeCommands (in [`Self::read_commands`]).
pub struct PrintProcessOk<P: Peripherals>
{
	pub read_lines: Option<String>,
	pub read_commands: Vec<Box<dyn GCodeCommand<P>>>,
}

/// The call to [`PrintProcess::tick`] hasn't been successful. This enum contains the problems that
/// could have arised.
pub enum PrintProcessError<Spi: SpiDevice<u8>>
{
	/// Check [`FileSystem::read_file`].
	CouldntOpenFileForRead,

	/// The file contains some characters that are not [`utf-8`].
	///
	/// [`utf-8`]: <https://en.wikipedia.org/wiki/UTF-8>
	FileContainsInvalidUtf8(FromUtf8Error),

	/// Check [`FileReader::read_data`].
	SPIError(ReadError<Spi>),

	/// One of the lines read from the file is a [`GCodeLine::Error`].
	CouldntParseLine(String),
}

pub struct LineToExecuteParsed<'a, P: Peripherals>
{
	pub comment: Option<&'a str>,
	pub command: Option<Box<dyn GCodeCommand<P>>>,
}

impl<Spi: SpiDevice<u8>> Debug for PrintProcessError<Spi>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Self::CouldntOpenFileForRead => write!(f, "CouldntOpenFileForRead"),
			Self::FileContainsInvalidUtf8(arg0) => f.debug_tuple("FileContainsInvalidUtf8").field(arg0).finish(),
			Self::SPIError(arg0) => f.debug_tuple("SPIError").field(arg0).finish(),
			Self::CouldntParseLine(line) => f.debug_struct("CoudlntParseLine").field("line", line).finish(),
		}
	}
}
