use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct TSC {
    pub CR:                 u32,
    pub IER:                u32,
    pub ICR:                u32,
    pub ISR:                u32,
    pub IOHCR:              u32,
    pub RESERVED1:          u32,
    pub IOASCR:             u32,
    pub RESERVED2:          u32,
    pub IOSCR:              u32,
    pub RESERVED3:          u32,
    pub IOCCR:              u32,
    pub RESERVED4:          u32,
    pub IOGCSR:             u32,
    pub IOGXCR:             [u32; 8],
}

//register addresses

registers! {
    const TSC:          TSC             = TSC_BASE,
}

//custom

