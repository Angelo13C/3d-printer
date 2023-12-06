use super::IpAddress;
use crate::printer::communication::security::Protection;

pub struct BlackListProtection
{
	black_listed_ips: Vec<IpAddress>,
}

impl Protection for BlackListProtection
{
	type Input<'a> = IpAddress;

	fn can_pass<'a>(&mut self, input: Self::Input<'a>) -> bool
	{
		!self.black_listed_ips.contains(&input)
	}
}
