use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct GPIO {
    pub MODER:              u32,
    pub OTYPER:             u16,
    pub RESERVED0:          u16,
    pub OSPEEDR:            u32,
    pub PUPDR:              u32,
    pub IDR:                u16,
    pub RESERVED1:          u16,
    pub ODR:                u16,
    pub RESERVED2:          u16,
    pub BSRR:               u32,
    pub LCKR:               u32,
    pub AFR:                [u32; 2],
    pub BRR:                u16,
    pub RESERVED3:          u16,
}

//register addresses

registers! {
    const GPIOA:        GPIO            = GPIOA_BASE,
    const GPIOB:        GPIO            = GPIOB_BASE,
    const GPIOC:        GPIO            = GPIOC_BASE,
    const GPIOD:        GPIO            = GPIOD_BASE,
    const GPIOE:        GPIO            = GPIOE_BASE,
    const GPIOF:        GPIO            = GPIOF_BASE,
    const GPIOG:        GPIO            = GPIOG_BASE,
    const GPIOH:        GPIO            = GPIOH_BASE,
}

//custom

use hardware::registers::Reg;

bitflags! {
    flags Mode: u32 {
        const MODE_IN    = 0x00,
        const MODE_OUT   = 0x01,
        const MODE_AF    = 0x02,
        const MODE_AN    = 0x03,
    }
}

bitflags! {
    flags OType: u16 {
        const OTYPE_PP   = 0x00,
        const OTYPE_OD   = 0x01,
    }
}

bitflags! {
    flags OSpeed: u32 {
        const OSPEED_2MHZ  = 0x01,
        const OSPEED_10MHZ = 0x02,
        const OSPEED_50MHZ = 0x03,
    }
}

bitflags! {
    flags PuPd: u32 {
        const PUPD_NOPULL = 0x00,
        const PUPD_UP     = 0x01,
        const PUPD_DOWN   = 0x02,
    }
}

impl GPIO {
    pub fn init(&mut self, pins: Pin,
                mode: Mode, ospeed: OSpeed, otype: OType, pupd: PuPd) {
        let pins = pins.bits();
        for pin in 0..16 {
            if pins | (1 << pin) != 0 {
                self.OSPEEDR.shift_mask_set(ospeed.bits(), OSpeed::all().bits(), pin * 2);
                self.OTYPER.shift_mask_set(otype.bits(), OType::all().bits(), pin);
                self.MODER.shift_mask_set(mode.bits(), Mode::all().bits(), pin * 2);
                self.PUPDR.shift_mask_set(pupd.bits(), PuPd::all().bits(), pin * 2);
            }
        }
    }
}

bitflags! {
    flags Pin: u16 {
        const PIN0  = 1 <<  0,
        const PIN1  = 1 <<  1,
        const PIN2  = 1 <<  2,
        const PIN3  = 1 <<  3,
        const PIN4  = 1 <<  4,
        const PIN5  = 1 <<  5,
        const PIN6  = 1 <<  6,
        const PIN7  = 1 <<  7,
        const PIN8  = 1 <<  8,
        const PIN9  = 1 <<  9,
        const PIN10 = 1 << 10,
        const PIN11 = 1 << 11,
        const PIN12 = 1 << 12,
        const PIN13 = 1 << 13,
        const PIN14 = 1 << 14,
        const PIN15 = 1 << 15,
    }
}

pub struct Gpio {
    pub gpio: RegPtr<GPIO>,
    pub pin:  Pin,
    pub clk:  rcc::AHBENR,
}

impl Gpio {
    pub const fn new(gpio: RegPtr<GPIO>, pin: Pin, clk: rcc::AHBENR) -> Self {
        Gpio {
            gpio: gpio,
            pin: pin,
            clk: clk,
        }
    }

    pub fn init(&mut self,
                mode: Mode, ospeed: OSpeed, otype: OType, pupd: PuPd) {
        //TODO report compiler bug
        //rcc::RCC.AHBENR |= self.clk.bits();
        (*rcc::RCC).AHBENR |= self.clk.bits();
        
        self.gpio.init(self.pin, mode, ospeed, otype, pupd);
    }

    pub fn toggle(&mut self) {
        let gpio = &mut *self.gpio;
        let pin = self.pin;

        gpio.ODR ^= pin.bits();
    }
}

