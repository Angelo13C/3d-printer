//! # This crate
//! This is the core part of the firmware for a [3D printer](https://en.wikipedia.org/wiki/3D_printing), and it's still in development.
//!
//! This crate is an Hardware Abstraction Layer (HAL), meaning it provides an implementation of:
//! - [Communication] with the external world, through an [HTTP server] (over [WiFi]) for example
//! - A basic [file system] to store and later retrieve G-code files from a [flash memory]
//! - Parsing and execution of [G-code]
//! - Closed loop [PID] heaters
//! - A [motion controller] to move the tool that is connected to the [stepper motors], with support for [homing] and [bed leveling]
//! - Some drivers for various components (like [BLTouch], [TMC2209], [fans]...)
//! - [OTA updates]
//! - [Pausing and resuming] a print
//!
//! But it doesn't provide any platform specific implementation, which is done by other crates (like the `esp32-s3` one).
//!
//! This makes the adding of support for a new platform easy since only the specific hardware implementations need to be written.
//! (which means implementing some traits from [embedded_hal] and some other custom traits present in this crate).
//!
//! [Communication]: printer::communication
//! [HTTP server]: printer::communication::http
//! [WiFi]: printer::communication::communicator::wifi
//! [file system]: printer::components::file_system
//! [G-code]: printer::components::g_code
//! [PID]: printer::components::temperature
//! [motion controller]: printer::components::motion
//! [stepper motors]: printer::components::drivers::stepper_motor
//! [homing]: printer::components::motion::homing
//! [bed leveling]: printer::components::motion::bed_leveling
//! [fans]: printer::components::drivers::fan
//! [flash memory]: printer::components::drivers::spi_flash_memory
//! [BLTouch]: printer::components::drivers::bl_touch
//! [TMC2209]: printer::components::drivers::stepper_motor::tmc2209
//! [OTA updates]: printer::communication::ota
//! [Pausing and resuming]: printer::components::pauser

extern crate alloc;

pub mod printer;
pub mod utils;

pub use embedded_hal;
