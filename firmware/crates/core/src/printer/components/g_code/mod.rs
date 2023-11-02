//! **G-code** is the language understood by 3D printers, and it's used to make the 3D printer execute
//! commands (like move the tool to a specific location, set the bed's temperature...).
//!
//! A 3D printer slicer software receives a 3D model and converts it to a long list of G-code commands,
//! which are then sent to the printer so that it can print the model.
//!
//! This firmware (for now) only supports the most important G-code commands.
//!
//! For an extensive list of all the existing G-code commands, check the [RepRap's documentation](https://reprap.org/wiki/G-code).

pub mod parameters;
