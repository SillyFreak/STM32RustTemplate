use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct OPAMP {
    pub CSR:                u32,                                        //OPAMP control and status register,            Address offset: 0x00
}

//register addresses

registers! {
    const OPAMP:            OPAMP               = OPAMP_BASE,
    const OPAMP1:           OPAMP               = OPAMP1_BASE,
    const OPAMP2:           OPAMP               = OPAMP2_BASE,
    const OPAMP3:           OPAMP               = OPAMP3_BASE,
    const OPAMP4:           OPAMP               = OPAMP4_BASE,
}

//bit definitions

constants! {
    CSR: u32 {
        const EN                                = 0x00000001,           //OPAMP1 enable
        const FORCEVP                           = 0x00000002,           //Connect the internal references to the plus input of the OPAMPX
        const VPSEL                             = 0x0000000C,           //Non inverting input selection
        const VPSEL_0                           = 0x00000004,           //Bit 0
        const VPSEL_1                           = 0x00000008,           //Bit 1
        const VMSEL                             = 0x00000060,           //Inverting input selection
        const VMSEL_0                           = 0x00000020,           //Bit 0
        const VMSEL_1                           = 0x00000040,           //Bit 1
        const TCMEN                             = 0x00000080,           //Timer-Controlled Mux mode enable
        const VMSSEL                            = 0x00000100,           //Inverting input secondary selection
        const VPSSEL                            = 0x00000600,           //Non inverting input secondary selection
        const VPSSEL_0                          = 0x00000200,           //Bit 0
        const VPSSEL_1                          = 0x00000400,           //Bit 1
        const CALON                             = 0x00000800,           //Calibration mode enable
        const CALSEL                            = 0x00003000,           //Calibration selection
        const CALSEL_0                          = 0x00001000,           //Bit 0
        const CALSEL_1                          = 0x00002000,           //Bit 1
        const PGGAIN                            = 0x0003C000,           //Gain in PGA mode
        const PGGAIN_0                          = 0x00004000,           //Bit 0
        const PGGAIN_1                          = 0x00008000,           //Bit 1
        const PGGAIN_2                          = 0x00010000,           //Bit 2
        const PGGAIN_3                          = 0x00020000,           //Bit 3
        const USERTRIM                          = 0x00040000,           //User trimming enable
        const TRIMOFFSETP                       = 0x00F80000,           //Offset trimming value (PMOS)
        const TRIMOFFSETN                       = 0x1F000000,           //Offset trimming value (NMOS)
        const TSTREF                            = 0x20000000,           //It enables the switch to put out the internal reference
        const OUTCAL                            = 0x40000000,           //OPAMP output status flag
        const LOCK                              = 0x80000000,           //OPAMP lock
    }
}

//custom

