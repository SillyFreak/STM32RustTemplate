use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct SYSCFG {
    pub CFGR1:              u32,
    pub RCR:                u32,
    pub EXTICR:             [u32; 4],
    pub CFGR2:              u32,
    pub RESERVED0:          u32,
    pub RESERVED1:          u32,
    pub RESERVED2:          u32,
    pub RESERVED4:          u32,
    pub RESERVED5:          u32,
    pub RESERVED6:          u32,
    pub RESERVED7:          u32,
    pub RESERVED8:          u32,
    pub RESERVED9:          u32,
    pub RESERVED10:         u32,
    pub RESERVED11:         u32,
    pub CFGR4:              u32,
    pub RESERVED13:         u32,
    pub CFGR3:              u32,
}

//register addresses

registers! {
    const SYSCFG:       SYSCFG          = SYSCFG_BASE,
}

//custom

