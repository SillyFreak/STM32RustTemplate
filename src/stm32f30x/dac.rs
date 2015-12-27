use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct DAC {
    pub CR:                 u32,
    pub SWTRIGR:            u32,
    pub DHR12R1:            u32,
    pub DHR12L1:            u32,
    pub DHR8R1:             u32,
    pub DHR12R2:            u32,
    pub DHR12L2:            u32,
    pub DHR8R2:             u32,
    pub DHR12RD:            u32,
    pub DHR12LD:            u32,
    pub DHR8RD:             u32,
    pub DOR1:               u32,
    pub DOR2:               u32,
    pub SR:                 u32,
}

//register addresses

registers! {
    const DAC1:         DAC             = DAC1_BASE,
    const DAC2:         DAC             = DAC2_BASE,
}

//custom

