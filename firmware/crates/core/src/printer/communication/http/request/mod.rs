mod callbacks;

use embedded_hal::spi::SpiDevice;
use embedded_svc::http::{
	server::{Connection, HandlerResult, Request},
	Method,
};
use strum::EnumIter;

use super::resources::Resources;
use crate::printer::components::{drivers::spi_flash_memory::FlashMemoryChip, Peripherals};

#[derive(EnumIter, Clone, Copy)]
pub enum HttpRequest
{
	ListFiles,
	DeleteFile,
	PrintFile,
	SendFile,
	GetPrintStatus,
	PauseOrResume,
	PrinterState,
	Move,
	ListGCodeCommandsInMemory,
	SendGCodeCommands,
}

type Callback<C, P> = fn(Request<&mut C>, Resources<P>) -> HandlerResult;

impl HttpRequest
{
	pub fn get_method(&self) -> Method
	{
		match self
		{
			HttpRequest::ListFiles => Method::Get,
			HttpRequest::DeleteFile => Method::Delete,
			HttpRequest::PrintFile => Method::Post,
			HttpRequest::SendFile => Method::Post,
			HttpRequest::GetPrintStatus => Method::Get,
			HttpRequest::PauseOrResume => Method::Post,
			HttpRequest::PrinterState => Method::Get,
			HttpRequest::Move => Method::Post,
			HttpRequest::ListGCodeCommandsInMemory => Method::Get,
			HttpRequest::SendGCodeCommands => Method::Post,
		}
	}

	pub fn get_uri(&self) -> &'static str
	{
		match self
		{
			HttpRequest::ListFiles => "list-files",
			HttpRequest::DeleteFile => "delete-file",
			HttpRequest::PrintFile => "print-file",
			HttpRequest::SendFile => "send-file",
			HttpRequest::GetPrintStatus => "print-status",
			HttpRequest::PauseOrResume => "pause-or-resume",
			HttpRequest::PrinterState => "printer-state",
			HttpRequest::Move => "move",
			HttpRequest::ListGCodeCommandsInMemory => "list-gcode-commands-in-memory",
			HttpRequest::SendGCodeCommands => "send-gcode-commands",
		}
	}

	pub fn get_callback<C: Connection, P: Peripherals>(&self) -> Callback<C, P>
	{
		match self
		{
			HttpRequest::ListFiles => callbacks::list_files,
			HttpRequest::DeleteFile => callbacks::delete_file,
			HttpRequest::PrintFile => callbacks::print_file,
			HttpRequest::SendFile => callbacks::send_file,
			HttpRequest::GetPrintStatus => callbacks::get_print_status,
			HttpRequest::PauseOrResume => callbacks::pause_or_resume,
			HttpRequest::PrinterState => callbacks::printer_state,
			HttpRequest::Move => callbacks::move_,
			HttpRequest::ListGCodeCommandsInMemory => callbacks::list_g_code_commands_in_memory,
			HttpRequest::SendGCodeCommands => callbacks::send_g_code_commands,
		}
	}
}
