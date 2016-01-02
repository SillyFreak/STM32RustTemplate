use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct PWR {
    pub CR:                 u32,                                        //PWR power control register,        Address offset: 0x00
    pub CSR:                u32,                                        //PWR power control/status register, Address offset: 0x04
}

//register addresses

registers! {
    const PWR:              PWR                 = PWR_BASE,
}

//bit definitions

constants! {
    CR: u32 {
        const LPSDSR                            = 0x0001,               //Low-power deepsleep/sleep/low power run
        const PDDS                              = 0x0002,               //Power Down Deepsleep
        const CWUF                              = 0x0004,               //Clear Wakeup Flag
        const CSBF                              = 0x0008,               //Clear Standby Flag
        const PVDE                              = 0x0010,               //Power Voltage Detector Enable
        const PLS                               = 0x00E0,               //PLS[2:0] bits (PVD Level Selection)
        const PLS_0                             = 0x0020,               //Bit 0
        const PLS_1                             = 0x0040,               //Bit 1
        const PLS_2                             = 0x0080,               //Bit 2
        const PLS_LEV0                          = 0x0000,               //PVD level 0
        const PLS_LEV1                          = 0x0020,               //PVD level 1
        const PLS_LEV2                          = 0x0040,               //PVD level 2
        const PLS_LEV3                          = 0x0060,               //PVD level 3
        const PLS_LEV4                          = 0x0080,               //PVD level 4
        const PLS_LEV5                          = 0x00A0,               //PVD level 5
        const PLS_LEV6                          = 0x00C0,               //PVD level 6
        const PLS_LEV7                          = 0x00E0,               //PVD level 7
        const DBP                               = 0x0100,               //Disable Backup Domain write protection
    }

    CSR: u32 {
        const WUF                               = 0x0001,               //Wakeup Flag
        const SBF                               = 0x0002,               //Standby Flag
        const PVDO                              = 0x0004,               //PVD Output
        const VREFINTRDYF                       = 0x0008,               //Internal voltage reference (VREFINT) ready flag
        const EWUP1                             = 0x0100,               //Enable WKUP pin 1
        const EWUP2                             = 0x0200,               //Enable WKUP pin 2
        const EWUP3                             = 0x0400,               //Enable WKUP pin 3
    }
}

//custom

