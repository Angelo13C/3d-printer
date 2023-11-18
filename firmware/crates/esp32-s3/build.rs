use std::path::Path;

// Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
fn main() -> Result<(), Box<dyn std::error::Error>>
{
	embuild::build::CfgArgs::output_propagated("ESP_IDF")?;
	embuild::build::LinkArgs::output_propagated("ESP_IDF")?;

	set_environment_variables();

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
