use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct COMP {
    pub CSR:                u32,
}

//register addresses

registers! {
    const COMP:         COMP            = COMP_BASE,
    const COMP1:        COMP            = COMP1_BASE,
    const COMP2:        COMP            = COMP2_BASE,
    const COMP3:        COMP            = COMP3_BASE,
    const COMP4:        COMP            = COMP4_BASE,
    const COMP5:        COMP            = COMP5_BASE,
    const COMP6:        COMP            = COMP6_BASE,
    const COMP7:        COMP            = COMP7_BASE,
}

//custom

