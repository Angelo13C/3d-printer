use crate::printer::components::time::SystemTime;

pub struct MockSystemTime;
impl SystemTime for MockSystemTime
{
	fn now(&self) -> std::time::Duration
	{
		todo!()
	}

	fn delay(&self, _: std::time::Duration)
	{
		todo!()
	}
}
