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
        const MODER0                            = 0x00000003,
        const MODER0_0                          = 0x00000001,
        const MODER0_1                          = 0x00000002,
        const MODER1                            = 0x0000000C,
        const MODER1_0                          = 0x00000004,
        const MODER1_1                          = 0x00000008,
        const MODER2                            = 0x00000030,
        const MODER2_0                          = 0x00000010,
        const MODER2_1                          = 0x00000020,
        const MODER3                            = 0x000000C0,
        const MODER3_0                          = 0x00000040,
        const MODER3_1                          = 0x00000080,
        const MODER4                            = 0x00000300,
        const MODER4_0                          = 0x00000100,
        const MODER4_1                          = 0x00000200,
        const MODER5                            = 0x00000C00,
        const MODER5_0                          = 0x00000400,
        const MODER5_1                          = 0x00000800,
        const MODER6                            = 0x00003000,
        const MODER6_0                          = 0x00001000,
        const MODER6_1                          = 0x00002000,
        const MODER7                            = 0x0000C000,
        const MODER7_0                          = 0x00004000,
        const MODER7_1                          = 0x00008000,
        const MODER8                            = 0x00030000,
        const MODER8_0                          = 0x00010000,
        const MODER8_1                          = 0x00020000,
        const MODER9                            = 0x000C0000,
        const MODER9_0                          = 0x00040000,
        const MODER9_1                          = 0x00080000,
        const MODER10                           = 0x00300000,
        const MODER10_0                         = 0x00100000,
        const MODER10_1                         = 0x00200000,
        const MODER11                           = 0x00C00000,
        const MODER11_0                         = 0x00400000,
        const MODER11_1                         = 0x00800000,
        const MODER12                           = 0x03000000,
        const MODER12_0                         = 0x01000000,
        const MODER12_1                         = 0x02000000,
        const MODER13                           = 0x0C000000,
        const MODER13_0                         = 0x04000000,
        const MODER13_1                         = 0x08000000,
        const MODER14                           = 0x30000000,
        const MODER14_0                         = 0x10000000,
        const MODER14_1                         = 0x20000000,
        const MODER15                           = 0xC0000000,
        const MODER15_0                         = 0x40000000,
        const MODER15_1                         = 0x80000000,
    }

    OTYPER: u16 {
        const OT_0                              = 0x00000001,
        const OT_1                              = 0x00000002,
        const OT_2                              = 0x00000004,
        const OT_3                              = 0x00000008,
        const OT_4                              = 0x00000010,
        const OT_5                              = 0x00000020,
        const OT_6                              = 0x00000040,
        const OT_7                              = 0x00000080,
        const OT_8                              = 0x00000100,
        const OT_9                              = 0x00000200,
        const OT_10                             = 0x00000400,
        const OT_11                             = 0x00000800,
        const OT_12                             = 0x00001000,
        const OT_13                             = 0x00002000,
        const OT_14                             = 0x00004000,
        const OT_15                             = 0x00008000,
    }

    PUPDR: u32 {
        const PUPDR0                            = 0x00000003,
        const PUPDR0_0                          = 0x00000001,
        const PUPDR0_1                          = 0x00000002,
        const PUPDR1                            = 0x0000000C,
        const PUPDR1_0                          = 0x00000004,
        const PUPDR1_1                          = 0x00000008,
        const PUPDR2                            = 0x00000030,
        const PUPDR2_0                          = 0x00000010,
        const PUPDR2_1                          = 0x00000020,
        const PUPDR3                            = 0x000000C0,
        const PUPDR3_0                          = 0x00000040,
        const PUPDR3_1                          = 0x00000080,
        const PUPDR4                            = 0x00000300,
        const PUPDR4_0                          = 0x00000100,
        const PUPDR4_1                          = 0x00000200,
        const PUPDR5                            = 0x00000C00,
        const PUPDR5_0                          = 0x00000400,
        const PUPDR5_1                          = 0x00000800,
        const PUPDR6                            = 0x00003000,
        const PUPDR6_0                          = 0x00001000,
        const PUPDR6_1                          = 0x00002000,
        const PUPDR7                            = 0x0000C000,
        const PUPDR7_0                          = 0x00004000,
        const PUPDR7_1                          = 0x00008000,
        const PUPDR8                            = 0x00030000,
        const PUPDR8_0                          = 0x00010000,
        const PUPDR8_1                          = 0x00020000,
        const PUPDR9                            = 0x000C0000,
        const PUPDR9_0                          = 0x00040000,
        const PUPDR9_1                          = 0x00080000,
        const PUPDR10                           = 0x00300000,
        const PUPDR10_0                         = 0x00100000,
        const PUPDR10_1                         = 0x00200000,
        const PUPDR11                           = 0x00C00000,
        const PUPDR11_0                         = 0x00400000,
        const PUPDR11_1                         = 0x00800000,
        const PUPDR12                           = 0x03000000,
        const PUPDR12_0                         = 0x01000000,
        const PUPDR12_1                         = 0x02000000,
        const PUPDR13                           = 0x0C000000,
        const PUPDR13_0                         = 0x04000000,
        const PUPDR13_1                         = 0x08000000,
        const PUPDR14                           = 0x30000000,
        const PUPDR14_0                         = 0x10000000,
        const PUPDR14_1                         = 0x20000000,
        const PUPDR15                           = 0xC0000000,
        const PUPDR15_0                         = 0x40000000,
        const PUPDR15_1                         = 0x80000000,
    }

    IDR: u16 {
        const _0                                = 0x00000001,
        const _1                                = 0x00000002,
        const _2                                = 0x00000004,
        const _3                                = 0x00000008,
        const _4                                = 0x00000010,
        const _5                                = 0x00000020,
        const _6                                = 0x00000040,
        const _7                                = 0x00000080,
        const _8                                = 0x00000100,
        const _9                                = 0x00000200,
        const _10                               = 0x00000400,
        const _11                               = 0x00000800,
        const _12                               = 0x00001000,
        const _13                               = 0x00002000,
        const _14                               = 0x00004000,
        const _15                               = 0x00008000,
    }

    ODR: u16 {
        const _0                                = 0x00000001,
        const _1                                = 0x00000002,
        const _2                                = 0x00000004,
        const _3                                = 0x00000008,
        const _4                                = 0x00000010,
        const _5                                = 0x00000020,
        const _6                                = 0x00000040,
        const _7                                = 0x00000080,
        const _8                                = 0x00000100,
        const _9                                = 0x00000200,
        const _10                               = 0x00000400,
        const _11                               = 0x00000800,
        const _12                               = 0x00001000,
        const _13                               = 0x00002000,
        const _14                               = 0x00004000,
        const _15                               = 0x00008000,
    }

    BSRR: u32 {
        const BS_0                              = 0x00000001,
        const BS_1                              = 0x00000002,
        const BS_2                              = 0x00000004,
        const BS_3                              = 0x00000008,
        const BS_4                              = 0x00000010,
        const BS_5                              = 0x00000020,
        const BS_6                              = 0x00000040,
        const BS_7                              = 0x00000080,
        const BS_8                              = 0x00000100,
        const BS_9                              = 0x00000200,
        const BS_10                             = 0x00000400,
        const BS_11                             = 0x00000800,
        const BS_12                             = 0x00001000,
        const BS_13                             = 0x00002000,
        const BS_14                             = 0x00004000,
        const BS_15                             = 0x00008000,
        const BR_0                              = 0x00010000,
        const BR_1                              = 0x00020000,
        const BR_2                              = 0x00040000,
        const BR_3                              = 0x00080000,
        const BR_4                              = 0x00100000,
        const BR_5                              = 0x00200000,
        const BR_6                              = 0x00400000,
        const BR_7                              = 0x00800000,
        const BR_8                              = 0x01000000,
        const BR_9                              = 0x02000000,
        const BR_10                             = 0x04000000,
        const BR_11                             = 0x08000000,
        const BR_12                             = 0x10000000,
        const BR_13                             = 0x20000000,
        const BR_14                             = 0x40000000,
        const BR_15                             = 0x80000000,
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

    BRR: u16 {
        const BR_0                              = 0x00000001,
        const BR_1                              = 0x00000002,
        const BR_2                              = 0x00000004,
        const BR_3                              = 0x00000008,
        const BR_4                              = 0x00000010,
        const BR_5                              = 0x00000020,
        const BR_6                              = 0x00000040,
        const BR_7                              = 0x00000080,
        const BR_8                              = 0x00000100,
        const BR_9                              = 0x00000200,
        const BR_10                             = 0x00000400,
        const BR_11                             = 0x00000800,
        const BR_12                             = 0x00001000,
        const BR_13                             = 0x00002000,
        const BR_14                             = 0x00004000,
        const BR_15                             = 0x00008000,
    }
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
    pub fn set_mode(&mut self, pin: u8, mode: Mode) {
        self.MODER.shift_mask_set(mode.bits(), Mode::all().bits(), pin * 2);
    }

    pub fn set_otype(&mut self, pin: u8, otype: OType) {
        self.OTYPER.shift_mask_set(otype.bits(), OType::all().bits(), pin);
    }

    pub fn set_ospeed(&mut self, pin: u8, ospeed: OSpeed) {
        self.OSPEEDR.shift_mask_set(ospeed.bits(), OSpeed::all().bits(), pin * 2);
    }

    pub fn set_pupd(&mut self, pin: u8, pupd: PuPd) {
        self.PUPDR.shift_mask_set(pupd.bits(), PuPd::all().bits(), pin * 2);
    }

    pub fn set_af(&mut self, pin: u8, af: u8) {
        let index = pin as usize >> 3;
        let off = pin & 0x07;
        self.AFR[index].shift_mask_set(af as u32, 0x0F, off * 4);
    }

    pub fn init(&mut self, pins: Pin,
                mode: Mode, ospeed: OSpeed, otype: OType, pupd: PuPd) {
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
                mode: Mode, ospeed: OSpeed, otype: OType, pupd: PuPd) {
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

