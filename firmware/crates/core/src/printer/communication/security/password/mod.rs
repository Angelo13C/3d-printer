use std::str::FromStr;

use argon2::{
	password_hash::{PasswordHashString, SaltString},
	PasswordHasher, PasswordVerifier, Params,
};
pub use brute_force::BruteForceProtection;

use self::algorithm::AlgorithmIter;
use super::{Protection, ProtectionInput};
use crate::{printer::communication::http::resources::ResourcesImpl, utils::mutex::MutexGuard};

mod brute_force;

pub enum PasswordProtection
{
	/// Uses the [`argon2`] crate to store the password with a secure hash algorithm.
	///
	/// [`argon2`]: <https://docs.rs/argon2/latest/argon2/>
	Hashed
	{
		hashed_password: PasswordHashString,
		hash_settings: Params
	},
	NotHashed
	{
		password: String
	}
}

impl PasswordProtection
{
	pub fn new(password: &str, hash_settings: Option<Params>) -> Result<Self, argon2::password_hash::errors::Error>
	{
		match hash_settings
		{
			Some(hash_settings) => 
			{
				let salt = SaltString::generate(&mut rand_core::OsRng);
		
				let hashed_password = AlgorithmIter::get_random(hash_settings.clone())
					.hash_password(password.as_bytes(), &salt)?
					.serialize();
		
				Ok(Self::Hashed { hashed_password, hash_settings })
			},
			None => Ok(Self::NotHashed { password: password.to_string() }),
		}
	}
}

impl Protection for PasswordProtection
{
	type Input<'a> = PasswordAttempt<'a>;

	fn can_pass<'a>(&mut self, password_attempt: Self::Input<'a>) -> bool
	{
		match self
		{
			PasswordProtection::Hashed { hashed_password, hash_settings } => 
			{
				let algorithm_iter = AlgorithmIter::start_iterating(hash_settings.clone());
				for algorithm in algorithm_iter
				{
					if algorithm
						.verify_password(password_attempt.0.as_bytes(), &hashed_password.password_hash())
						.is_ok()
					{
						return true;
					}
				}

				false
			},
			PasswordProtection::NotHashed { password } => return password == password_attempt.0,
		}
	}
}

pub struct PasswordAttempt<'a>(&'a str);
impl<'a> ProtectionInput<'a> for PasswordAttempt<'a>
{
	fn generate<C: embedded_svc::http::server::Connection, P: crate::printer::components::Peripherals>(
		request: &'a mut embedded_svc::http::server::Request<C>, _: &mut MutexGuard<'_, ResourcesImpl<P>>,
	) -> Result<Self, ()>
	{
		Ok(Self(request.header("Password").ok_or(())?))
	}
}

mod algorithm
{
	use std::str::SplitWhitespace;

	use argon2::*;
	use rand_core::{OsRng, RngCore};

	pub struct AlgorithmIter
	{
		peppers: SplitWhitespace<'static>,
		hash_settings: Params
	}
	impl AlgorithmIter
	{
		const POSSIBLE_PEPPERS: &'static str = env!("PRINTER_PASSWORD_PEPPERS");

		pub fn get_random(hash_settings: Params) -> Argon2<'static>
		{
			let mut i = [0, 3];
			let _ = OsRng.try_fill_bytes(&mut i);
			let i = u16::from_ne_bytes(i);

			let mut self_ = Self::start_iterating(hash_settings.clone());
			let count = self_.peppers.clone().count();
			self_
				.peppers
				.nth(i as usize % count)
				.map(|secret| Self::generate_argon2(secret, hash_settings))
				.flatten()
				.unwrap_or_default()
		}

		pub fn start_iterating(hash_settings: Params) -> Self
		{
			Self {
				peppers: Self::POSSIBLE_PEPPERS.split_whitespace(),
				hash_settings
			}
		}

		fn generate_argon2(secret: &str, hash_settings: Params) -> Option<Argon2<'_>>
		{
			Argon2::new_with_secret(
				secret.as_bytes(),
				Algorithm::default(),
				Version::default(),
				hash_settings,
			)
			.ok()
		}
	}
	impl Iterator for AlgorithmIter
	{
		type Item = Argon2<'static>;

		fn next(&mut self) -> Option<Self::Item>
		{
			self.peppers
				.next()
				.map(|secret| Self::generate_argon2(secret, self.hash_settings.clone()))
				.flatten()
		}
	}
}
