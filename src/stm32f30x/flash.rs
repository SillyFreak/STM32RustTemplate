use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct FLASH {
    pub ACR:                u32,
    pub KEYR:               u32,
    pub OPTKEYR:            u32,
    pub SR:                 u32,
    pub CR:                 u32,
    pub AR:                 u32,
    pub RESERVED:           u32,
    pub OBR:                u32,
    pub WRPR:               u32,
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct OB {
    pub RDP:                u16,
    pub USER:               u16,
    pub RESERVED0:          u16,
    pub RESERVED1:          u16,
    pub WRP0:               u16,
    pub WRP1:               u16,
    pub WRP2:               u16,
    pub WRP3:               u16,
}

//register addresses

registers! {
    const FLASH:        FLASH           = FLASH_R_BASE,
}

registers! {
    const OB:           OB              = OB_BASE,
}

//custom

