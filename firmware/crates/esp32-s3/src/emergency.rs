pub unsafe fn disable_all_pins_function()
{
	let gpio_pins = (0..=21).chain(26..=48);

	for gpio_pin in gpio_pins
	{
		esp_idf_sys::gpio_set_level(gpio_pin, 0);
	}
}
