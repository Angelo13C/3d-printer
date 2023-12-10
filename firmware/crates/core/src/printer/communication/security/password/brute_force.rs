use std::time::Duration;

use super::{PasswordAttempt, PasswordProtection, Protection, ProtectionInput};
use crate::{
	printer::{communication::http::resources::ResourcesImpl, components::time::SystemTime},
	utils::mutex::MutexGuard,
};

/// Embeds a [`PasswordProtection`] to protect it from brute forces attacks.
///
/// This protection will add a minimum delay time on each possible attempt when a password attempt is wrong.
pub struct BruteForceProtection
{
	password_protection: PasswordProtection,
	last_attempt_time: Option<Duration>,
	consecutive_wrong_attempts: u32,
	delays_and_wrong_attempts_count_for_it: Vec<(u32, Duration)>,
}

impl BruteForceProtection
{
	pub fn new(
		password_protection: PasswordProtection, delays_and_wrong_attempts_count_for_it: Vec<(u32, Duration)>,
	) -> Self
	{
		Self {
			password_protection,
			last_attempt_time: None,
			consecutive_wrong_attempts: 0,
			delays_and_wrong_attempts_count_for_it,
		}
	}
	fn delay_between_attempts(&self) -> Duration
	{
		self.delays_and_wrong_attempts_count_for_it
			.iter()
			.find_map(|(attempts, delay)| (self.consecutive_wrong_attempts >= *attempts).then_some(*delay))
			.unwrap_or(Duration::ZERO)
	}
}

impl Protection for BruteForceProtection
{
	type Input<'a> = BruteForceAttempt<'a>;

	fn can_pass<'a>(&mut self, input: Self::Input<'a>) -> bool
	{
		if let Some(last_attempt_time) = self.last_attempt_time
		{
			if input.current_time - last_attempt_time < self.delay_between_attempts()
			{
				return false;
			}
		}

		match self.password_protection.can_pass(input.password)
		{
			true =>
			{
				self.consecutive_wrong_attempts = 0;
				true
			},
			false =>
			{
				self.last_attempt_time = Some(input.current_time);
				self.consecutive_wrong_attempts += 1;
				false
			},
		}
	}
}

pub struct BruteForceAttempt<'a>
{
	password: PasswordAttempt<'a>,
	current_time: Duration,
}
impl<'a> ProtectionInput<'a> for BruteForceAttempt<'a>
{
	fn generate<C: embedded_svc::http::server::Connection, P: crate::printer::components::Peripherals>(
		request: &'a mut embedded_svc::http::server::Request<C>, resources: &mut MutexGuard<'_, ResourcesImpl<P>>,
	) -> Result<Self, ()>
	{
		let password = PasswordAttempt::generate(request, resources)?;

		let current_time = { resources.system_time.as_ref().ok_or(())?.now() };

		Ok(Self { password, current_time })
	}
}
