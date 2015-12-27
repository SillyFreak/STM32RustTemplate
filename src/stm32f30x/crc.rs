use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct CRC {
    pub DR:                 u32,
    pub IDR:                u8,
    pub RESERVED0:          u8,
    pub RESERVED1:          u16,
    pub CR:                 u32,
    pub RESERVED2:          u32,
    pub INIT:               u32,
    pub POL:                u32,
}

//register addresses

registers! {
    const CRC:          CRC             = CRC_BASE,
}

//custom

