use stm32f30x::gpio::*;
use stm32f30x::rcc::*;

use hardware::registers::Reg;

pub trait Led {
    fn led_init(&mut self);
    fn led_toggle(&mut self);
    fn led_set(&mut self, value: bool);
}

impl Led for Gpio {
    fn led_init(&mut self) {
        self.init(MODER::OUT, OSPEEDR::_50MHZ, OTYPER::PP, PUPDR::UP);
    }

    fn led_toggle(&mut self) {
        self.gpio.ODR ^= self.pin.bits();
    }

    fn led_set(&mut self, value: bool) {
        if value {
            self.gpio.ODR.mask_set(Pin::all().bits(), self.pin.bits());
        } else {
            self.gpio.ODR.mask_set(Pin::empty().bits(), self.pin.bits());
        }
    }
}

pub const LED6:  Gpio = Gpio::new(GPIOE, PIN15, AHBENR::GPIOEEN);
pub const LED8:  Gpio = Gpio::new(GPIOE, PIN14, AHBENR::GPIOEEN);
pub const LED10: Gpio = Gpio::new(GPIOE, PIN13, AHBENR::GPIOEEN);
pub const LED9:  Gpio = Gpio::new(GPIOE, PIN12, AHBENR::GPIOEEN);
pub const LED7:  Gpio = Gpio::new(GPIOE, PIN11, AHBENR::GPIOEEN);
pub const LED5:  Gpio = Gpio::new(GPIOE, PIN10, AHBENR::GPIOEEN);
pub const LED3:  Gpio = Gpio::new(GPIOE, PIN9,  AHBENR::GPIOEEN);
pub const LED4:  Gpio = Gpio::new(GPIOE, PIN8,  AHBENR::GPIOEEN);

pub const LED: [Gpio; 8] = [
    LED3, LED4, LED5, LED6,
    LED7, LED8, LED9, LED10,
];

