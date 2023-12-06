use enumset::EnumSet;
use esp_idf_hal::{
	spi::{
		config::{BitOrder, Config, DriverConfig, Duplex},
		Dma,
	},
	units::Hertz,
};

pub const FLASH_SPI_DRIVER_CONFIG: DriverConfig = DriverConfig {
	dma: Dma::Disabled,
	intr_flags: EnumSet::EMPTY,
};

pub const FLASH_SPI_CONFIG: Config = Config {
	baudrate: Hertz(80_000_000),
	data_mode: esp_idf_hal::spi::config::MODE_0,
	write_only: false,
	duplex: Duplex::Full,
	bit_order: BitOrder::MsbFirst,
	cs_active_high: false,
	input_delay_ns: 0,
	polling: true,
	allow_pre_post_delays: false,
	queue_size: 1,
};
