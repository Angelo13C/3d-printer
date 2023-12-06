use esp_idf_hal::{
	uart::config::{Config, DataBits, FlowControl, Parity, StopBits},
	units::*,
};

pub const UART_CONFIGURATION: Config = Config {
	baudrate: Hertz(500_000),
	data_bits: DataBits::DataBits8,
	parity: Parity::ParityNone,
	stop_bits: StopBits::STOP1P5,
	flow_control: FlowControl::None,
	flow_control_rts_threshold: 0,
	..Config::new()
};
