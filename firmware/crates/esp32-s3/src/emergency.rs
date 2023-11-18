use std::ops::RangeInclusive;

pub unsafe fn disable_all_pins_function()
{
    const GPIO_PINS: RangeInclusive<i32> = 0..=100;

    for gpio_pin in GPIO_PINS
    {
        esp_idf_sys::gpio_set_level(gpio_pin, 0);
    }
}