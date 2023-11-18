use std::{env, fmt::Write, fs, path::Path};

use quote::ToTokens;
use syn::GenericArgument;

const G_CODE_COMMANDS_FILE_PATH: &str = "src/printer/components/g_code/commands.rs";

fn main() -> Result<(), Box<dyn std::error::Error>>
{
	generate_g_code_deserializer()?;
	
	set_environment_variables();

	Ok(())
}

// Thanks to: https://doc.rust-lang.org/cargo/reference/build-script-examples.html
fn generate_g_code_deserializer() -> Result<(), Box<dyn std::error::Error>>
{
	let out_dir = env::var_os("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join("g_code_deserializer.rs");

	let project_file_path = env::var("CARGO_MANIFEST_DIR")?;
	let g_code_commands_file_path = Path::new(&project_file_path).join(G_CODE_COMMANDS_FILE_PATH);
	let g_code_commands_file_content = fs::read_to_string(g_code_commands_file_path)?;
	let g_code_commands_syn_file = syn::parse_file(&g_code_commands_file_content)?;

	struct GCodeCommandsParsed
	{
		parameters_by_name: Vec<(String, Vec<GCodeParameterParsed>)>,
	}
	impl GCodeCommandsParsed
	{
		fn new() -> Self
		{
			Self {
				parameters_by_name: Vec::new(),
			}
		}

		fn add_command(&mut self, command_name: String, parameters: Vec<GCodeParameterParsed>)
		{
			self.parameters_by_name.push((command_name, parameters));
		}

		fn into_code(self) -> String
		{
			let mut code = String::with_capacity(1000 * self.parameters_by_name.len());

			code += "match command {";

			let mut find_identifiers = String::with_capacity(120 * 6);
			let mut parameters = String::with_capacity(5000);
			for (command_name, command_parameters) in self.parameters_by_name
			{
				find_identifiers.clear();
				parameters.clear();
				for command_parameter in command_parameters
				{
					//let axes = find_identifier::<AnyWithoutSpaces<(X, Y, Z, E)>>(parameters).unzip();
					let variable_name = &command_parameter.variable_name;
					let parameter_identifier = &command_parameter.parameter_identifier;
					let parameter_type = &command_parameter.parameter_type;

					find_identifiers += &format!(
						"let {variable_name} = find_identifier::<{parameter_identifier}>(parameters.clone()).unzip();\n"
					);

					parameters += &format!("{variable_name}: Param::new({variable_name}.0, {variable_name}.1.and_then(|value| <{parameter_type} as GCodeParameterValue>::from_str(value, units).ok())),\n");
				}

				write!(
					code,
					"
    \"{command_name}\" => {{
        {find_identifiers}
        Some(Box::new({command_name} {{
            {parameters}
        }}))
    }},"
				)
				.unwrap();
			}

			code += "_ => None }";

			code
		}
	}

	struct GCodeParameterParsed
	{
		parameter_identifier: String,
		parameter_type: String,
		variable_name: String,
	}

	let mut g_code_commands_parsed = GCodeCommandsParsed::new();
	for item in g_code_commands_syn_file.items
	{
		if let syn::Item::Struct(struct_definition) = item
		{
			let name = struct_definition.ident.to_string();
			let mut parameters = Vec::with_capacity(struct_definition.fields.len());
			for field in struct_definition.fields
			{
				if let syn::Type::Path(field_type) = field.ty
				{
					for segment in field_type.path.segments
					{
						if segment.ident.to_string() == "Param"
						{
							if let syn::PathArguments::AngleBracketed(angle_bracket_args) = segment.arguments
							{
								let variable_name = field.ident.as_ref().unwrap().to_string();
								let [parameter_identifier, parameter_type] = std::array::from_fn(|index| {
									if let GenericArgument::Type(ty) = &angle_bracket_args.args[index]
									{
										if let syn::Type::Path(ty) = ty
										{
											let segment = ty.path.segments.last().unwrap();
											let mut result = segment.ident.to_string();
											if let syn::PathArguments::AngleBracketed(args) = &segment.arguments
											{
												result += "<";

												for arg in &args.args
												{
													if let syn::GenericArgument::Type(arg_type) = arg
													{
														result += &format!("{}", arg_type.to_token_stream());
													}
												}
												result += ">";
											}

											return result;
										}
									}
									String::new()
								});

								parameters.push(GCodeParameterParsed {
									parameter_identifier,
									parameter_type,
									variable_name,
								});
							}
						}
					}
				}
			}

			g_code_commands_parsed.add_command(name, parameters);
		}
	}

	fs::write(&dest_path, g_code_commands_parsed.into_code())?;

	Ok(())
}

fn set_environment_variables()
{
	const DIRECTORY_PATH: &str = "../../../private/Secrets/";

	fn set_environment_variable(relative_path: &str, environment_variable_key: &str)
	{
		if let Ok(environment_variable_value) = std::fs::read_to_string(Path::new(DIRECTORY_PATH).join(Path::new(relative_path)))
		{
			println!("cargo:rustc-env={}={}", environment_variable_key, environment_variable_value);
		}
	}

	set_environment_variable("WiFi/SSID.txt", "WIFI_SSID");
	set_environment_variable("WiFi/Password.txt", "WIFI_PASSWORD");
	set_environment_variable("Password/Password.txt", "PRINTER_PASSWORD");
	set_environment_variable("Password/Peppers.txt", "PRINTER_PASSWORD_PEPPERS");
}
