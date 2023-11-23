use std::{error::Error, path::Path};

use clap::*;

use crate::preferences::Preference;

#[derive(Subcommand)]
pub enum Commands
{
	/// Flash an application in ELF format to a target device.
	Flash
	{
		/// Path to the application to flash.
		path_of_firmware_to_flash: String,

		/// Path to a partition table that will be merged to with the application to flash.
		#[arg(long)]
		partition_table: Option<String>,

		/// If this flag is set the update will be flashed Over-The-Air (an HTTP "ota-update" request sent via WiFi to the provided `ip`),
		/// otherwise it's going to invoke the command `espflash flash --monitor` (flashing it via USB).
		#[arg(long)]
		ota: bool,

		/// Requires `--ota`. IP address of the device that should be flashed. If no IP is provided, the one that you provided the last time will be used.
		#[arg(long, requires = "ota")]
		ip: Option<String>,
	},
}

impl Commands
{
	pub fn run(self) -> Result<(), Box<dyn Error>>
	{
		match self
		{
			Commands::Flash {
				path_of_firmware_to_flash,
				partition_table,
				ota,
				ip,
			} =>
			{
				if ota
				{
					let ip = if let Some(ip) = ip
					{
						Preference::FlashOtaKey.save(ip.clone())?;
						ip
					}
					else
					{
						Preference::FlashOtaKey.load()?
					};

					let path_of_firmware_to_flash_bin = format!("{}.bin", path_of_firmware_to_flash);
					std::process::Command::new("espflash")
						.args(["save-image", "--chip", "esp32s3", &path_of_firmware_to_flash, &path_of_firmware_to_flash_bin, "--flash-size", "8mb"])
						.spawn()?
						.wait()?;
					
					let path = Path::new(&path_of_firmware_to_flash_bin);
					let firmware_to_flash = std::fs::read(path)?;

					let client = reqwest::blocking::ClientBuilder::new().timeout(None).build().unwrap();
					const METHOD: &str = "http";
					const CALLBACK: &str = "ota-update";

					let response = client
						.post(format!("{METHOD}://{ip}/{CALLBACK}"))
						.body(firmware_to_flash)
						.send()?;

					println!("Response: {:#?}", response);

					std::fs::remove_file(path)?;

					println!("Removed .bin file");
				}
				else
				{
					std::process::Command::new("espflash")
						.args(["flash", "--monitor", &path_of_firmware_to_flash, "--partition-table", &partition_table.unwrap()])
						.spawn()?
						.wait()?;
				}
			},
		}

		Ok(())
	}
}
