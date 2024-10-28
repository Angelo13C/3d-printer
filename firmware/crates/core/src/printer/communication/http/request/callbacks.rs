use embedded_svc::http::server::{Connection, HandlerError, Request, Response};
use serde::{Deserialize, Serialize};
use spin::MutexGuard;

use crate::printer::{
	communication::http::{
		command::Command,
		other::printer_state,
		resources::{Resources, ResourcesImpl},
	},
	components::{file_system::regions::metadata::FileId, pauser, time::SystemTime, Peripherals},
};

const SER_BUFFER_SIZE: usize = super::STACK_SIZE / 3;

macro_rules! deserialize_request {
	(BUFFER_SIZE = $buffer_size: expr, CALLBACK = $callback_name: expr, $type: ty, $request: expr) => {{
		let mut buffer = [0; $buffer_size];
		let read_bytes_count = $request
			.read(&mut buffer)
			.map_err(|_| HandlerError::new(concat!("`", $callback_name, "` couldn't read the request")))?;

		let (value, _) = serde_json_core::from_slice::<$type>(&buffer[..read_bytes_count])
			.map_err(|_| HandlerError::new(concat!("`", $callback_name, "` couldn't parse the request")))?;
		value
	}};
}

macro_rules! send_response {
	(BUFFER_SIZE = $buffer_size: expr, CALLBACK = $callback_name: expr, $response: expr, $request: expr) => {{
		$request
			.write(&serde_json_core::ser::to_vec::<_, $buffer_size>(&$response)?)
			.map_err(|_| HandlerError::new(concat!("`", $callback_name, "` couldn't write the response")))?;
	}};
}

pub fn hello<C: Connection, P: Peripherals>(
	request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Start handling `hello` HTTP request");

	let mut response = ok_response(request)?;
	response.flush()?;

	log::info!("Successfully handled `hello` HTTP request");

	Ok(())
}

pub fn list_files<C: Connection, P: Peripherals>(
	mut request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Start handling `list-files` HTTP request");

	let mut resources = get_resources(&resources)?;
	let _ = check_security(&mut request, &mut resources)?;

	let mut file_system = &mut resources.file_system;
	let file_metadatas = Vec::from(file_system.get_existing_files_metadatas());

	#[derive(Serialize)]
	#[serde(rename_all = "camelCase")]
	struct HttpResponse
	{
		files: Vec<File>,
	}

	#[derive(Serialize)]
	#[serde(rename_all = "camelCase")]
	struct File
	{
		name: String,
		size_in_bytes: u32,
		file_id: u32,
	}
	let mut response_message = Vec::<File>::with_capacity(file_metadatas.len());

	for file_metadata in file_metadatas
	{
		if let Ok(mut file_reader) = file_system.read_file(file_metadata.id)
		{
			if let Ok(file_name) = file_reader.read_name(&mut file_system)
			{
				response_message.push(File {
					name: file_name,
					size_in_bytes: file_metadata.file_data_length + file_metadata.file_name_length,
					file_id: u32::from_le_bytes(file_metadata.id.to_bytes()),
				});
			}
		}
	}

	let response_message = HttpResponse {
		files: response_message,
	};

	let mut response = ok_response(request)?;
	send_response!(
		BUFFER_SIZE = SER_BUFFER_SIZE,
		CALLBACK = "list_files",
		response_message,
		response
	);

	log::info!("Successfully handled `list-files` HTTP request");

	Ok(())
}

pub fn options_list_files<C: Connection, P: Peripherals>(
	request: Request<&mut C>, _: Resources<P>,
) -> Result<(), HandlerError>
{
	options_callback(request, "list-files", "")
}

pub fn delete_file<C: Connection, P: Peripherals>(
	mut request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Start handling `delete-file` HTTP request");

	let mut resources = get_resources(&resources)?;
	let _ = check_security(&mut request, &mut resources)?;

	let file_system = &mut resources.file_system;

	#[derive(Deserialize)]
	#[serde(rename_all = "camelCase")]
	struct HttpRequest
	{
		file_id: u32,
	}
	let request = deserialize_request!(BUFFER_SIZE = 100, CALLBACK = "delete_file", HttpRequest, request);
	let file_id = FileId::from_bytes(request.file_id.to_le_bytes());

	file_system
		.delete_file(file_id)
		.map_err(|_| HandlerError::new("Unable to delete a file from the file system"))?;

	log::info!("Successfully handled `delete-file` HTTP request");

	Ok(())
}

pub fn print_file<C: Connection, P: Peripherals>(
	mut request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Start handling `print-file` HTTP request");

	let mut resources = get_resources(&resources)?;
	let _ = check_security(&mut request, &mut resources)?;

	#[derive(Deserialize)]
	#[serde(rename_all = "camelCase")]
	struct HttpRequest
	{
		file_id: u32,
	}
	let request = deserialize_request!(BUFFER_SIZE = 100, CALLBACK = "print_file", HttpRequest, request);
	let file_id = FileId::from_bytes(request.file_id.to_le_bytes());

	let current_time = resources.system_time.as_ref().map(|time| time.now());
	resources.print_process.print_file(file_id, current_time);

	log::info!("Successfully handled `print-file` HTTP request");

	Ok(())
}

pub fn send_file<C: Connection, P: Peripherals>(
	mut request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Start handling `send-file` HTTP request");

	let mut resources = get_resources(&resources)?;
	let _ = check_security(&mut request, &mut resources)?;

	let file_name = request
		.header("File-Name")
		.ok_or(HandlerError::new("The request doesn't have a `File-Name` header"))?;
	let file_length = request
		.header("Content-Length")
		.ok_or(HandlerError::new("The request doesn't have a `Content-Length` header"))?;
	let file_length = file_length
		.parse::<u32>()
		.map_err(|_| HandlerError::new("The `Content-Length` header is not a valid number"))?;

	log::info!("Receive file `{}` of {} bytes", file_name, file_length);

	let mut file_writer = resources
		.file_system
		.create_file(file_name, file_length)
		.map_err(|_| HandlerError::new("Not enough space available in the flash memory"))?;

	let mut buffer = [0; super::STACK_SIZE];
	while let Ok(read_bytes) = request.read(&mut buffer)
	{
		if read_bytes == 0
		{
			break;
		}

		file_writer
			.write_data(&mut resources.file_system, &buffer[0..read_bytes])
			.map_err(|error| HandlerError::new(&format!("{:#?}", error)))?;
	}

	file_writer
		.finish_writing(&mut resources.file_system)
		.map_err(|error| HandlerError::new(&format!("{:#?}", error)))?;

	log::info!("Successfully handled `send-file` HTTP request");

	Ok(())
}

pub fn get_print_status<C: Connection, P: Peripherals>(
	mut request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Start handling `get-print-status` HTTP request");

	let mut resources = get_resources(&resources)?;
	let _ = check_security(&mut request, &mut resources)?;

	#[derive(Serialize)]
	#[serde(rename_all = "camelCase")]
	struct HttpResponse
	{
		is_printing: bool,
		file_name_being_printed: String,
		print_duration_in_seconds: i32,
		time_printed_in_seconds: i32,
		is_paused: bool,
	}

	let response_message = match resources.print_process.get_file_being_printed()
	{
		Some(file_id_being_printed) =>
		{
			let mut file_being_printed = resources
				.file_system
				.read_file(file_id_being_printed)
				.map_err(|_| HandlerError::new("Couldn't open the file for read"))?;
			let file_name_being_printed = file_being_printed
				.read_name(&mut resources.file_system)
				.map_err(|_| HandlerError::new("Couldn't read the name of the file being printed"))?;

			let time_printed_in_seconds = resources.print_process.get_print_start_time().and_then(|start_time| {
				resources
					.system_time
					.as_ref()
					.and_then(|system_time| Some((system_time.now() - start_time).as_secs() as i32))
			});

			HttpResponse {
				is_printing: true,
				file_name_being_printed,
				print_duration_in_seconds: resources
					.print_process
					.get_print_estimated_duration_in_seconds()
					.map(|value| value as i32)
					.unwrap_or(-1),
				time_printed_in_seconds: time_printed_in_seconds.unwrap_or(-1),
				is_paused: pauser::is_paused(),
			}
		},
		None => HttpResponse {
			is_printing: false,
			file_name_being_printed: String::new(),
			print_duration_in_seconds: -1,
			time_printed_in_seconds: -1,
			is_paused: false,
		},
	};

	let mut response = ok_response(request)?;
	send_response!(
		BUFFER_SIZE = SER_BUFFER_SIZE,
		CALLBACK = "get_print_status",
		response_message,
		response
	);

	log::info!("Successfully handled `get-print-status` HTTP request");

	Ok(())
}

pub fn options_get_print_status<C: Connection, P: Peripherals>(
	request: Request<&mut C>, _: Resources<P>,
) -> Result<(), HandlerError>
{
	options_callback(request, "print-status", "")
}

pub fn pause_or_resume<C: Connection, P: Peripherals>(
	mut request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Start handling `pause-or-resume` HTTP request");

	let _ = check_security(&mut request, &mut get_resources(&resources)?)?;

	pauser::toggle_pause();

	let mut response = ok_response(request)?;
	response.flush()?;

	log::info!("Successfully handled `pause-or-resume` HTTP request");

	Ok(())
}

pub fn printer_state<C: Connection, P: Peripherals>(
	mut request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Start handling `printer-state` HTTP request");

	let _ = check_security(&mut request, &mut resources.lock())?;

	let mut response = ok_response(request)?;
	send_response!(
		BUFFER_SIZE = 400,
		CALLBACK = "printer_state",
		printer_state::get_current_state(),
		response
	);

	log::info!("Successfully handled `printer-state` HTTP request");

	Ok(())
}

pub fn options_printer_state<C: Connection, P: Peripherals>(
	request: Request<&mut C>, _: Resources<P>,
) -> Result<(), HandlerError>
{
	options_callback(request, "printer-state", "")
}

pub fn move_<C: Connection, P: Peripherals>(
	request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Start handling `move` HTTP request");

	todo!()
}

pub fn ota_update<C: Connection, P: Peripherals>(
	mut request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Start handling `ota-update` HTTP request");

	let mut resources = get_resources(&resources)?;
	let _ = check_security(&mut request, &mut resources)?;

	let ota_length = request
		.header("Content-Length")
		.ok_or(HandlerError::new("The request doesn't have a `Content-Length` header"))?;
	let ota_length = ota_length
		.parse::<usize>()
		.map_err(|_| HandlerError::new("The `Content-Length` header is not a valid number"))?;

	log::info!("Receiving OTA update of {ota_length} bytes");

	let mut update = resources
		.ota_updater
		.initiate_update(ota_length)
		.map_err(|error| HandlerError::new(&format!("Initiate: {:#?}", error)))?;

	let mut buffer = [0; super::STACK_SIZE];
	while let Ok(read_bytes) = request.read(&mut buffer)
	{
		if read_bytes == 0
		{
			break;
		}

		match update.write(&buffer[0..read_bytes])
		{
			Ok(written_percentage) => log::info!("OTA update {written_percentage}"),
			Err(error) =>
			{
				if let Err(error) = update.abort()
				{
					return Err(HandlerError::new(&format!("Abort update: {:#?}", error)));
				}
				return Err(HandlerError::new(&format!("Write: {:#?}", error)));
			},
		}
	}

	update
		.complete()
		.map_err(|error| HandlerError::new(&format!("Complete: {:#?}", error)))?;

	let mut response = ok_response(request)?;
	response.flush()?;

	log::info!("Successfully handled `ota-update` HTTP request");

	Ok(())
}

pub fn list_g_code_commands_in_memory<C: Connection, P: Peripherals>(
	mut request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Start handling `list-gcode-commands-in-memory` HTTP request");

	let mut resources = get_resources(&resources)?;
	let _ = check_security(&mut request, &mut resources)?;

	let starting_line = request
		.header("Starting-Line")
		.ok_or(HandlerError::new("The request doesn't have a `Starting-Line` header"))?;
	let starting_line = starting_line
		.parse::<u32>()
		.map_err(|_| HandlerError::new("The `Starting-Line` header is not a valid number"))?;

	let (lines, line_of_first_command) = resources
		.g_code_history
		.get_lines_from_history(starting_line)
		.unwrap_or(("", 0));

	#[derive(Serialize)]
	#[serde(rename_all = "camelCase")]
	struct HttpResponse<'a>
	{
		line_of_first_command: u32,
		commands: &'a str,
	}

	let response_message = HttpResponse {
		line_of_first_command,
		commands: lines,
	};

	let mut response = ok_response(request)?;
	send_response!(
		BUFFER_SIZE = SER_BUFFER_SIZE,
		CALLBACK = "list_g_code_commands_in_memory",
		response_message,
		response
	);

	log::info!("Successfully handled `list-gcode-commands-in-memory` HTTP request");

	Ok(())
}

pub fn options_list_g_code_commands_in_memory<C: Connection, P: Peripherals>(
	request: Request<&mut C>, _: Resources<P>,
) -> Result<(), HandlerError>
{
	options_callback(request, "gcode-commands", ", Starting-Line")
}

pub fn send_g_code_commands<C: Connection, P: Peripherals>(
	mut request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Start handling `send-gcode-commands` HTTP request");

	let mut resources = get_resources(&resources)?;
	let _ = check_security(&mut request, &mut resources)?;

	#[derive(Deserialize)]
	#[serde(rename_all = "camelCase")]
	struct HttpRequest
	{
		commands: String,
	}
	let request = deserialize_request!(
		BUFFER_SIZE = 2048,
		CALLBACK = "send_g_code_commands",
		HttpRequest,
		request
	);
	let lines = request.commands.lines();
	let mut commands = Vec::with_capacity(lines.clone().count());
	for line in lines
	{
		let parsed_line = resources
			.print_process
			.parse_line_to_execute(line)
			.map_err(|_| HandlerError::new("Invalid G-code"))?;
		if let Some(command) = parsed_line.command
		{
			commands.push(command);
		}
	}

	resources
		.command_sender
		.send_command(Command::AddGCodeCommandsToBuffer(commands))
		.map_err(|_| HandlerError::new("Coudln't send a Command::AddGCodeCommandsToBuffer"))?;

	log::info!("Successfully handled `send-gcode-commands` HTTP request");

	Ok(())
}

fn options_callback<C: Connection>(
	request: Request<&mut C>, callback_name: &str, allowed_headers: &str,
) -> Result<(), HandlerError>
{
	log::info!("Start handling `{callback_name}` HTTP request (OPTIONS)");

	let mut response = request.into_response(
		204,
		Some("No-Content"),
		&[
			("Access-Control-Allow-Origin", "*"),
			("Access-Control-Allow-Methods", "GET, POST, OPTIONS"),
			(
				"Access-Control-Allow-Headers",
				&format!("Content-Type, Password{}", allowed_headers),
			),
		],
	)?;
	response.flush()?;

	log::info!("Successfully handled `{callback_name}` HTTP request (OPTIONS)");

	Ok(())
}

fn get_resources<P: Peripherals>(resources: &Resources<P>) -> Result<MutexGuard<'_, ResourcesImpl<P>>, HandlerError>
{
	resources
		.try_lock()
		.ok_or(HandlerError::new("Resources mutex was locked"))
}

fn check_security<C: Connection, P: Peripherals>(
	request: &mut Request<&mut C>, resources: &mut MutexGuard<'_, ResourcesImpl<P>>,
) -> Result<(), HandlerError>
{
	let mut result = Ok(());
	if let Some(mut security) = resources.security.take()
	{
		if !security.can_pass(request, resources)
		{
			result = Err(HandlerError::new("Security"));
		}
		resources.security = Some(security);
	}
	else
	{
		result = Err(HandlerError::new("Sec"));
	}

	result
}

fn ok_response<C: Connection>(request: Request<&mut C>) -> Result<Response<&mut C>, C::Error>
{
	request.into_response(
		200,
		Some("OK"),
		&[
			("Access-Control-Allow-Origin", "*"),
			("Access-Control-Allow-Methods", "GET, POST, OPTIONS"),
			("Access-Control-Allow-Headers", "Content-Type"),
			("Content-Type", "application/json"),
		],
	)
}
