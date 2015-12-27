use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct IWDG {
    pub KR:                 u32,
    pub PR:                 u32,
    pub RLR:                u32,
    pub SR:                 u32,
    pub WINR:               u32,
}

//register addresses

registers! {
    const IWDG:         IWDG            = IWDG_BASE,
}

//custom

