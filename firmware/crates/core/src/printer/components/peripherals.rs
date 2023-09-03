use super::hal::pwm::PwmPin;

pub trait Peripherals
{
    type FanPin: PwmPin;

    fn take_layer_fan_pin(&mut self) -> Option<Self::FanPin>;
    fn take_hotend_fan_pin(&mut self) -> Option<Self::FanPin>;
}