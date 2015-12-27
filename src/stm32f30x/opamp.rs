use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct OPAMP {
    pub CSR:                u32,
}

//register addresses

registers! {
    const OPAMP:        OPAMP           = OPAMP_BASE,
    const OPAMP1:       OPAMP           = OPAMP1_BASE,
    const OPAMP2:       OPAMP           = OPAMP2_BASE,
    const OPAMP3:       OPAMP           = OPAMP3_BASE,
    const OPAMP4:       OPAMP           = OPAMP4_BASE,
}

//custom

