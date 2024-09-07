mod datagram;
mod microstepping;
mod registers;
mod slave_address;

use datagram::*;
pub use microstepping::MicrostepsPerStep;
pub use registers::*;
pub use slave_address::UARTAddress;

use crate::{
	printer::components::hal::uart::Uart as UartTrait,
	utils::{math::Percentage, measurement::duration::SmallDuration},
};

/// A driver to communicate with the [`TMC2209`] chip (to control a stepper motor).
///
/// Only UART communication is supported.
///
/// This struct contains various methods to change the settings of the TMC2209 chip
/// writing to its internal registers. Keep in mind that talking to the TMC2209 chip will
/// block the microcontroller for as few as some hundreds of microseconds to as much as
/// variuos milliseconds (it depends on the baud rate of the UART driver).
///
/// For further information on what each setting will do check [`TMC2209's datasheet`].
///
/// [`TMC2209's datasheet`]: <https://www.trinamic.com/fileadmin/assets/Products/ICs_Documents/TMC2209_datasheet_rev1.09.pdf>
/// [`TMC2209`]: <https://www.trinamic.com/products/integrated-circuits/details/tmc2209-la/>
pub struct TMC2209
{
	address: UARTAddress,

	registers: Registers,
}

impl TMC2209
{
	pub const STEP_MIN_LOW_TIME: SmallDuration = SmallDuration::from_tens_of_nanos(10);
	pub const STEP_MIN_HIGH_TIME: SmallDuration = SmallDuration::from_tens_of_nanos(10);

	/// Returns a [`TMC2209`] driver that is connected controlled by the microcontroller using `UART`.
	///
	/// This will also set some default settings on the TMC2209 that should be good
	/// for a 3D printer.
	/// The driver is [`disabled`] by default, so call [`TMC2209::enable`] when you want
	/// the motor to start being able to rotate.
	///
	/// [`disabled`]: `Self::disable`
	pub fn new_using_uart<Uart: UartTrait>(
		uart_address: UARTAddress, uart_driver: &mut Uart, microsteps_per_step: MicrostepsPerStep,
	) -> Result<Self, Uart::Error>
	{
		let mut self_ = Self {
			address: uart_address,
			registers: Registers::default(),
		};

		self_.initialize_uart(uart_driver)?;

		self_.set_current_scaling(Some(Percentage::from_0_to_100(100.).unwrap()), None, uart_driver)?;
		self_.enable_automatic_current_scaling(uart_driver)?;
		self_.enable_automatic_gradient_adaptation(uart_driver)?;
		self_.set_microsteps_per_step(microsteps_per_step, uart_driver)?;
		self_.enable_stealth_chop(uart_driver)?;
		self_.set_reply_delay(ReplyDelay::BitTime5, uart_driver)?;

		self_.enable(uart_driver)?;

		Ok(self_)
	}

	pub fn set_enabled<Uart: UartTrait>(&mut self, enabled: bool, uart_driver: &mut Uart) -> Result<(), Uart::Error>
	{
		match enabled
		{
			true => self.enable(uart_driver),
			false => self.disable(uart_driver),
		}
	}

	/// Enable the TMC2209 via software (UART).
	///
	/// This switches on the power stage on the TMC2209, making the motor able to move.
	///
	/// # Blocking
	/// Since the value is sent to the TMC2209's chip, calling this function will
	/// block the microcontroller for a bit.
	pub fn enable<Uart: UartTrait>(&mut self, uart_driver: &mut Uart) -> Result<(), Uart::Error>
	{
		write_field_of_register(CHOPCONF::TOFF, &mut self.registers.chopconf, 3);

		self.send_register::<CHOPCONF, Uart>(self.registers.chopconf, uart_driver)
	}

	/// Disable the TMC2209 via software (UART).
	///
	/// This switches off the power stage on the TMC2209, making all motor outputs floating (the motor can't move).
	///
	/// # Blocking
	/// Since the value is sent to the TMC2209's chip, calling this function will
	/// block the microcontroller for a bit.
	pub fn disable<Uart: UartTrait>(&mut self, uart_driver: &mut Uart) -> Result<(), Uart::Error>
	{
		write_field_of_register(CHOPCONF::TOFF, &mut self.registers.chopconf, 0);

		self.send_register::<CHOPCONF, Uart>(self.registers.chopconf, uart_driver)
	}

	/// Enable `StealthChop` on the driver, disabling `SpreadCycle`.
	///
	/// # Blocking
	/// Since the value is sent to the TMC2209's chip, calling this function will
	/// block the microcontroller for a bit.
	pub fn enable_stealth_chop<Uart: UartTrait>(&mut self, uart_driver: &mut Uart) -> Result<(), Uart::Error>
	{
		write_field_of_register(GCONF::en_SpreadCycle, &mut self.registers.gconf, 0);

		self.send_register::<GCONF, Uart>(self.registers.gconf, uart_driver)
	}

	/// Disable `StealthChop` on the driver, enabling `SpreadCycle`.
	///
	/// # Blocking
	/// Since the value is sent to the TMC2209's chip, calling this function will
	/// block the microcontroller for a bit.
	pub fn disable_stealth_chop<Uart: UartTrait>(&mut self, uart_driver: &mut Uart) -> Result<(), Uart::Error>
	{
		write_field_of_register(GCONF::en_SpreadCycle, &mut self.registers.gconf, 1);

		self.send_register::<GCONF, Uart>(self.registers.gconf, uart_driver)
	}

	/// Sets the amount of [`microsteps`] the motor will take for each single step.
	///
	/// # Blocking
	/// Since the value is sent to the TMC2209's chip, calling this function will
	/// block the microcontroller for a bit.
	///
	/// [`microsteps`]: <https://www.trinamic.com/technology/motor-control-technology/microstepping/>
	pub fn set_microsteps_per_step<Uart: UartTrait>(
		&mut self, microsteps_per_step: MicrostepsPerStep, uart_driver: &mut Uart,
	) -> Result<(), Uart::Error>
	{
		write_field_of_register(
			CHOPCONF::MRES,
			&mut self.registers.chopconf,
			microsteps_per_step.as_value_of_register() as u32,
		);

		self.send_register::<CHOPCONF, Uart>(self.registers.chopconf, uart_driver)
	}

	/// Get the value you previously set using [`Self::set_microsteps_per_step`].
	pub fn get_microsteps_per_step(&self) -> MicrostepsPerStep
	{
		let microsteps = read_field_of_register(CHOPCONF::MRES, self.registers.chopconf);
		MicrostepsPerStep::from_value_of_register(microsteps as u8).unwrap()
	}

	/// Read the state of a physical pin of the TMC2209 chip using UART.
	///
	/// # Blocking
	/// Since the value is read from the TMC2209's registers, calling this function will
	/// block the microcontroller for a bit.
	pub fn read_pin_state<Uart: UartTrait>(
		&self, register_field: IOIN, uart_driver: &mut Uart,
	) -> Result<u8, ReadRegisterError<Uart>>
	{
		let ioin = self.receive_register::<IOIN, Uart>(uart_driver)?;
		Ok(read_field_of_register(register_field, ioin) as u8)
	}

	/// Enable automatic current control.
	///
	/// # Blocking
	/// Since the value is sent to the TMC2209's chip, calling this function will
	/// block the microcontroller for a bit.
	pub fn enable_automatic_current_scaling<Uart: UartTrait>(
		&mut self, uart_driver: &mut Uart,
	) -> Result<(), Uart::Error>
	{
		write_field_of_register(PWMCONF::PWM_AUTOSCALE, &mut self.registers.pwm_conf, 1);

		self.send_register::<PWMCONF, Uart>(self.registers.pwm_conf, uart_driver)
	}

	/// Disable automatic current control.
	///
	/// # Blocking
	/// Since the value is sent to the TMC2209's chip, calling this function will
	/// block the microcontroller for a bit.
	pub fn disable_automatic_current_scaling<Uart: UartTrait>(
		&mut self, uart_driver: &mut Uart,
	) -> Result<(), Uart::Error>
	{
		write_field_of_register(PWMCONF::PWM_AUTOSCALE, &mut self.registers.pwm_conf, 0);

		self.send_register::<PWMCONF, Uart>(self.registers.pwm_conf, uart_driver)
	}

	/// Makes the TMC2209 chip automatically adjust the PWM gradient value.
	///
	/// # Blocking
	/// Since the value is sent to the TMC2209's chip, calling this function will
	/// block the microcontroller for a bit.
	pub fn enable_automatic_gradient_adaptation<Uart: UartTrait>(
		&mut self, uart_driver: &mut Uart,
	) -> Result<(), Uart::Error>
	{
		write_field_of_register(PWMCONF::PWM_AUTOGRAD, &mut self.registers.pwm_conf, 1);

		self.send_register::<PWMCONF, Uart>(self.registers.pwm_conf, uart_driver)
	}

	/// Stops the automatic adjustment of the PWM gradient value in the TMC2209 chip.
	///
	/// # Blocking
	/// Since the value is sent to the TMC2209's chip, calling this function will
	/// block the microcontroller for a bit.
	pub fn disable_automatic_gradient_adaptation<Uart: UartTrait>(
		&mut self, uart_driver: &mut Uart,
	) -> Result<(), Uart::Error>
	{
		write_field_of_register(PWMCONF::PWM_AUTOGRAD, &mut self.registers.pwm_conf, 0);

		self.send_register::<PWMCONF, Uart>(self.registers.pwm_conf, uart_driver)
	}

	fn percentage_to_current_scaling(percentage: Percentage) -> u32
	{
		const MAX_VALUE: f32 = 31.;
		(MAX_VALUE * percentage.into_0_to_1()).round() as u32
	}

	/// Scales the current going to the motors.
	///
	/// `on_run` is the scale used when the motor is running, `on_hold` instead when it is `idle`.
	///
	/// Pass `None` as a parameter to not change the scale you had before.
	///
	/// # Examples
	/// ```ignore
	/// let mut uart_driver = //...
	/// let mut tmc2209 = //...
	///
	/// // This will set the current scaling of the motor when running to 80% and leave
	/// // the current scaling when the motor is idle unchanged.
	/// tmc2209.set_current_scaling(Some(Percentage::from_0_to_1(0.8).unwrap()), None, &mut uart_driver);
	/// ```
	///
	/// # Blocking
	/// Since the value is sent to the TMC2209's chip, calling this function will
	/// block the microcontroller for a bit.
	pub fn set_current_scaling<Uart: UartTrait>(
		&mut self, on_run: Option<Percentage>, on_hold: Option<Percentage>, uart_driver: &mut Uart,
	) -> Result<(), Uart::Error>
	{
		if let Some(on_run) = on_run
		{
			write_field_of_register(
				IHOLD_IRUN::IRUN,
				&mut self.registers.ihold_irun,
				Self::percentage_to_current_scaling(on_run),
			);
		}
		if let Some(on_hold) = on_hold
		{
			write_field_of_register(
				IHOLD_IRUN::IHOLD,
				&mut self.registers.ihold_irun,
				Self::percentage_to_current_scaling(on_hold),
			);
		}

		self.send_register::<IHOLD_IRUN, Uart>(self.registers.ihold_irun, uart_driver)
	}

	/// # Blocking
	/// Since the value is sent to the TMC2209's chip, calling this function will
	/// block the microcontroller for a bit.
	pub fn enable_analog_scale_current<Uart: UartTrait>(&mut self, uart_driver: &mut Uart) -> Result<(), Uart::Error>
	{
		write_field_of_register(GCONF::I_scale_analog, &mut self.registers.gconf, 1);

		self.send_register::<GCONF, Uart>(self.registers.gconf, uart_driver)
	}

	/// # Blocking
	/// Since the value is sent to the TMC2209's chip, calling this function will
	/// block the microcontroller for a bit.
	pub fn disable_analog_scale_current<Uart: UartTrait>(&mut self, uart_driver: &mut Uart) -> Result<(), Uart::Error>
	{
		write_field_of_register(GCONF::I_scale_analog, &mut self.registers.gconf, 0);

		self.send_register::<GCONF, Uart>(self.registers.gconf, uart_driver)
	}

	/// The TMC2209 chip will extrapolate the microsteps per step you [`previously set`] to 256.
	///
	/// This won't make any difference in the way you control the motor, it will just make the motor
	/// less noisy at the cost of a small positional error.
	///
	/// # Blocking
	/// Since the value is sent to the TMC2209's chip, calling this function will
	/// block the microcontroller for a bit.
	///
	/// [`previously set`]: `Self::set_microsteps_per_step`
	pub fn enable_microstep_interpolation<Uart: UartTrait>(&mut self, uart_driver: &mut Uart)
		-> Result<(), Uart::Error>
	{
		write_field_of_register(CHOPCONF::INTPOL, &mut self.registers.chopconf, 1);

		self.send_register::<CHOPCONF, Uart>(self.registers.chopconf, uart_driver)
	}

	/// Disable the microstep interpolation. Check [`TMC2209::enable_microstep_interpolation`] for more info.
	///
	/// # Blocking
	/// Since the value is sent to the TMC2209's chip, calling this function will
	/// block the microcontroller for a bit.
	pub fn disable_microstep_interpolation<Uart: UartTrait>(
		&mut self, uart_driver: &mut Uart,
	) -> Result<(), Uart::Error>
	{
		write_field_of_register(CHOPCONF::INTPOL, &mut self.registers.chopconf, 0);

		self.send_register::<CHOPCONF, Uart>(self.registers.chopconf, uart_driver)
	}

	pub fn set_reply_delay<Uart: UartTrait>(
		&mut self, delay: ReplyDelay, uart_driver: &mut Uart,
	) -> Result<(), Uart::Error>
	{
		write_field_of_register(NODECONF::SENDDELAY, &mut self.registers.nodeconf, delay as u16);

		self.send_register::<NODECONF, Uart>(self.registers.nodeconf as u32, uart_driver)
	}

	/// Initialize the UART communication with the TMC2209 chip.
	fn initialize_uart<Uart: UartTrait>(&mut self, uart_driver: &mut Uart) -> Result<(), Uart::Error>
	{
		write_field_of_register(GCONF::I_scale_analog, &mut self.registers.gconf, 0);
		write_field_of_register(GCONF::pdn_disable, &mut self.registers.gconf, 1);
		write_field_of_register(GCONF::mstep_reg_select, &mut self.registers.gconf, 1);
		write_field_of_register(GCONF::multistep_filt, &mut self.registers.gconf, 1);

		self.send_register::<GCONF, Uart>(self.registers.gconf, uart_driver)
	}

	/// Actually sends the value of a register to the TMC2209 chip.
	/// If you write to a register without actually sending its value to the chip using this function,
	/// it's like you did nothing.
	///
	/// # Blocking
	/// This will block the microcontroller for some time (depending on the UART's baud rate).
	fn send_register<R: WriteableRegister, Uart: UartTrait>(
		&self, data: impl Into<u32>, uart_driver: &mut Uart,
	) -> Result<(), Uart::Error>
	{
		let mut datagram = WriteAccessDatagram::new::<R>(self.address, data.into());

		uart_driver.write(datagram.as_bytes_mut())?;

		Ok(())
	}

	/// Actually receives the value of a register from the TMC2209 chip.
	/// It's possible to receives some register and cache the value inside the microcontroller,
	/// but with this function you will actually get the up-to-date value of the register.
	///
	/// # Blocking
	/// This will block the microcontroller for some time.
	fn receive_register<R: ReadableRegister, Uart: UartTrait>(
		&self, uart_driver: &mut Uart,
	) -> Result<u32, ReadRegisterError<Uart>>
	{
		uart_driver.flush_read().map_err(ReadRegisterError::UartError)?;

		let mut read_request_datagram = ReadAccessRequestDatagram::new::<R>(self.address);
		uart_driver
			.write(read_request_datagram.as_bytes_mut())
			.map_err(ReadRegisterError::UartError)?;

		// Flush the echo caused by the fact that TX and RX share the same electrical line
		const ECHO_DELAY_MAX_TICKS: SmallDuration = SmallDuration::from_micros(4_000);
		let echo_bytes_count = uart_driver
			.read(read_request_datagram.as_bytes_mut(), ECHO_DELAY_MAX_TICKS)
			.map_err(ReadRegisterError::UartError)?;
		if echo_bytes_count < std::mem::size_of::<ReadAccessRequestDatagram>()
		{
			return Err(ReadRegisterError::FlushEchoInvalidSize);
		}

		// Read the reply
		const REPLY_DELAY_MAX_TICKS: SmallDuration = SmallDuration::from_micros(10_000);
		let mut read_reply_datagram = ReadAccessReplyDatagram::default();
		let reply_bytes_count = uart_driver
			.read(read_reply_datagram.as_bytes_mut(), REPLY_DELAY_MAX_TICKS)
			.map_err(ReadRegisterError::UartError)?;
		if reply_bytes_count < std::mem::size_of::<ReadAccessReplyDatagram>()
		{
			return Err(ReadRegisterError::ReplyDatagramIncomplete);
		}
		read_reply_datagram.reverse_data_endianness();
		if !read_reply_datagram.is_valid()
		{
			return Err(ReadRegisterError::ReplyDatagramInvaild);
		}

		Ok(read_reply_datagram.data())
	}
}

/// Error that occurred while trying to read a register of the [`TMC2209`] chip.
pub enum ReadRegisterError<Uart: UartTrait>
{
	UartError(Uart::Error),
	FlushEchoInvalidSize,
	ReplyDatagramIncomplete,
	ReplyDatagramInvaild,
}
impl<Uart: UartTrait> std::fmt::Debug for ReadRegisterError<Uart>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Self::UartError(arg0) => f.debug_tuple("UartError").field(arg0).finish(),
			Self::FlushEchoInvalidSize => write!(f, "FlushEchoInvalidSize"),
			Self::ReplyDatagramIncomplete => write!(f, "ReplyDatagramIncomplete"),
			Self::ReplyDatagramInvaild => write!(f, "ReplyDatagramInvaild"),
		}
	}
}

#[repr(u8)]
pub enum ReplyDelay
{
	BitTime1 = 1,
	BitTime3 = 3,
	BitTime5 = 5,
	BitTime7 = 7,
	BitTime9 = 9,
	BitTime11 = 11,
	BitTime13 = 13,
	BitTime15 = 15,
}
