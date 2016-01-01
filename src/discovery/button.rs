use stm32f30x::gpio::*;
use stm32f30x::rcc::*;

pub trait Button {
    fn button_init(&mut self);
    fn button_get(&self) -> bool;
}

impl Button for Gpio {
    fn button_init(&mut self) {
        self.init(MODER::IN, OSPEEDR::_50MHZ, OTYPER::PP, PUPDR::NOPULL);
    }

    fn button_get(&self) -> bool {
        self.gpio.IDR & self.pin.bits() != 0
    }
}

pub const BUTTON: Gpio = Gpio::new(GPIOA, PIN0, AHBENR::GPIOAEN);

