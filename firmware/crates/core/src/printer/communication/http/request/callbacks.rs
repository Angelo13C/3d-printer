use embedded_svc::http::server::{Connection, HandlerError, Request};

use crate::printer::{communication::http::resources::Resources, components::Peripherals};

pub fn list_files<C: Connection, P: Peripherals>(
	request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	todo!()
}

pub fn delete_file<C: Connection, P: Peripherals>(
	request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	todo!()
}

pub fn print_file<C: Connection, P: Peripherals>(
	request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	todo!()
}

pub fn send_file<C: Connection, P: Peripherals>(
	request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	todo!()
}

pub fn get_print_status<C: Connection, P: Peripherals>(
	request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	todo!()
}

pub fn pause_or_resume<C: Connection, P: Peripherals>(
	request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	todo!()
}

pub fn printer_state<C: Connection, P: Peripherals>(
	request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	todo!()
}

pub fn move_<C: Connection, P: Peripherals>(
	request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	todo!()
}

pub fn list_g_code_commands_in_memory<C: Connection, P: Peripherals>(
	request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	todo!()
}

pub fn send_g_code_commands<C: Connection, P: Peripherals>(
	request: Request<&mut C>, resources: Resources<P>,
) -> Result<(), HandlerError>
{
	todo!()
}
