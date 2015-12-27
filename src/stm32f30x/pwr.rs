use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct PWR {
    pub CR:                 u32,
    pub CSR:                u32,
}

//register addresses

registers! {
    const PWR:          PWR             = PWR_BASE,
}

//custom

