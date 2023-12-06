use super::IpAddress;
use crate::printer::communication::security::Protection;

pub struct WhiteListProtection
{
	white_listed_ips: Vec<IpAddress>,
}

impl Protection for WhiteListProtection
{
	type Input<'a> = IpAddress;

	fn can_pass<'a>(&mut self, input: Self::Input<'a>) -> bool
	{
		self.white_listed_ips.contains(&input)
	}
}
