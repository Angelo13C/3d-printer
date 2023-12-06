pub mod commands;

use std::error::Error as ErrorStd;

use clap::*;

use self::commands::Commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli
{
	#[command(subcommand)]
	command: Commands,
}

impl Cli
{
	pub fn run(self) -> Result<(), Box<dyn ErrorStd>>
	{
		self.command.run()
	}
}
