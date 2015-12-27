use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct DBGMCU {
    pub IDCODE:             u32,
    pub CR:                 u32,
    pub APB1FZ:             u32,
    pub APB2FZ:             u32,
}

//register addresses

registers! {
    const DBGMCU:       DBGMCU          = DBGMCU_BASE,
}

//custom

