use esp_idf_hal::adc::config::{Config, Resolution};

pub const ADC_CONFIG: Config = Config {
	resolution: Resolution::Resolution12Bit,
	#[cfg(any(esp_idf_comp_esp_adc_cal_enabled, esp_idf_comp_esp_adc_enabled))]
	calibration: false,
};

/// The value of this constant isn't `3300` but is `3250` because of some small problems with my PCB design
/// (or some problems with the ADC of the ESP32-S3) which lower the max readable voltage.
pub const ADC_MAX_READABLE_MILLIVOLTS: u16 = 3250;