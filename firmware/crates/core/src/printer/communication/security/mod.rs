//! This module provides security functionality for communications with external systems.
//! It ensures that HTTP requests are validated through password protection, brute-force
//! prevention mechanisms...

use std::time::Duration;

use argon2::Params;
use embedded_svc::http::server::{Connection, Request};
use password::{BruteForceProtection, PasswordProtection};
use spin::MutexGuard;

use super::http::resources::ResourcesImpl;
use crate::printer::components::Peripherals;

mod ip_address;
mod password;

pub use ip_address::{GetIpAddress, IpAddress};

/// A trait that defines a layer of security for communication with external systems.
pub trait Protection
{
	type Input<'a>: ProtectionInput<'a>;

	/// Checks if the input passes the protection criteria.
	fn can_pass<'a>(&mut self, input: Self::Input<'a>) -> bool;
}

/// Input data required by [`Protection`] to verify the validity of a request.
pub trait ProtectionInput<'a>
{
	/// Generates input data from an HTTP request and resources.
	fn generate<C: Connection, P: Peripherals>(
		request: &'a mut Request<C>, resources: &mut MutexGuard<'_, ResourcesImpl<P>>,
	) -> Result<Self, ()>
	where Self: Sized;
}

/// Macro for testing protection mechanisms against a request.
macro_rules! test_protection {
	($protection_type: ty => $protection: expr, $request: expr, $resources: expr) => {
		if let Some(protection) = $protection.as_mut()
		{
			if let Ok(input) = <$protection_type as Protection>::Input::generate($request, $resources)
			{
				if !protection.can_pass(input)
				{
					return false;
				}
			}
			else
			{
				return false;
			}
		}
	};
}

/// A struct that encapsulates security mechanisms, including password and brute-force protection.
pub struct Security
{
	password_protection: Option<PasswordProtection>,
	brute_force_protection: Option<BruteForceProtection>,
}

impl Security
{
	/// Creates a new `Security` instance with the specified configuration.
	pub fn new(configuration: Configuration) -> Result<Self, CreationError>
	{
		log::info!(
			"Start the communication security with configuration: {:#?}",
			configuration
		);

		let (password_protection, brute_force_protection) = match configuration.password
		{
			PasswordConfiguration::None => (None, None),
			PasswordConfiguration::Password {
				password,
				hash_settings,
			} => (
				Some(PasswordProtection::new(password, hash_settings).map_err(CreationError::PasswordProtection)?),
				None,
			),
			PasswordConfiguration::PasswordAndBruteforce {
				password,
				hash_settings,
				delays_and_wrong_attempts_count_for_it,
			} => (
				None,
				Some(BruteForceProtection::new(
					PasswordProtection::new(password, hash_settings).map_err(CreationError::BruteForceProtection)?,
					delays_and_wrong_attempts_count_for_it,
				)),
			),
		};

		Ok(Self {
			password_protection,
			brute_force_protection,
		})
	}

	/// Validates the request against the security criteria.
	pub fn can_pass<C: Connection, P: Peripherals>(
		&mut self, request: &mut Request<C>, resources: &mut MutexGuard<'_, ResourcesImpl<P>>,
	) -> bool
	{
		test_protection!(PasswordProtection => self.password_protection, request, resources);
		test_protection!(BruteForceProtection => self.brute_force_protection, request, resources);

		true
	}
}

/// Possible errors that can occur during the creation of security mechanisms.
#[derive(Debug)]
pub enum CreationError
{
	PasswordProtection(argon2::password_hash::errors::Error),
	BruteForceProtection(argon2::password_hash::errors::Error),
}

/// Configuration settings for the `Security` struct.
#[derive(Debug)]
pub struct Configuration
{
	pub password: PasswordConfiguration,
}

/// Different configurations for password protection.
#[derive(Debug)]
pub enum PasswordConfiguration
{
	None,
	Password
	{
		password: &'static str,
		hash_settings: Option<Params>,
	},
	PasswordAndBruteforce
	{
		password: &'static str,
		hash_settings: Option<Params>,
		delays_and_wrong_attempts_count_for_it: Vec<(u32, Duration)>,
	},
}
