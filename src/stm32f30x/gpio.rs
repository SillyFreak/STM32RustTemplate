use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct GPIO {
    pub MODER:              u32,                                        //GPIO port mode register,                                  Address offset: 0x00
    pub OTYPER:             u16,                                        //GPIO port output type register,                           Address offset: 0x04
    pub RESERVED0:          u16,                                        //Reserved,                                                                 0x06
    pub OSPEEDR:            u32,                                        //GPIO port output speed register,                          Address offset: 0x08
    pub PUPDR:              u32,                                        //GPIO port pull-up/pull-down register,                     Address offset: 0x0C
    pub IDR:                u16,                                        //GPIO port input data register,                            Address offset: 0x10
    pub RESERVED1:          u16,                                        //Reserved,                                                                 0x12
    pub ODR:                u16,                                        //GPIO port output data register,                           Address offset: 0x14
    pub RESERVED2:          u16,                                        //Reserved,                                                                 0x16
    pub BSRR:               u32,                                        //GPIO port bit set/reset registerBSRR,                     Address offset: 0x18
    pub LCKR:               u32,                                        //GPIO port configuration lock register,                    Address offset: 0x1C
    pub AFR:                [u32; 2],                                   //GPIO alternate function low register,                Address offset: 0x20-0x24
    pub BRR:                u16,                                        //GPIO bit reset register,                                  Address offset: 0x28
    pub RESERVED3:          u16,                                        //Reserved,                                                                 0x2A
}

//register addresses

registers! {
    const GPIOA:            GPIO                = GPIOA_BASE,
    const GPIOB:            GPIO                = GPIOB_BASE,
    const GPIOC:            GPIO                = GPIOC_BASE,
    const GPIOD:            GPIO                = GPIOD_BASE,
    const GPIOE:            GPIO                = GPIOE_BASE,
    const GPIOF:            GPIO                = GPIOF_BASE,
    const GPIOG:            GPIO                = GPIOG_BASE,
    const GPIOH:            GPIO                = GPIOH_BASE,
}

//bit definitions

constants! {
    MODER: u32 {
        const IN                                = 0x00,
        const OUT                               = 0x01,
        const AF                                = 0x02,
        const AN                                = 0x03,
        const MASK                              = 0x03,
    }

    OTYPER: u16 {
        const PP                                = 0x00,
        const OD                                = 0x01,
        const MASK                              = 0x01,
    }

    OSPEEDR: u32 {
        const _2MHZ                             = 0x01,
        const _10MHZ                            = 0x02,
        const _50MHZ                            = 0x03,
        const MASK                              = 0x03,
    }

    PUPDR: u32 {
        const NOPULL                            = 0x00,
        const UP                                = 0x01,
        const DOWN                              = 0x02,
        const MASK                              = 0x03,
    }

    LCKR: u32 {
        const LCK0                              = 0x00000001,
        const LCK1                              = 0x00000002,
        const LCK2                              = 0x00000004,
        const LCK3                              = 0x00000008,
        const LCK4                              = 0x00000010,
        const LCK5                              = 0x00000020,
        const LCK6                              = 0x00000040,
        const LCK7                              = 0x00000080,
        const LCK8                              = 0x00000100,
        const LCK9                              = 0x00000200,
        const LCK10                             = 0x00000400,
        const LCK11                             = 0x00000800,
        const LCK12                             = 0x00001000,
        const LCK13                             = 0x00002000,
        const LCK14                             = 0x00004000,
        const LCK15                             = 0x00008000,
        const LCKK                              = 0x00010000,
    }
}

//custom

use hardware::registers::Reg;

impl GPIO {
    pub fn set_mode(&mut self, pin: u8, mode: MODER::Type) {
        self.MODER.shift_mask_set(mode, MODER::MASK, pin * 2);
    }

    pub fn set_otype(&mut self, pin: u8, otype: OTYPER::Type) {
        self.OTYPER.shift_mask_set(otype, OTYPER::MASK, pin);
    }

    pub fn set_ospeed(&mut self, pin: u8, ospeed: OSPEEDR::Type) {
        self.OSPEEDR.shift_mask_set(ospeed, OSPEEDR::MASK, pin * 2);
    }

    pub fn set_pupd(&mut self, pin: u8, pupd: PUPDR::Type) {
        self.PUPDR.shift_mask_set(pupd, PUPDR::MASK, pin * 2);
    }

    pub fn set_af(&mut self, pin: u8, af: u8) {
        let index = (pin as usize >> 3) & 0x01;
        let off = pin & 0x07;
        self.AFR[index].shift_mask_set(af as u32, 0x0F, off * 4);
    }

    pub fn init(&mut self, pins: Pin,
                mode: MODER::Type, ospeed: OSPEEDR::Type, otype: OTYPER::Type, pupd: PUPDR::Type) {
        let pins = pins.bits();
        for pin in 0..16 {
            if pins | (1 << pin) != 0 {
                self.set_ospeed(pin, ospeed);
                self.set_otype(pin, otype);
                self.set_mode(pin, mode);
                self.set_pupd(pin, pupd);
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
    pub clk:  rcc::AHBENR::Type,
}

impl Gpio {
    pub const fn new(gpio: RegPtr<GPIO>, pin: Pin, clk: rcc::AHBENR::Type) -> Self {
        Gpio {
            gpio: gpio,
            pin: pin,
            clk: clk,
        }
    }

    pub fn init(&mut self,
                mode: MODER::Type, ospeed: OSPEEDR::Type, otype: OTYPER::Type, pupd: PUPDR::Type) {
        //TODO report compiler bug
        //rcc::RCC.AHBENR |= self.clk;
        (*rcc::RCC).AHBENR |= self.clk;
        
        self.gpio.init(self.pin, mode, ospeed, otype, pupd);
    }

    pub fn toggle(&mut self) {
        let gpio = &mut *self.gpio;
        let pin = self.pin;

        gpio.ODR ^= pin.bits();
    }
}

