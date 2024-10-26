use std::path::Path;

// Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
fn main() -> Result<(), Box<dyn std::error::Error>>
{
	embuild::build::CfgArgs::output_propagated("ESP_IDF")?;
	embuild::build::LinkArgs::output_propagated("ESP_IDF")?;

	set_environment_variables();
	copy_partition_table();

	Ok(())
}

fn set_environment_variables()
{
	const DIRECTORY_PATH: &str = "../../../private/Secrets/";

	fn set_environment_variable(relative_path: &str, environment_variable_key: &str)
	{
		if let Ok(environment_variable_value) =
			std::fs::read_to_string(Path::new(DIRECTORY_PATH).join(Path::new(relative_path)))
		{
			println!(
				"cargo:rustc-env={}={}",
				environment_variable_key, environment_variable_value
			);
		}
	}

	set_environment_variable("WiFi/SSID.txt", "WIFI_SSID");
	set_environment_variable("WiFi/Password.txt", "WIFI_PASSWORD");
	set_environment_variable("Password/Password.txt", "PRINTER_PASSWORD");
	set_environment_variable("Password/Peppers.txt", "PRINTER_PASSWORD_PEPPERS");
}

fn copy_partition_table()
{
	const PARTITION_TABLE_PATH: &str = "partitions.csv";
	let output_directory_path = std::env::var("OUT_DIR").unwrap();
	let output_file_path = Path::new(&output_directory_path)
		.parent()
		.unwrap()
		.parent()
		.unwrap()
		.parent()
		.unwrap()
		.join(Path::new(PARTITION_TABLE_PATH));

	std::fs::File::create(output_file_path.clone()).unwrap();
	std::fs::copy(PARTITION_TABLE_PATH, output_file_path).unwrap();
}
