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
        self.init(MODE_OUT, OSPEED_50MHZ, OTYPE_PP, PUPD_UP);
    }

    fn led_toggle(&mut self) {
        self.port.ODR ^= self.pin.bits();
    }

    fn led_set(&mut self, value: bool) {
        if value {
            self.port.ODR.mask_set(Pin::all().bits(), self.pin.bits());
        } else {
            self.port.ODR.mask_set(Pin::empty().bits(), self.pin.bits());
        }
    }
}

pub const LED6:  Gpio = Gpio::new(PORT_E, PIN15, GPIOEEN);
pub const LED8:  Gpio = Gpio::new(PORT_E, PIN14, GPIOEEN);
pub const LED10: Gpio = Gpio::new(PORT_E, PIN13, GPIOEEN);
pub const LED9:  Gpio = Gpio::new(PORT_E, PIN12, GPIOEEN);
pub const LED7:  Gpio = Gpio::new(PORT_E, PIN11, GPIOEEN);
pub const LED5:  Gpio = Gpio::new(PORT_E, PIN10, GPIOEEN);
pub const LED3:  Gpio = Gpio::new(PORT_E, PIN9,  GPIOEEN);
pub const LED4:  Gpio = Gpio::new(PORT_E, PIN8,  GPIOEEN);

pub const LED: [Gpio; 8] = [
    LED3, LED4, LED5, LED6,
    LED7, LED8, LED9, LED10,
];

