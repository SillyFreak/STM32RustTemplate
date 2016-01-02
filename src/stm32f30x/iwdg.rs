use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct IWDG {
    pub KR:                 u32,                                        //IWDG Key register,       Address offset: 0x00
    pub PR:                 u32,                                        //IWDG Prescaler register, Address offset: 0x04
    pub RLR:                u32,                                        //IWDG Reload register,    Address offset: 0x08
    pub SR:                 u32,                                        //IWDG Status register,    Address offset: 0x0C
    pub WINR:               u32,                                        //IWDG Window register,    Address offset: 0x10
}

//register addresses

registers! {
    const IWDG:             IWDG                = IWDG_BASE,
}

//bit definitions

constants! {
    KR: u32 {
        const KEY                               = 0xFFFF,               //Key value (write only, read 0000h)
    }

    PR: u32 {
        const PR                                = 0x07,                 //PR[2:0] (Prescaler divider)
        const PR_0                              = 0x01,                 //Bit 0
        const PR_1                              = 0x02,                 //Bit 1
        const PR_2                              = 0x04,                 //Bit 2
    }

    RLR: u32 {
        const RL                                = 0x0FFF,               //Watchdog counter reload value
    }

    SR: u32 {
        const PVU                               = 0x01,                 //Watchdog prescaler value update
        const RVU                               = 0x02,                 //Watchdog counter reload value update
        const WVU                               = 0x04,                 //Watchdog counter window value update
    }

    WINR: u32 {
        const WIN                               = 0x0FFF,               //Watchdog counter window value
    }
}

//custom

