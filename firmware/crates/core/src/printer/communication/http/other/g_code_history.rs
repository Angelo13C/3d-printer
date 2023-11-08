use core::ops::Range;

/// This struct keeps track of the G-code commands executed by the printer (basically the ones read from the file system).
///
/// This is useful because the history of commands could be displayed in the control interface program.
pub struct GCodeHistory
{
	history: String,
	history_lines_bounds: Range<u32>,
}

impl GCodeHistory
{
	/// Returns an empty [`GCodeHistory`].
	///
	/// # Examples
	/// ```
	/// # use firmware_core::printer::communication::http::other::*;
	/// #
	/// let history = GCodeHistory::new();
	///
	/// for i in 0..10_000
	/// {
	/// 	assert_eq!(history.get_lines_from_history(i), "");
	/// }
	/// ```
	pub fn new() -> Self
	{
		Self {
			history: String::new(),
			history_lines_bounds: 0..0,
		}
	}

	/// Add the provided `lines` to the history of commands. You can later retrieve this string using
	/// [`Self::get_lines_from_history`].
	///
	/// # Warning
	/// Due to the fact that microcontrollers have a low amount of RAM, the old lines you provided in the
	/// last call to this function will be replaces by these new ones.
	/// The control interface program should periodically poll this history to avoid missing some lines.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::printer::communication::http::other::*;
	/// #
	/// let mut history = GCodeHistory::new();
	///
	/// history.add_read_lines("G1 X0".to_string());
	/// assert_eq!(history.get_lines_from_history(0), "G1 X0");
	/// history.add_read_lines("G0 Y10".to_string());
	/// assert!(history.get_lines_from_history(0) != "G1 X0");
	/// ```
	pub fn add_read_lines(&mut self, lines: String)
	{
		let lines_count = self.history.lines().count() as u32;
		self.history_lines_bounds = self.history_lines_bounds.end..(self.history_lines_bounds.end + lines_count);

		self.history = lines;
	}

	/// Returns a string containing the `lines` you provided to [`Self::add_read_lines`] whose number is greater
	/// than `start_line_number`.
	///
	/// # Examples
	/// ```
	/// # use firmware_core::printer::communication::http::other::*;
	/// #
	/// let mut history = GCodeHistory::new();
	///
	/// history.add_read_lines("G1 X0\nG1 Y10".to_string());
	/// assert_eq!(history.get_lines_from_history(0), "G1 X0\nG1 Y10");
	/// assert_eq!(history.get_lines_from_history(1), "G1 Y10");
	/// ```
	pub fn get_lines_from_history(&self, start_line_number: u32) -> &str
	{
		let line_offset = start_line_number.saturating_sub(self.history_lines_bounds.start);
		let character_offset = match line_offset == 0
		{
			true => 0,
			false =>
			{
				if let Some((index, _)) = self.history.match_indices("\n").nth(line_offset as usize - 1)
				{
					index + 1
				}
				else
				{
					return "";
				}
			},
		};

		&self.history[character_offset..]
	}
}
