//! This module defines the possible HTTP requests that the firmware can handle.
//! Each request is associated with an HTTP method, a URI, and a callback function
//! that processes the request. The requests cover various functionalities related
//! to the 3D printer, such as managing files, controlling print jobs, and
//! retrieving printer status.
//
//! The module provides:
//!
//! - An enum `HttpRequest` that lists all supported API endpoints and their
//!   corresponding HTTP methods and URIs.
//! - Functions to retrieve the method, URI, and callback for each request,
//!   facilitating interaction with the HTTP server.

mod callbacks;

use embedded_svc::http::{
	server::{Connection, HandlerResult, Request},
	Method,
};
use strum::{EnumCount, EnumIter};

use super::resources::Resources;
use crate::printer::components::Peripherals;

pub const STACK_SIZE: usize = 30_000;

#[derive(EnumIter, EnumCount, Clone, Copy)]
/// A possible request that the HTTP server in this firmware can handle. Each request has a [`method`], an [`URI`]
/// and a [`callback function`] that handles it.
///
/// [`method`]: Self::get_method
/// [`URI`]: Self::get_uri
/// [`callback function`]: Self::get_callback
pub enum HttpRequest
{
	/// List the metadatas of all the G-code files saved in the file system.
	ListFiles,
	/// Delete a specific file from the file system.
	DeleteFile,
	/// Start printing a specific file.
	PrintFile,
	/// Send a G-code file to the printer (that later on could be [`printed`](Self::PrintFile)).
	SendFile,
	/// Get the status of the current print (if a print is in execution), providing some info like the expected duration,
	/// the name of the file being printed...
	GetPrintStatus,
	/// Pause or resume (based on the previous state) the current print.
	PauseOrResume,
	/// Get the status of various components of the machine (like the current temperature of the hotend, or the target
	/// temperature of the bed).
	PrinterState,
	/// Moves the tool of the machine by the specified amount on the possible directions.
	Move,
	/// Provide the history of G-code commands that have been read from the file system to print the current file.
	/// This request must be issued often, since microcontrollers don't have much RAM (which means that the history
	/// being tracked can't be long), and a large amount of time between different requests might lead to some G-code
	/// commands that have been executed by the printer to not appear in the history.
	ListGCodeCommandsInMemory,
	/// "Manually" add the provided G-code commands to the command buffer of the current print process.
	SendGCodeCommands,
	/// Over-The-Air update.
	OTAUpdate,
}

type Callback<C, P> = fn(Request<&mut C>, Resources<P>) -> HandlerResult;

impl HttpRequest
{
	/// Returns the HTTP method that the request received by the server should have to invoke the [`callback`]
	/// of this variant of the enum.
	///
	/// [`callback`]: Self::get_callback
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
			HttpRequest::OTAUpdate => Method::Post,
		}
	}

	/// Returns the relative URI that the request received by the server should have to invoke the [`callback`]
	/// of this variant of the enum.
	///
	/// [`callback`]: Self::get_callback
	pub fn get_uri(&self) -> &'static str
	{
		match self
		{
			HttpRequest::ListFiles => "/list-files",
			HttpRequest::DeleteFile => "/delete-file",
			HttpRequest::PrintFile => "/print-file",
			HttpRequest::SendFile => "/send-file",
			HttpRequest::GetPrintStatus => "/print-status",
			HttpRequest::PauseOrResume => "/pause-or-resume",
			HttpRequest::PrinterState => "/printer-state",
			HttpRequest::Move => "/move",
			HttpRequest::ListGCodeCommandsInMemory => "/list-gcode-commands-in-memory",
			HttpRequest::SendGCodeCommands => "/send-gcode-commands",
			HttpRequest::OTAUpdate => "/ota-update",
		}
	}

	/// Returns a pointer to the function that should be called when the server receives a request
	/// with the corresponding [`URI`] and [`method`].
	///
	/// [`URI`]: Self::get_uri
	/// [`method`]: Self::get_method
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
			HttpRequest::OTAUpdate => callbacks::ota_update,
		}
	}
}

/// Returns the number of possible URI the HTTP server of this firmware can handle (which corresponds to
/// the number of variants of the [`HttpRequest`] enum).
pub const fn http_request_handlers_count() -> usize
{
	HttpRequest::COUNT
}
