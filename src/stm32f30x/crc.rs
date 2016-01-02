use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct CRC {
    pub DR:                 u32,                                        //CRC Data register,                           Address offset: 0x00
    pub IDR:                u8,                                         //CRC Independent data register,               Address offset: 0x04
    pub RESERVED0:          u8,                                         //Reserved,                                                    0x05
    pub RESERVED1:          u16,                                        //Reserved,                                                    0x06
    pub CR:                 u32,                                        //CRC Control register,                        Address offset: 0x08
    pub RESERVED2:          u32,                                        //Reserved,                                                    0x0C
    pub INIT:               u32,                                        //Initial CRC value register,                  Address offset: 0x10
    pub POL:                u32,                                        //CRC polynomial register,                     Address offset: 0x14
}

//register addresses

registers! {
    const CRC:              CRC                 = CRC_BASE,
}

//bit definitions

constants! {
    DR: u32 {
        const DR                                = 0xFFFFFFFF,           //Data register bits
    }

    IDR: u8 {
        const IDR                               = 0xFF,                 //General-purpose 8-bit data register bits
    }

    CR: u32 {
        const RESET                             = 0x00000001,           //RESET the CRC computation unit bit
        const POLSIZE                           = 0x00000018,           //Polynomial size bits
        const POLSIZE_0                         = 0x00000008,           //Polynomial size bit 0
        const POLSIZE_1                         = 0x00000010,           //Polynomial size bit 1
        const REV_IN                            = 0x00000060,           //REV_IN Reverse Input Data bits
        const REV_IN_0                          = 0x00000020,           //Bit 0
        const REV_IN_1                          = 0x00000040,           //Bit 1
        const REV_OUT                           = 0x00000080,           //REV_OUT Reverse Output Data bits
    }

    INIT: u32 {
        const INIT                              = 0xFFFFFFFF,           //Initial CRC value bits
    }

    POL: u32 {
        const POL                               = 0xFFFFFFFF,           //Coefficients of the polynomial
    }
}

//custom

