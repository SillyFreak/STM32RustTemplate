use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct WWDG {
    pub CR:                 u32,
    pub CFR:                u32,
    pub SR:                 u32,
}

//register addresses

registers! {
    const WWDG:         WWDG            = WWDG_BASE,
}

//custom

