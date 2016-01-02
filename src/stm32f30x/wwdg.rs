use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct WWDG {
    pub CR:                 u32,                                        //WWDG Control register,       Address offset: 0x00
    pub CFR:                u32,                                        //WWDG Configuration register, Address offset: 0x04
    pub SR:                 u32,                                        //WWDG Status register,        Address offset: 0x08
}

//register addresses

registers! {
    const WWDG:             WWDG                = WWDG_BASE,
}

//bit definitions

constants! {
    CR: u32 {
        const T                                 = 0x7F,                 //T[6:0] bits (7-Bit counter (MSB to LSB))
        const T0                                = 0x01,                 //Bit 0
        const T1                                = 0x02,                 //Bit 1
        const T2                                = 0x04,                 //Bit 2
        const T3                                = 0x08,                 //Bit 3
        const T4                                = 0x10,                 //Bit 4
        const T5                                = 0x20,                 //Bit 5
        const T6                                = 0x40,                 //Bit 6
        const WDGA                              = 0x80,                 //Activation bit
    }

    CFR: u32 {
        const W                                 = 0x007F,               //W[6:0] bits (7-bit window value)
        const W0                                = 0x0001,               //Bit 0
        const W1                                = 0x0002,               //Bit 1
        const W2                                = 0x0004,               //Bit 2
        const W3                                = 0x0008,               //Bit 3
        const W4                                = 0x0010,               //Bit 4
        const W5                                = 0x0020,               //Bit 5
        const W6                                = 0x0040,               //Bit 6
        const WDGTB                             = 0x0180,               //WDGTB[1:0] bits (Timer Base)
        const WDGTB0                            = 0x0080,               //Bit 0
        const WDGTB1                            = 0x0100,               //Bit 1
        const EWI                               = 0x0200,               //Early Wakeup Interrupt
    }

    SR: u32 {
        const EWIF                              = 0x01,                 //Early Wakeup Interrupt Flag
    }
}

//custom

