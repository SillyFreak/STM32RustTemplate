use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct RCC {
    pub CR:       u32, //0x00
    pub CFGR:     u32, //0x04
    pub CIR:      u32, //0x08
    pub APB2RSTR: u32, //0x0C
    pub APB1RSTR: u32, //0x10
    pub AHBENR:   u32, //0x14
    pub APB2ENR:  u32, //0x18
    pub APB1ENR:  u32, //0x1C
    pub BDCR:     u32, //0x20
    pub CSR:      u32, //0x24
    pub AHBRSTR:  u32, //0x28
    pub CFGR2:    u32, //0x2C
    pub CFGR3:    u32, //0x30
}

//register addresses

registers! {
    const RCC: RCC = RCC_BASE,
}

//custom

bitflags! {
    flags AHBENR: u32 {
        const DMA1EN  = 1 <<  0,
        const DMA2EN  = 1 <<  1,
        const SRAMEN  = 1 <<  2,
        const FLITFEN = 1 <<  4,
        const CRCEN   = 1 <<  6,
        const GPIOAEN = 1 << 17,
        const GPIOBEN = 1 << 18,
        const GPIOCEN = 1 << 19,
        const GPIODEN = 1 << 20,
        const GPIOEEN = 1 << 21,
        const GPIOFEN = 1 << 22,
        const TSEN    = 1 << 24,
        const ADC12EN = 1 << 28,
        const ADC34EN = 1 << 29,
    }
}

