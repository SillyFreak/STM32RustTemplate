use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct DAC {
    pub CR:                 u32,                                        //DAC control register,                                    Address offset: 0x00
    pub SWTRIGR:            u32,                                        //DAC software trigger register,                           Address offset: 0x04
    pub DHR12R1:            u32,                                        //DAC channel1 12-bit right-aligned data holding register, Address offset: 0x08
    pub DHR12L1:            u32,                                        //DAC channel1 12-bit left aligned data holding register,  Address offset: 0x0C
    pub DHR8R1:             u32,                                        //DAC channel1 8-bit right aligned data holding register,  Address offset: 0x10
    pub DHR12R2:            u32,                                        //DAC channel2 12-bit right aligned data holding register, Address offset: 0x14
    pub DHR12L2:            u32,                                        //DAC channel2 12-bit left aligned data holding register,  Address offset: 0x18
    pub DHR8R2:             u32,                                        //DAC channel2 8-bit right-aligned data holding register,  Address offset: 0x1C
    pub DHR12RD:            u32,                                        //Dual DAC 12-bit right-aligned data holding register,     Address offset: 0x20
    pub DHR12LD:            u32,                                        //DUAL DAC 12-bit left aligned data holding register,      Address offset: 0x24
    pub DHR8RD:             u32,                                        //DUAL DAC 8-bit right aligned data holding register,      Address offset: 0x28
    pub DOR1:               u32,                                        //DAC channel1 data output register,                       Address offset: 0x2C
    pub DOR2:               u32,                                        //DAC channel2 data output register,                       Address offset: 0x30
    pub SR:                 u32,                                        //DAC status register,                                     Address offset: 0x34
}

//register addresses

registers! {
    const DAC1:             DAC                 = DAC1_BASE,
    const DAC2:             DAC                 = DAC2_BASE,
}

//bit definitions

constants! {
    CR: u32 {
        const EN1                               = 0x00000001,           //DAC channel1 enable
        const BOFF1                             = 0x00000002,           //DAC channel1 output buffer disable
        const TEN1                              = 0x00000004,           //DAC channel1 Trigger enable
        const TSEL1                             = 0x00000038,           //TSEL1[2:0] (DAC channel1 Trigger selection)
        const TSEL1_0                           = 0x00000008,           //Bit 0
        const TSEL1_1                           = 0x00000010,           //Bit 1
        const TSEL1_2                           = 0x00000020,           //Bit 2
        const WAVE1                             = 0x000000C0,           //WAVE1[1:0] (DAC channel1 noise/triangle wave generation enable)
        const WAVE1_0                           = 0x00000040,           //Bit 0
        const WAVE1_1                           = 0x00000080,           //Bit 1
        const MAMP1                             = 0x00000F00,           //MAMP1[3:0] (DAC channel1 Mask/Amplitude selector)
        const MAMP1_0                           = 0x00000100,           //Bit 0
        const MAMP1_1                           = 0x00000200,           //Bit 1
        const MAMP1_2                           = 0x00000400,           //Bit 2
        const MAMP1_3                           = 0x00000800,           //Bit 3
        const DMAEN1                            = 0x00001000,           //DAC channel1 DMA enable
        const DMAUDRIE1                         = 0x00002000,           //DAC channel1 DMA underrun IT enable
        const EN2                               = 0x00010000,           //DAC channel2 enable
        const BOFF2                             = 0x00020000,           //DAC channel2 output buffer disable
        const TEN2                              = 0x00040000,           //DAC channel2 Trigger enable
        const TSEL2                             = 0x00380000,           //TSEL2[2:0] (DAC channel2 Trigger selection)
        const TSEL2_0                           = 0x00080000,           //Bit 0
        const TSEL2_1                           = 0x00100000,           //Bit 1
        const TSEL2_2                           = 0x00200000,           //Bit 2
        const WAVE2                             = 0x00C00000,           //WAVE2[1:0] (DAC channel2 noise/triangle wave generation enable)
        const WAVE2_0                           = 0x00400000,           //Bit 0
        const WAVE2_1                           = 0x00800000,           //Bit 1
        const MAMP2                             = 0x0F000000,           //MAMP2[3:0] (DAC channel2 Mask/Amplitude selector)
        const MAMP2_0                           = 0x01000000,           //Bit 0
        const MAMP2_1                           = 0x02000000,           //Bit 1
        const MAMP2_2                           = 0x04000000,           //Bit 2
        const MAMP2_3                           = 0x08000000,           //Bit 3
        const DMAEN2                            = 0x10000000,           //DAC channel2 DMA enabled
        const DMAUDRIE2                         = 0x20000000,           //DAC channel2 DMA underrun IT enable
    }

    SWTRIGR: u32 {
        const SWTRIG1                           = 0x01,                 //DAC channel1 software trigger
        const SWTRIG2                           = 0x02,                 //DAC channel2 software trigger
    }

    DHR12R1: u32 {
        const DACC1DHR                          = 0x0FFF,               //DAC channel1 12-bit Right aligned data
    }

    DHR12L1: u32 {
        const DACC1DHR                          = 0xFFF0,               //DAC channel1 12-bit Left aligned data
    }

    DHR8R1: u32 {
        const DACC1DHR                          = 0xFF,                 //DAC channel1 8-bit Right aligned data
    }

    DHR12R2: u32 {
        const DACC2DHR                          = 0x0FFF,               //DAC channel2 12-bit Right aligned data
    }

    DHR12L2: u32 {
        const DACC2DHR                          = 0xFFF0,               //DAC channel2 12-bit Left aligned data
    }

    DHR8R2: u32 {
        const DACC2DHR                          = 0xFF,                 //DAC channel2 8-bit Right aligned data
    }

    DHR12RD: u32 {
        const DACC1DHR                          = 0x00000FFF,           //DAC channel1 12-bit Right aligned data
        const DACC2DHR                          = 0x0FFF0000,           //DAC channel2 12-bit Right aligned data
    }

    DHR12LD: u32 {
        const DACC1DHR                          = 0x0000FFF0,           //DAC channel1 12-bit Left aligned data
        const DACC2DHR                          = 0xFFF00000,           //DAC channel2 12-bit Left aligned data
    }

    DHR8RD: u32 {
        const DACC1DHR                          = 0x00FF,               //DAC channel1 8-bit Right aligned data
        const DACC2DHR                          = 0xFF00,               //DAC channel2 8-bit Right aligned data
    }

    DOR1: u32 {
        const DACC1DOR                          = 0x0FFF,               //DAC channel1 data output
    }

    DOR2: u32 {
        const DACC2DOR                          = 0x0FFF,               //DAC channel2 data output
    }

    SR: u32 {
        const DMAUDR1                           = 0x00002000,           //DAC channel1 DMA underrun flag
        const DMAUDR2                           = 0x20000000,           //DAC channel2 DMA underrun flag
    }
}

//custom

