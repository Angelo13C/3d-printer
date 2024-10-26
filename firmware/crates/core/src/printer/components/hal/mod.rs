//! Hardware Abstraction Layer (HAL) module.
//!
//! This module provides abstractions for various hardware components that
//! are commonly used in embedded systems. It includes modules for ADC, PWM,
//! timers, UART communication, interrupts, and watchdog timers.
//!
//! The purpose of this module is to provide a consistent interface for interacting
//! with hardware peripherals, allowing for easier development and portability
//! across different platforms.

pub mod adc;
pub mod interrupt;
pub mod pwm;
pub mod timer;
pub mod uart;
pub mod watchdog;
