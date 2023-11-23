use embedded_svc::{
	http::server::{Connection, HandlerError, Request},
	io::Write,
	ota::OtaUpdate,
};
use serde::{Deserialize, Serialize};

use crate::{
	printer::{
		communication::http::{
			command::Command,
			other::printer_state,
			resources::{Resources, ResourcesImpl},
		},
		components::{file_system::regions::metadata::FileId, pauser, time::SystemTime, Peripherals},
	},
	utils::mutex::MutexGuard,
};

const SER_BUFFER_SIZE: usize = 2048;

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

pub fn list_files<C: Connection, P: Peripherals>(
	mut request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Handle `list_files` HTTP request");

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
		id: u32,
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
					id: u32::from_le_bytes(file_metadata.id.to_bytes()),
				});
			}
		}
	}

	let mut response = request.into_ok_response()?;
	send_response!(
		BUFFER_SIZE = SER_BUFFER_SIZE,
		CALLBACK = "list_files",
		response_message,
		response
	);

	Ok(())
}

pub fn delete_file<C: Connection, P: Peripherals>(
	mut request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Handle `delete_file` HTTP request");

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

	Ok(())
}

pub fn print_file<C: Connection, P: Peripherals>(
	mut request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Handle `print_file` HTTP request");

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

	Ok(())
}

pub fn send_file<C: Connection, P: Peripherals>(
	request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	todo!()
}

pub fn get_print_status<C: Connection, P: Peripherals>(
	mut request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Handle `get_print_status` HTTP request");

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

	let mut response = request.into_ok_response()?;
	send_response!(
		BUFFER_SIZE = SER_BUFFER_SIZE,
		CALLBACK = "get_print_status",
		response_message,
		response
	);

	Ok(())
}

pub fn pause_or_resume<C: Connection, P: Peripherals>(
	mut request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Handle `pause_or_resume` HTTP request");

	let _ = check_security(&mut request, &mut get_resources(&resources)?)?;

	pauser::toggle_pause();

	let mut response = request.into_ok_response()?;
	response.flush()?;

	Ok(())
}

pub fn printer_state<C: Connection, P: Peripherals>(
	request: Request<&mut C>, _: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Handle `printer_state` HTTP request");

	let mut response = request.into_ok_response()?;
	send_response!(
		BUFFER_SIZE = 400,
		CALLBACK = "printer_state",
		printer_state::get_current_state(),
		response
	);

	Ok(())
}

pub fn move_<C: Connection, P: Peripherals>(
	request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Handle `move` HTTP request");

	todo!()
}

pub fn ota_update<C: Connection, P: Peripherals>(
	mut request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Handle `ota_update` HTTP request");

	let mut resources = get_resources(&resources)?;
	let _ = check_security(&mut request, &mut resources)?;

	let mut update = resources
		.ota_updater
		.initiate_update()
		.map_err(|error| HandlerError::new(&format!("{:#?}", error)))?;

	let mut buffer = [0; super::STACK_SIZE];
	while let Ok(read_bytes) = request.read(&mut buffer)
	{
		if read_bytes == 0
		{
			break;
		}

		if let Err(error) = update.write(&buffer[0..read_bytes])
		{
			update.abort();
			return Err(HandlerError::new(&format!("{:#?}", error)));
		}
	}

	update.complete()?;

	Ok(())
}

pub fn list_g_code_commands_in_memory<C: Connection, P: Peripherals>(
	mut request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Handle `list_g_code_commands_in_memory` HTTP request");

	let mut resources = get_resources(&resources)?;
	let _ = check_security(&mut request, &mut resources)?;

	#[derive(Deserialize)]
	#[serde(rename_all = "camelCase")]
	struct HttpRequest
	{
		requested_line: u32,
	}

	let http_request = deserialize_request!(
		BUFFER_SIZE = 100,
		CALLBACK = "list_g_code_commands_in_memory",
		HttpRequest,
		request
	);
	let (lines, line_of_first_command) = resources
		.g_code_history
		.get_lines_from_history(http_request.requested_line)
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

	let mut response = request.into_ok_response()?;
	send_response!(
		BUFFER_SIZE = SER_BUFFER_SIZE,
		CALLBACK = "list_g_code_commands_in_memory",
		response_message,
		response
	);

	Ok(())
}

pub fn send_g_code_commands<C: Connection, P: Peripherals>(
	mut request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	log::info!("Handle `send_g_code_commands` HTTP request");

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
