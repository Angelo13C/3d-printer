use std::time::Duration;

use embedded_svc::http::server::{Connection, Request};
use password::{BruteForceProtection, PasswordProtection};

use super::http::resources::ResourcesImpl;
use crate::{printer::components::Peripherals, utils::mutex::MutexGuard};

mod ip_address;
mod password;

pub use ip_address::{GetIpAddress, IpAddress};

/// A type that implements this trait is a layer of security used for the communication with the external world.
pub trait Protection
{
	type Input<'a>: ProtectionInput<'a>;

	fn can_pass<'a>(&mut self, input: Self::Input<'a>) -> bool;
}
/// Input data required by [`Protection`] to check if the request is valid.
pub trait ProtectionInput<'a>
{
	fn generate<C: Connection, P: Peripherals>(
		request: &'a mut Request<C>, resources: &mut MutexGuard<'_, ResourcesImpl<P>>,
	) -> Result<Self, ()>
	where Self: Sized;
}

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

pub struct Security
{
	password_protection: Option<PasswordProtection>,
	brute_force_protection: Option<BruteForceProtection>,
}

impl Security
{
	pub fn new(configuration: Configuration) -> Result<Self, CreationError>
	{
		let (password_protection, brute_force_protection) = match configuration.password
		{
			PasswordConfiguration::None => (None, None),
			PasswordConfiguration::Password { password } => (
				Some(PasswordProtection::new(password).map_err(CreationError::PasswordProtection)?),
				None,
			),
			PasswordConfiguration::PasswordAndBruteforce {
				password,
				delays_and_wrong_attempts_count_for_it,
			} => (
				None,
				Some(BruteForceProtection::new(
					PasswordProtection::new(password).map_err(CreationError::BruteForceProtection)?,
					delays_and_wrong_attempts_count_for_it,
				)),
			),
		};

		Ok(Self {
			password_protection,
			brute_force_protection,
		})
	}

	pub fn can_pass<C: Connection, P: Peripherals>(
		&mut self, request: &mut Request<C>, resources: &mut MutexGuard<'_, ResourcesImpl<P>>,
	) -> bool
	{
		test_protection!(PasswordProtection => self.password_protection, request, resources);
		test_protection!(BruteForceProtection => self.brute_force_protection, request, resources);

		true
	}
}

#[derive(Debug)]
pub enum CreationError
{
	PasswordProtection(argon2::password_hash::errors::Error),
	BruteForceProtection(argon2::password_hash::errors::Error),
}

pub struct Configuration
{
	pub password: PasswordConfiguration,
}
pub enum PasswordConfiguration
{
	None,
	Password
	{
		password: &'static str,
	},
	PasswordAndBruteforce
	{
		password: &'static str,
		delays_and_wrong_attempts_count_for_it: Vec<(u32, Duration)>,
	},
}
