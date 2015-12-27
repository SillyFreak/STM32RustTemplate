use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct EXTI {
    pub IMR:                u32,
    pub EMR:                u32,
    pub RTSR:               u32,
    pub FTSR:               u32,
    pub SWIER:              u32,
    pub PR:                 u32,
    pub RESERVED1:          u32,
    pub RESERVED2:          u32,
    pub IMR2:               u32,
    pub EMR2:               u32,
    pub RTSR2:              u32,
    pub FTSR2:              u32,
    pub SWIER2:             u32,
    pub PR2:                u32,
}

//register addresses

registers! {
    const EXTI:         EXTI            = EXTI_BASE,
}

//custom

