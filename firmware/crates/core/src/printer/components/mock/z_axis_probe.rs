use crate::printer::components::motion::bed_leveling::ZAxisProbe;

pub struct MockZAxisProbe;

impl ZAxisProbe for MockZAxisProbe
{
	type IsEndReachedError = ();
	type OnEndReachedError = ();
	type HomingError = ();

	fn is_end_reached(&self) -> Result<bool, Self::IsEndReachedError>
	{
		todo!()
	}

	unsafe fn on_end_reached(&mut self, _: impl FnMut() + 'static) -> Result<(), Self::OnEndReachedError>
	{
		todo!()
	}

	fn prepare_for_homing(&mut self) -> Result<(), Self::HomingError>
	{
		todo!()
	}

	fn finish_homing(&mut self) -> Result<(), Self::HomingError>
	{
		todo!()
	}
}
