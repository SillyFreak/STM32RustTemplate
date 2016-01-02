use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct SYSCFG {
    pub CFGR1:              u32,                                        //SYSCFG configuration register 1,                   Address offset: 0x00
    pub RCR:                u32,                                        //SYSCFG CCM SRAM protection register,               Address offset: 0x04
    pub EXTICR:             [u32; 4],                                   //SYSCFG external interrupt configuration registers, Address offset: 0x14-0x08
    pub CFGR2:              u32,                                        //SYSCFG configuration register 2,                    Address offset: 0x18
    pub RESERVED0:          u32,                                        //Reserved,                                                           0x1C
    pub RESERVED1:          u32,                                        //Reserved,                                                          0x20
    pub RESERVED2:          u32,                                        //Reserved,                                                          0x24
    pub RESERVED4:          u32,                                        //Reserved,                                                          0x28
    pub RESERVED5:          u32,                                        //Reserved,                                                          0x2C
    pub RESERVED6:          u32,                                        //Reserved,                                                          0x30
    pub RESERVED7:          u32,                                        //Reserved,                                                          0x34
    pub RESERVED8:          u32,                                        //Reserved,                                                          0x38
    pub RESERVED9:          u32,                                        //Reserved,                                                          0x3C
    pub RESERVED10:         u32,                                        //Reserved,                                                          0x40
    pub RESERVED11:         u32,                                        //Reserved,                                                          0x44
    pub CFGR4:              u32,                                        //SYSCFG configuration register 4,                   Address offset: 0x48
    pub RESERVED13:         u32,                                        //Reserved,                                                          0x4C
    pub CFGR3:              u32,                                        //SYSCFG configuration register 3,                   Address offset: 0x50
}

//register addresses

registers! {
    const SYSCFG:           SYSCFG              = SYSCFG_BASE,
}

//bit definitions

constants! {
    CFGR1: u32 {
        const MEM_MODE                          = 0x00000007,           //SYSCFG_Memory Remap Config
        const MEM_MODE_0                        = 0x00000001,           //Bit 0
        const MEM_MODE_1                        = 0x00000002,           //Bit 1
        const MEM_MODE_2                        = 0x00000004,           //Bit 2
        const USB_IT_RMP                        = 0x00000020,           //USB interrupt remap
        const TIM1_ITR3_RMP                     = 0x00000040,           //Timer 1 ITR3 selection
        const DAC1_TRIG1_RMP                    = 0x00000080,           //DAC1 Trigger1 remap
        const ADC24_DMA_RMP                     = 0x00000100,           //ADC2 and ADC4 DMA remap
        const TIM16_DMA_RMP                     = 0x00000800,           //Timer 16 DMA remap
        const TIM17_DMA_RMP                     = 0x00001000,           //Timer 17 DMA remap
        const TIM6DAC1CH1_DMA_RMP               = 0x00002000,           //Timer 6 / DAC1 CH1 DMA remap
        const TIM7DAC1CH2_DMA_RMP               = 0x00004000,           //Timer 7 / DAC1 CH2 DMA remap
        const DAC2CH1_DMA_RMP                   = 0x00008000,           //DAC2 CH1 DMA remap
        const I2C_PB6_FMP                       = 0x00010000,           //I2C PB6 Fast mode plus
        const I2C_PB7_FMP                       = 0x00020000,           //I2C PB7 Fast mode plus
        const I2C_PB8_FMP                       = 0x00040000,           //I2C PB8 Fast mode plus
        const I2C_PB9_FMP                       = 0x00080000,           //I2C PB9 Fast mode plus
        const I2C1_FMP                          = 0x00100000,           //I2C1 Fast mode plus
        const I2C2_FMP                          = 0x00200000,           //I2C2 Fast mode plus
        const ENCODER_MODE                      = 0x00C00000,           //Encoder Mode
        const ENCODER_MODE_0                    = 0x00400000,           //Encoder Mode 0
        const ENCODER_MODE_1                    = 0x00800000,           //Encoder Mode 1
        const I2C3_FMP                          = 0x01000000,           //I2C3 Fast mode plus
        const FPU_IE                            = 0xFC000000,           //Floating Point Unit Interrupt Enable
        const FPU_IE_0                          = 0x04000000,           //Floating Point Unit Interrupt Enable 0
        const FPU_IE_1                          = 0x08000000,           //Floating Point Unit Interrupt Enable 1
        const FPU_IE_2                          = 0x10000000,           //Floating Point Unit Interrupt Enable 2
        const FPU_IE_3                          = 0x20000000,           //Floating Point Unit Interrupt Enable 3
        const FPU_IE_4                          = 0x40000000,           //Floating Point Unit Interrupt Enable 4
        const FPU_IE_5                          = 0x80000000,           //Floating Point Unit Interrupt Enable 5
    }

    RCR: u32 {
        const PAGE0                             = 0x00000001,           //ICODE SRAM Write protection page 0
        const PAGE1                             = 0x00000002,           //ICODE SRAM Write protection page 1
        const PAGE2                             = 0x00000004,           //ICODE SRAM Write protection page 2
        const PAGE3                             = 0x00000008,           //ICODE SRAM Write protection page 3
        const PAGE4                             = 0x00000010,           //ICODE SRAM Write protection page 4
        const PAGE5                             = 0x00000020,           //ICODE SRAM Write protection page 5
        const PAGE6                             = 0x00000040,           //ICODE SRAM Write protection page 6
        const PAGE7                             = 0x00000080,           //ICODE SRAM Write protection page 7
        const PAGE8                             = 0x00000100,           //ICODE SRAM Write protection page 8
        const PAGE9                             = 0x00000200,           //ICODE SRAM Write protection page 9
        const PAGE10                            = 0x00000400,           //ICODE SRAM Write protection page 10
        const PAGE11                            = 0x00000800,           //ICODE SRAM Write protection page 11
        const PAGE12                            = 0x00001000,           //ICODE SRAM Write protection page 12
        const PAGE13                            = 0x00002000,           //ICODE SRAM Write protection page 13
        const PAGE14                            = 0x00004000,           //ICODE SRAM Write protection page 14
        const PAGE15                            = 0x00008000,           //ICODE SRAM Write protection page 15
    }

    CFGR2: u32 {
        const LOCKUP_LOCK                       = 0x00000001,           //Enables and locks the PVD connection with Timer1/8/15/16/17 Break Input and also the PVD_EN and PVDSEL[2:0] bits of the Power Control Interface
        const SRAM_PARITY_LOCK                  = 0x00000002,           //Enables and locks the SRAM_PARITY error signal with Break Input of TIMER1/8/15/16/17
        const PVD_LOCK                          = 0x00000004,           //Enables and locks the LOCKUP (Hardfault) output of CortexM4 with Break Input of TIMER1/8/15/16/17
        const BYP_ADDR_PAR                      = 0x00000010,           //Disables the address parity check on RAM
        const SRAM_PE                           = 0x00000100,           //SRAM Parity error flag
    }

    CFGR4: u32 {
        const ADC12_EXT2_RMP                    = 0x00000001,           //ADC12 regular channel EXT2 remap
        const ADC12_EXT3_RMP                    = 0x00000002,           //ADC12 regular channel EXT3 remap
        const ADC12_EXT5_RMP                    = 0x00000004,           //ADC12 regular channel EXT5 remap
        const ADC12_EXT13_RMP                   = 0x00000008,           //ADC12 regular channel EXT13 remap
        const ADC12_EXT15_RMP                   = 0x00000010,           //ADC12 regular channel EXT15 remap
        const ADC12_JEXT3_RMP                   = 0x00000020,           //ADC12 injected channel JEXT3 remap
        const ADC12_JEXT6_RMP                   = 0x00000040,           //ADC12 injected channel JEXT6 remap
        const ADC12_JEXT13_RMP                  = 0x00000080,           //ADC12 injected channel JEXT13 remap
        const ADC34_EXT5_RMP                    = 0x00000100,           //ADC34 regular channel EXT5 remap
        const ADC34_EXT6_RMP                    = 0x00000200,           //ADC34 regular channel EXT6 remap
        const ADC34_EXT15_RMP                   = 0x00000400,           //ADC34 regular channel EXT15 remap
        const ADC34_JEXT5_RMP                   = 0x00000800,           //ADC34 injected channel JEXT5 remap
        const ADC34_JEXT11_RMP                  = 0x00001000,           //ADC34 injected channel JEXT11 remap
        const ADC34_JEXT14_RMP                  = 0x00002000,           //ADC34 injected channel JEXT14 remap
    }

    CFGR3: u32 {
        const SPI1_RX_DMA_RMP                   = 0x00000003,           //SPI1 RX DMA remap
        const SPI1_RX_DMA_RMP_0                 = 0x00000001,           //SPI1 RX DMA remap bit 0
        const SPI1_RX_DMA_RMP_1                 = 0x00000002,           //SPI1 RX DMA remap bit 1
        const SPI1_TX_DMA_RMP                   = 0x0000000C,           //SPI1 TX DMA remap
        const SPI1_TX_DMA_RMP_0                 = 0x00000004,           //SPI1 TX DMA remap bit 0
        const SPI1_TX_DMA_RMP_1                 = 0x00000008,           //SPI1 TX DMA remap bit 1
        const I2C1_RX_DMA_RMP                   = 0x00000030,           //I2C1 RX DMA remap
        const I2C1_RX_DMA_RMP_0                 = 0x00000010,           //I2C1 RX DMA remap bit 0
        const I2C1_RX_DMA_RMP_1                 = 0x00000020,           //I2C1 RX DMA remap bit 1
        const I2C1_TX_DMA_RMP                   = 0x000000C0,           //I2C1 RX DMA remap
        const I2C1_TX_DMA_RMP_0                 = 0x00000040,           //I2C1 TX DMA remap bit 0
        const I2C1_TX_DMA_RMP_1                 = 0x00000080,           //I2C1 TX DMA remap bit 1
        const ADC2_DMA_RMP                      = 0x00000300,           //ADC2 DMA remap
        const ADC2_DMA_RMP_0                    = 0x00000100,           //ADC2 DMA remap bit 0
        const ADC2_DMA_RMP_1                    = 0x00000200,           //ADC2 DMA remap bit 1
        const DAC1_TRG3_RMP                     = 0x00010000,           //DAC1 TRG3 remap
        const DAC1_TRG5_RMP                     = 0x00020000,           //DAC1 TRG5 remap
    }
}

//custom

