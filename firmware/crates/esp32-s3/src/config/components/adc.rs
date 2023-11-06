use esp_idf_hal::adc::config::{Config, Resolution};

pub const ADC_CONFIG: Config = Config {
	resolution: Resolution::Resolution12Bit,
	#[cfg(any(esp_idf_comp_esp_adc_cal_enabled, esp_idf_comp_esp_adc_enabled))]
	calibration: false,
};
