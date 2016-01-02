use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct FLASH {
    pub ACR:                u32,                                        //FLASH access control register,              Address offset: 0x00
    pub KEYR:               u32,                                        //FLASH key register,                         Address offset: 0x04
    pub OPTKEYR:            u32,                                        //FLASH option key register,                  Address offset: 0x08
    pub SR:                 u32,                                        //FLASH status register,                      Address offset: 0x0C
    pub CR:                 u32,                                        //FLASH control register,                     Address offset: 0x10
    pub AR:                 u32,                                        //FLASH address register,                     Address offset: 0x14
    pub RESERVED:           u32,                                        //Reserved, 0x18
    pub OBR:                u32,                                        //FLASH Option byte register,                 Address offset: 0x1C
    pub WRPR:               u32,                                        //FLASH Write register,                       Address offset: 0x20
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct OB {
    pub RDP:                u16,                                        //FLASH option byte Read protection,             Address offset: 0x00
    pub USER:               u16,                                        //FLASH option byte user options,                Address offset: 0x02
    pub RESERVED0:          u16,                                        //Reserved,                                                     0x04
    pub RESERVED1:          u16,                                        //Reserved,                                                     0x06
    pub WRP0:               u16,                                        //FLASH option byte write protection 0,          Address offset: 0x08
    pub WRP1:               u16,                                        //FLASH option byte write protection 1,          Address offset: 0x0C
    pub WRP2:               u16,                                        //FLASH option byte write protection 2,          Address offset: 0x10
    pub WRP3:               u16,                                        //FLASH option byte write protection 3,          Address offset: 0x12
}

//register addresses

registers! {
    const FLASH:            FLASH               = FLASH_R_BASE,
}

registers! {
    const OB:               OB                  = OB_BASE,
}

//bit definitions

constants! {
    ACR: u32 {
        const LATENCY                           = 0x03,                 //LATENCY[2:0] bits (Latency)
        const LATENCY_0                         = 0x01,                 //Bit 0
        const LATENCY_1                         = 0x02,                 //Bit 1
        const HLFCYA                            = 0x08,                 //Flash Half Cycle Access Enable
        const PRFTBE                            = 0x10,                 //Prefetch Buffer Enable
        const PRFTBS                            = 0x20,
    }

    KEYR: u32 {
        const FKEYR                             = 0xFFFFFFFF,           //FPEC Key
    }

    OPTKEYR: u32 {
        const OPTKEYR                           = 0xFFFFFFFF,           //Option Byte Key
    }

    SR: u32 {
        const BSY                               = 0x00000001,           //Busy
        const PGERR                             = 0x00000004,           //Programming Error
        const WRPERR                            = 0x00000010,           //Write Protection Error
        const EOP                               = 0x00000020,           //End of operation
    }

    CR: u32 {
        const PG                                = 0x00000001,           //Programming
        const PER                               = 0x00000002,           //Page Erase
        const MER                               = 0x00000004,           //Mass Erase
        const OPTPG                             = 0x00000010,           //Option Byte Programming
        const OPTER                             = 0x00000020,           //Option Byte Erase
        const STRT                              = 0x00000040,           //Start
        const LOCK                              = 0x00000080,           //Lock
        const OPTWRE                            = 0x00000200,           //Option Bytes Write Enable
        const ERRIE                             = 0x00000400,           //Error Interrupt Enable
        const EOPIE                             = 0x00001000,           //End of operation interrupt enable
        const OBL_LAUNCH                        = 0x00002000,           //OptionBytes Loader Launch
    }

    AR: u32 {
        const FAR                               = 0xFFFFFFFF,           //Flash Address
    }

    OBR: u32 {
        const OPTERR                            = 0x00000001,           //Option Byte Error
        const RDPRT1                            = 0x00000002,           //Read protection Level 1
        const RDPRT2                            = 0x00000004,           //Read protection Level 2
        const IWDG_SW                           = 0x00000100,           //IWDG SW
        const NRST_STOP                         = 0x00000200,           //nRST_STOP
        const NRST_STDBY                        = 0x00000400,           //nRST_STDBY
        const NBOOT1                            = 0x00001000,           //nBOOT1
        const VDDA_MONITOR                      = 0x00002000,           //VDDA_MONITOR
        const SRAM_PE                           = 0x00004000,           //SRAM_PE
    }

    WRPR: u32 {
        const WRP                               = 0xFFFFFFFF,           //Write Protect
    }

    RDP: u16 {
        const RDP                               = 0x00FF,               //Read protection option byte
        const NRDP                              = 0xFF00,               //Read protection complemented option byte
    }

    USER: u16 {
        const USER                              = 0x00FF,               //User option byte
        const NUSER                             = 0xFF00,               //User complemented option byte
    }

    WRP0: u16 {
        const WRP0                              = 0x00FF,               //Flash memory write protection option bytes
        const NWRP0                             = 0xFF00,               //Flash memory write protection complemented option bytes
    }

    WRP1: u16 {
        const WRP1                              = 0x00FF,               //Flash memory write protection option bytes
        const NWRP1                             = 0xFF00,               //Flash memory write protection complemented option bytes
    }

    WRP2: u16 {
        const WRP2                              = 0x00FF,               //Flash memory write protection option bytes
        const NWRP2                             = 0xFF00,               //Flash memory write protection complemented option bytes
    }

    WRP3: u16 {
        const WRP3                              = 0x00FF,               //Flash memory write protection option bytes
        const NWRP3                             = 0xFF00,               //Flash memory write protection complemented option bytes
    }
}

//custom

