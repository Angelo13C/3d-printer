use embedded_hal::digital::{ErrorType, InputPin};

use super::servo_motor::{ServoMotor, ServoPosition};
use crate::{
	printer::components::{
		hal::{
			interrupt::{InterruptPin, Trigger},
			pwm::PwmPin,
		},
		motion::bed_leveling::ZAxisProbe,
	},
	utils::measurement::{angle::deg, duration::SmallDuration},
};

const MIN_PULSE_DURATION: u32 = 544;
const MAX_PULSE_DURATION: u32 = 2400;

/// A [`BLTouch`] sensor.
///
/// # Alarm mode
///
///
/// [`BLTouch`]: https://www.antclabs.com/bltouch-v3
pub struct BLTouch<CP: PwmPin, ZP: InputPin + InterruptPin>
{
	control: ServoMotor<CP, MIN_PULSE_DURATION, MAX_PULSE_DURATION>,
	z_min: ZP,
}

impl<CP: PwmPin, ZP: InputPin + InterruptPin> BLTouch<CP, ZP>
{
	pub fn new(control_pin: CP, z_min_pin: ZP) -> Result<Self, CP::Error>
	{
		Ok(Self {
			control: ServoMotor::new(control_pin)?,
			z_min: z_min_pin,
		})
	}

	/// Calls the provided `callback` when the BLTouch touches something.
	///
	/// # Safety
	/// Check [`InterruptPin::subscribe_to_interrupt`].
	pub unsafe fn on_touch(&mut self, callback: impl FnMut() + 'static) -> Result<(), <ZP as InterruptPin>::Error>
	{
		self.z_min.subscribe_to_interrupt(Trigger::AnyEdge, callback)
	}

	/// Makes the BLTouch sensor execute the provided `command`.
	pub fn send_command(&mut self, command: BLTouchCommand) -> Result<(), <CP as PwmPin>::Error>
	{
		self.control.move_to(command.servo_position())?;

		Ok(())
	}
}

#[derive(Clone, Copy)]
/// A command [`executable`] by the BLTouch sensor.
///
/// [`executable`]: `BLTouch::send_command`
pub enum BLTouchCommand
{
	/// Push pin down.
	Deploy,
	/// Pull pin up.
	Stow,
	/// Eventually exits from [`alarm mode`](struct.BLTouch.html#alarm-mode) and enters touch switch mode.
	AlarmReleaseAndSwMode,
	/// Do self test (pushes pin down and up a couple of times).
	SelfTest,
	ModeStore,
	V5Mode,
	OdMode,
	/// Eventually exits from [`alarm mode`](struct.BLTouch.html#alarm-mode) and [`pulls pin up`](Self::Stow).
	AlarmReleaseAndStow,
}

impl BLTouchCommand
{
	fn servo_position(&self) -> ServoPosition<MIN_PULSE_DURATION, MAX_PULSE_DURATION>
	{
		match self
		{
			BLTouchCommand::Deploy => ServoPosition::from_angle(deg(10.)).unwrap(),
			BLTouchCommand::Stow => ServoPosition::from_angle(deg(90.)).unwrap(),
			BLTouchCommand::AlarmReleaseAndSwMode => ServoPosition::from_angle(deg(60.)).unwrap(),
			BLTouchCommand::SelfTest => ServoPosition::from_angle(deg(120.)).unwrap(),
			BLTouchCommand::ModeStore => ServoPosition::from_angle(deg(130.)).unwrap(),
			BLTouchCommand::V5Mode => ServoPosition::from_angle(deg(140.)).unwrap(),
			BLTouchCommand::OdMode => ServoPosition::from_angle(deg(150.)).unwrap(),
			BLTouchCommand::AlarmReleaseAndStow => ServoPosition::from_angle(deg(160.)).unwrap(),
		}
	}

	fn required_delay(&self) -> Option<SmallDuration>
	{
		match self
		{
			BLTouchCommand::Deploy => Some(SmallDuration::from_millis(750)),
			BLTouchCommand::Stow => Some(SmallDuration::from_millis(750)),
			BLTouchCommand::ModeStore => Some(SmallDuration::from_millis(150)),
			BLTouchCommand::V5Mode => Some(SmallDuration::from_millis(150)),
			BLTouchCommand::OdMode => Some(SmallDuration::from_millis(150)),
			BLTouchCommand::AlarmReleaseAndStow => Some(SmallDuration::from_millis(500)),
			_ => None,
		}
	}
}

impl<CP: PwmPin, ZP: InputPin + InterruptPin> ZAxisProbe for BLTouch<CP, ZP>
{
	type IsEndReachedError = <ZP as ErrorType>::Error;
	type OnEndReachedError = <ZP as InterruptPin>::Error;
	type HomingError = CP::Error;

	fn is_end_reached(&self) -> Result<bool, Self::IsEndReachedError>
	{
		todo!()
	}

	/// Equivalent to [`BLTouch::on_touch`].
	unsafe fn on_end_reached(&mut self, callback: impl FnMut() + 'static) -> Result<(), Self::OnEndReachedError>
	{
		self.on_touch(callback)
	}

	/// [`Pushes pin down`] and goes in [`touch switch mode`].
	///
	/// [`Pushes pin down`]: `BLTouchCommand::Deploy`
	/// [`touch switch mode`]: `BLTouchCommand::AlarmReleaseAndSwMode`
	fn prepare_for_homing(&mut self) -> Result<(), Self::HomingError>
	{
		self.send_command(BLTouchCommand::Deploy)?;
		self.send_command(BLTouchCommand::AlarmReleaseAndSwMode)?;

		Ok(())
	}

	/// [`Pulls pin up`].
	///
	/// [`Pulls pin up`]: `BLTouchCommand::Stow`
	fn finish_homing(&mut self) -> Result<(), Self::HomingError>
	{
		self.send_command(BLTouchCommand::Stow)?;

		Ok(())
	}
}
