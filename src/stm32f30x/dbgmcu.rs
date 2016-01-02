use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct DBGMCU {
    pub IDCODE:             u32,                                        //MCU device ID code,               Address offset: 0x00
    pub CR:                 u32,                                        //Debug MCU configuration register, Address offset: 0x04
    pub APB1FZ:             u32,                                        //Debug MCU APB1 freeze register,   Address offset: 0x08
    pub APB2FZ:             u32,                                        //Debug MCU APB2 freeze register,   Address offset: 0x0C
}

//register addresses

registers! {
    const DBGMCU:           DBGMCU              = DBGMCU_BASE,
}

//bit definitions

constants! {
    IDCODE: u32 {
        const DEV_ID                            = 0x00000FFF,
        const REV_ID                            = 0xFFFF0000,
    }

    CR: u32 {
        const DBG_SLEEP                         = 0x00000001,
        const DBG_STOP                          = 0x00000002,
        const DBG_STANDBY                       = 0x00000004,
        const TRACE_IOEN                        = 0x00000020,
        const TRACE_MODE                        = 0x000000C0,
        const TRACE_MODE_0                      = 0x00000040,           //Bit 0
        const TRACE_MODE_1                      = 0x00000080,           //Bit 1
    }
}

//custom

