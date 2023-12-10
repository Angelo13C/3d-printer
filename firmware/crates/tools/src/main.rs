use clap::Parser;

pub mod cli;
pub mod preferences;

fn main()
{
	let cli = cli::Cli::parse();

	cli.run().unwrap()
}
