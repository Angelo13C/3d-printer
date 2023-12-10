# Firmware
![Rust workflow](https://github.com/Angelo13C/3d-printer/actions/workflows/rust.yml/badge.svg)
[![License](https://img.shields.io/badge/license-GPL_3.0-blue.svg)](LICENSE)
[![Docs](https://github.com/Angelo13C/3d-printer/blob/master/docs/badge.svg)](https://angelo13c.github.io/3d-printer/)

This is the software that runs on top of the electrical control board.

This project is subdivided in 3 parts:
- Core
- Specific platforms bindings
- Tools

## Core
This is the essential part of the firmware, it contains all the functionality used by a 3D printer to work correctly.

It's an Hardware Abstraction Layer: it provides a set of interfaces that are implemented by some specific hardware to make it work.

## Specific platforms bindings
These are the bindings of each microcontroller to the core part of the firmware.

Currently only the `esp32-s3` is supported.

The advantage of this architecture is that the `core` part of the firmware is easily testable (with some Mock structs) and adding support to new platforms is
easy, only a really small part of code needs to be written.

## Tools
A CLI software that helps you in some way to work with this firmware. Currently it consists of only a way to flash the firmware `Over-The-Air` (OTA updates)
and via `USB`.
