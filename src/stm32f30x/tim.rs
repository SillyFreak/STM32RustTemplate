use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct TIM {
    pub CR1:                u16,                                        //TIM control register 1,              Address offset: 0x00
    pub RESERVED0:          u16,                                        //Reserved, 0x02
    pub CR2:                u32,                                        //TIM control register 2,              Address offset: 0x04
    pub SMCR:               u32,                                        //TIM slave mode control register,     Address offset: 0x08
    pub DIER:               u32,                                        //TIM DMA/interrupt enable register,   Address offset: 0x0C
    pub SR:                 u32,                                        //TIM status register,                 Address offset: 0x10
    pub EGR:                u32,                                        //TIM event generation register,       Address offset: 0x14
    pub CCMR1:              u32,                                        //TIM capture/compare mode register 1, Address offset: 0x18
    pub CCMR2:              u32,                                        //TIM capture/compare mode register 2, Address offset: 0x1C
    pub CCER:               u32,                                        //TIM capture/compare enable register, Address offset: 0x20
    pub CNT:                u32,                                        //TIM counter register,                Address offset: 0x24
    pub PSC:                u16,                                        //TIM prescaler,                       Address offset: 0x28
    pub RESERVED9:          u16,                                        //Reserved, 0x2A
    pub ARR:                u32,                                        //TIM auto-reload register,            Address offset: 0x2C
    pub RCR:                u16,                                        //TIM repetition counter register,     Address offset: 0x30
    pub RESERVED10:         u16,                                        //Reserved, 0x32
    pub CCR1:               u32,                                        //TIM capture/compare register 1,      Address offset: 0x34
    pub CCR2:               u32,                                        //TIM capture/compare register 2,      Address offset: 0x38
    pub CCR3:               u32,                                        //TIM capture/compare register 3,      Address offset: 0x3C
    pub CCR4:               u32,                                        //TIM capture/compare register 4,      Address offset: 0x40
    pub BDTR:               u32,                                        //TIM break and dead-time register,    Address offset: 0x44
    pub DCR:                u16,                                        //TIM DMA control register,            Address offset: 0x48
    pub RESERVED12:         u16,                                        //Reserved, 0x4A
    pub DMAR:               u16,                                        //TIM DMA address for full transfer,   Address offset: 0x4C
    pub RESERVED13:         u16,                                        //Reserved, 0x4E
    pub OR:                 u16,                                        //TIM option register,                 Address offset: 0x50
    pub CCMR3:              u32,                                        //TIM capture/compare mode register 3, Address offset: 0x54
    pub CCR5:               u32,                                        //TIM capture/compare register5,      Address offset: 0x58
    pub CCR6:               u32,                                        //TIM capture/compare register 4,      Address offset: 0x5C
}

//register addresses

registers! {
    const TIM2:             TIM                 = TIM2_BASE,
    const TIM3:             TIM                 = TIM3_BASE,
    const TIM4:             TIM                 = TIM4_BASE,
    const TIM6:             TIM                 = TIM6_BASE,
    const TIM7:             TIM                 = TIM7_BASE,
    const TIM1:             TIM                 = TIM1_BASE,
    const TIM8:             TIM                 = TIM8_BASE,
    const TIM15:            TIM                 = TIM15_BASE,
    const TIM16:            TIM                 = TIM16_BASE,
    const TIM17:            TIM                 = TIM17_BASE,
    const TIM20:            TIM                 = TIM20_BASE,
}

//bit definitions

constants! {
    CR1: u16 {
        const CEN                               = 0x0001,               //Counter enable
        const UDIS                              = 0x0002,               //Update disable
        const URS                               = 0x0004,               //Update request source
        const OPM                               = 0x0008,               //One pulse mode
        const DIR                               = 0x0010,               //Direction
        const CMS                               = 0x0060,               //CMS[1:0] bits (Center-aligned mode selection)
        const CMS_0                             = 0x0020,               //Bit 0
        const CMS_1                             = 0x0040,               //Bit 1
        const ARPE                              = 0x0080,               //Auto-reload preload enable
        const CKD                               = 0x0300,               //CKD[1:0] bits (clock division)
        const CKD_0                             = 0x0100,               //Bit 0
        const CKD_1                             = 0x0200,               //Bit 1
        const UIFREMAP                          = 0x0800,               //Update interrupt flag remap
    }

    CR2: u32 {
        const CCPC                              = 0x00000001,           //Capture/Compare Preloaded Control
        const CCUS                              = 0x00000004,           //Capture/Compare Control Update Selection
        const CCDS                              = 0x00000008,           //Capture/Compare DMA Selection
        const MMS                               = 0x00000070,           //MMS[2:0] bits (Master Mode Selection)
        const MMS_0                             = 0x00000010,           //Bit 0
        const MMS_1                             = 0x00000020,           //Bit 1
        const MMS_2                             = 0x00000040,           //Bit 2
        const TI1S                              = 0x00000080,           //TI1 Selection
        const OIS1                              = 0x00000100,           //Output Idle state 1 (OC1 output)
        const OIS1N                             = 0x00000200,           //Output Idle state 1 (OC1N output)
        const OIS2                              = 0x00000400,           //Output Idle state 2 (OC2 output)
        const OIS2N                             = 0x00000800,           //Output Idle state 2 (OC2N output)
        const OIS3                              = 0x00001000,           //Output Idle state 3 (OC3 output)
        const OIS3N                             = 0x00002000,           //Output Idle state 3 (OC3N output)
        const OIS4                              = 0x00004000,           //Output Idle state 4 (OC4 output)
        const OIS5                              = 0x00010000,           //Output Idle state 4 (OC4 output)
        const OIS6                              = 0x00020000,           //Output Idle state 4 (OC4 output)
        const MMS2                              = 0x00F00000,           //MMS[2:0] bits (Master Mode Selection)
        const MMS2_0                            = 0x00100000,           //Bit 0
        const MMS2_1                            = 0x00200000,           //Bit 1
        const MMS2_2                            = 0x00400000,           //Bit 2
        const MMS2_3                            = 0x00800000,           //Bit 2
    }

    SMCR: u32 {
        const SMS                               = 0x00010007,           //SMS[2:0] bits (Slave mode selection)
        const SMS_0                             = 0x00000001,           //Bit 0
        const SMS_1                             = 0x00000002,           //Bit 1
        const SMS_2                             = 0x00000004,           //Bit 2
        const SMS_3                             = 0x00010000,           //Bit 3
        const OCCS                              = 0x00000008,           //OCREF clear selection
        const TS                                = 0x00000070,           //TS[2:0] bits (Trigger selection)
        const TS_0                              = 0x00000010,           //Bit 0
        const TS_1                              = 0x00000020,           //Bit 1
        const TS_2                              = 0x00000040,           //Bit 2
        const MSM                               = 0x00000080,           //Master/slave mode
        const ETF                               = 0x00000F00,           //ETF[3:0] bits (External trigger filter)
        const ETF_0                             = 0x00000100,           //Bit 0
        const ETF_1                             = 0x00000200,           //Bit 1
        const ETF_2                             = 0x00000400,           //Bit 2
        const ETF_3                             = 0x00000800,           //Bit 3
        const ETPS                              = 0x00003000,           //ETPS[1:0] bits (External trigger prescaler)
        const ETPS_0                            = 0x00001000,           //Bit 0
        const ETPS_1                            = 0x00002000,           //Bit 1
        const ECE                               = 0x00004000,           //External clock enable
        const ETP                               = 0x00008000,           //External trigger polarity
    }

    DIER: u32 {
        const UIE                               = 0x0001,               //Update interrupt enable
        const CC1IE                             = 0x0002,               //Capture/Compare 1 interrupt enable
        const CC2IE                             = 0x0004,               //Capture/Compare 2 interrupt enable
        const CC3IE                             = 0x0008,               //Capture/Compare 3 interrupt enable
        const CC4IE                             = 0x0010,               //Capture/Compare 4 interrupt enable
        const COMIE                             = 0x0020,               //COM interrupt enable
        const TIE                               = 0x0040,               //Trigger interrupt enable
        const BIE                               = 0x0080,               //Break interrupt enable
        const UDE                               = 0x0100,               //Update DMA request enable
        const CC1DE                             = 0x0200,               //Capture/Compare 1 DMA request enable
        const CC2DE                             = 0x0400,               //Capture/Compare 2 DMA request enable
        const CC3DE                             = 0x0800,               //Capture/Compare 3 DMA request enable
        const CC4DE                             = 0x1000,               //Capture/Compare 4 DMA request enable
        const COMDE                             = 0x2000,               //COM DMA request enable
        const TDE                               = 0x4000,               //Trigger DMA request enable
    }

    SR: u32 {
        const UIF                               = 0x00000001,           //Update interrupt Flag
        const CC1IF                             = 0x00000002,           //Capture/Compare 1 interrupt Flag
        const CC2IF                             = 0x00000004,           //Capture/Compare 2 interrupt Flag
        const CC3IF                             = 0x00000008,           //Capture/Compare 3 interrupt Flag
        const CC4IF                             = 0x00000010,           //Capture/Compare 4 interrupt Flag
        const COMIF                             = 0x00000020,           //COM interrupt Flag
        const TIF                               = 0x00000040,           //Trigger interrupt Flag
        const BIF                               = 0x00000080,           //Break interrupt Flag
        const B2IF                              = 0x00000100,           //Break2 interrupt Flag
        const CC1OF                             = 0x00000200,           //Capture/Compare 1 Over capture Flag
        const CC2OF                             = 0x00000400,           //Capture/Compare 2 Over capture Flag
        const CC3OF                             = 0x00000800,           //Capture/Compare 3 Over capture Flag
        const CC4OF                             = 0x00001000,           //Capture/Compare 4 Over capture Flag
        const CC5IF                             = 0x00010000,           //Capture/Compare 5 interrupt Flag
        const CC6IF                             = 0x00020000,           //Capture/Compare 6 interrupt Flag
    }

    EGR: u32 {
        const UG                                = 0x0001,               //Update Generation
        const CC1G                              = 0x0002,               //Capture/Compare 1 Generation
        const CC2G                              = 0x0004,               //Capture/Compare 2 Generation
        const CC3G                              = 0x0008,               //Capture/Compare 3 Generation
        const CC4G                              = 0x0010,               //Capture/Compare 4 Generation
        const COMG                              = 0x0020,               //Capture/Compare Control Update Generation
        const TG                                = 0x0040,               //Trigger Generation
        const BG                                = 0x0080,               //Break Generation
        const B2G                               = 0x0100,               //Break Generation
    }

    CCMR1: u32 {
        const CC1S                              = 0x00000003,           //CC1S[1:0] bits (Capture/Compare 1 Selection)
        const CC1S_0                            = 0x00000001,           //Bit 0
        const CC1S_1                            = 0x00000002,           //Bit 1
        const OC1FE                             = 0x00000004,           //Output Compare 1 Fast enable
        const OC1PE                             = 0x00000008,           //Output Compare 1 Preload enable
        const OC1M                              = 0x00010070,           //OC1M[2:0] bits (Output Compare 1 Mode)
        const OC1M_0                            = 0x00000010,           //Bit 0
        const OC1M_1                            = 0x00000020,           //Bit 1
        const OC1M_2                            = 0x00000040,           //Bit 2
        const OC1M_3                            = 0x00010000,           //Bit 3
        const OC1CE                             = 0x00000080,           //Output Compare 1Clear Enable
        const CC2S                              = 0x00000300,           //CC2S[1:0] bits (Capture/Compare 2 Selection)
        const CC2S_0                            = 0x00000100,           //Bit 0
        const CC2S_1                            = 0x00000200,           //Bit 1
        const OC2FE                             = 0x00000400,           //Output Compare 2 Fast enable
        const OC2PE                             = 0x00000800,           //Output Compare 2 Preload enable
        const OC2M                              = 0x01007000,           //OC2M[2:0] bits (Output Compare 2 Mode)
        const OC2M_0                            = 0x00001000,           //Bit 0
        const OC2M_1                            = 0x00002000,           //Bit 1
        const OC2M_2                            = 0x00004000,           //Bit 2
        const OC2M_3                            = 0x01000000,           //Bit 3
        const OC2CE                             = 0x00008000,           //Output Compare 2 Clear Enable
        const IC1PSC                            = 0x0000000C,           //IC1PSC[1:0] bits (Input Capture 1 Prescaler)
        const IC1PSC_0                          = 0x00000004,           //Bit 0
        const IC1PSC_1                          = 0x00000008,           //Bit 1
        const IC1F                              = 0x000000F0,           //IC1F[3:0] bits (Input Capture 1 Filter)
        const IC1F_0                            = 0x00000010,           //Bit 0
        const IC1F_1                            = 0x00000020,           //Bit 1
        const IC1F_2                            = 0x00000040,           //Bit 2
        const IC1F_3                            = 0x00000080,           //Bit 3
        const IC2PSC                            = 0x00000C00,           //IC2PSC[1:0] bits (Input Capture 2 Prescaler)
        const IC2PSC_0                          = 0x00000400,           //Bit 0
        const IC2PSC_1                          = 0x00000800,           //Bit 1
        const IC2F                              = 0x0000F000,           //IC2F[3:0] bits (Input Capture 2 Filter)
        const IC2F_0                            = 0x00001000,           //Bit 0
        const IC2F_1                            = 0x00002000,           //Bit 1
        const IC2F_2                            = 0x00004000,           //Bit 2
        const IC2F_3                            = 0x00008000,           //Bit 3
    }

    CCMR2: u32 {
        const CC3S                              = 0x00000003,           //CC3S[1:0] bits (Capture/Compare 3 Selection)
        const CC3S_0                            = 0x00000001,           //Bit 0
        const CC3S_1                            = 0x00000002,           //Bit 1
        const OC3FE                             = 0x00000004,           //Output Compare 3 Fast enable
        const OC3PE                             = 0x00000008,           //Output Compare 3 Preload enable
        const OC3M                              = 0x00000070,           //OC3M[2:0] bits (Output Compare 3 Mode)
        const OC3M_0                            = 0x00000010,           //Bit 0
        const OC3M_1                            = 0x00000020,           //Bit 1
        const OC3M_2                            = 0x00000040,           //Bit 2
        const OC3M_3                            = 0x00010000,           //Bit 3
        const OC3CE                             = 0x00000080,           //Output Compare 3 Clear Enable
        const CC4S                              = 0x00000300,           //CC4S[1:0] bits (Capture/Compare 4 Selection)
        const CC4S_0                            = 0x00000100,           //Bit 0
        const CC4S_1                            = 0x00000200,           //Bit 1
        const OC4FE                             = 0x00000400,           //Output Compare 4 Fast enable
        const OC4PE                             = 0x00000800,           //Output Compare 4 Preload enable
        const OC4M                              = 0x00007000,           //OC4M[2:0] bits (Output Compare 4 Mode)
        const OC4M_0                            = 0x00001000,           //Bit 0
        const OC4M_1                            = 0x00002000,           //Bit 1
        const OC4M_2                            = 0x00004000,           //Bit 2
        const OC4M_3                            = 0x00100000,           //Bit 3
        const OC4CE                             = 0x00008000,           //Output Compare 4 Clear Enable
        const IC3PSC                            = 0x0000000C,           //IC3PSC[1:0] bits (Input Capture 3 Prescaler)
        const IC3PSC_0                          = 0x00000004,           //Bit 0
        const IC3PSC_1                          = 0x00000008,           //Bit 1
        const IC3F                              = 0x000000F0,           //IC3F[3:0] bits (Input Capture 3 Filter)
        const IC3F_0                            = 0x00000010,           //Bit 0
        const IC3F_1                            = 0x00000020,           //Bit 1
        const IC3F_2                            = 0x00000040,           //Bit 2
        const IC3F_3                            = 0x00000080,           //Bit 3
        const IC4PSC                            = 0x00000C00,           //IC4PSC[1:0] bits (Input Capture 4 Prescaler)
        const IC4PSC_0                          = 0x00000400,           //Bit 0
        const IC4PSC_1                          = 0x00000800,           //Bit 1
        const IC4F                              = 0x0000F000,           //IC4F[3:0] bits (Input Capture 4 Filter)
        const IC4F_0                            = 0x00001000,           //Bit 0
        const IC4F_1                            = 0x00002000,           //Bit 1
        const IC4F_2                            = 0x00004000,           //Bit 2
        const IC4F_3                            = 0x00008000,           //Bit 3
    }

    CCER: u32 {
        const CC1E                              = 0x00000001,           //Capture/Compare 1 output enable
        const CC1P                              = 0x00000002,           //Capture/Compare 1 output Polarity
        const CC1NE                             = 0x00000004,           //Capture/Compare 1 Complementary output enable
        const CC1NP                             = 0x00000008,           //Capture/Compare 1 Complementary output Polarity
        const CC2E                              = 0x00000010,           //Capture/Compare 2 output enable
        const CC2P                              = 0x00000020,           //Capture/Compare 2 output Polarity
        const CC2NE                             = 0x00000040,           //Capture/Compare 2 Complementary output enable
        const CC2NP                             = 0x00000080,           //Capture/Compare 2 Complementary output Polarity
        const CC3E                              = 0x00000100,           //Capture/Compare 3 output enable
        const CC3P                              = 0x00000200,           //Capture/Compare 3 output Polarity
        const CC3NE                             = 0x00000400,           //Capture/Compare 3 Complementary output enable
        const CC3NP                             = 0x00000800,           //Capture/Compare 3 Complementary output Polarity
        const CC4E                              = 0x00001000,           //Capture/Compare 4 output enable
        const CC4P                              = 0x00002000,           //Capture/Compare 4 output Polarity
        const CC4NP                             = 0x00008000,           //Capture/Compare 4 Complementary output Polarity
        const CC5E                              = 0x00010000,           //Capture/Compare 5 output enable
        const CC5P                              = 0x00020000,           //Capture/Compare 5 output Polarity
        const CC6E                              = 0x00100000,           //Capture/Compare 6 output enable
        const CC6P                              = 0x00200000,           //Capture/Compare 6 output Polarity
    }

    CNT: u32 {
        const CNT                               = 0xFFFFFFFF,           //Counter Value
        const UIFCPY                            = 0x80000000,           //Update interrupt flag copy
    }

    PSC: u16 {
        const PSC                               = 0xFFFF,               //Prescaler Value
    }

    ARR: u32 {
        const ARR                               = 0xFFFFFFFF,           //actual auto-reload Value
    }

    RCR: u16 {
        const REP                               = 0xFF,                 //Repetition Counter Value
    }

    CCR1: u32 {
        const CCR1                              = 0xFFFF,               //Capture/Compare 1 Value
    }

    CCR2: u32 {
        const CCR2                              = 0xFFFF,               //Capture/Compare 2 Value
    }

    CCR3: u32 {
        const CCR3                              = 0xFFFF,               //Capture/Compare 3 Value
    }

    CCR4: u32 {
        const CCR4                              = 0xFFFF,               //Capture/Compare 4 Value
    }

    BDTR: u32 {
        const DTG                               = 0x000000FF,           //DTG[0:7] bits (Dead-Time Generator set-up)
        const DTG_0                             = 0x00000001,           //Bit 0
        const DTG_1                             = 0x00000002,           //Bit 1
        const DTG_2                             = 0x00000004,           //Bit 2
        const DTG_3                             = 0x00000008,           //Bit 3
        const DTG_4                             = 0x00000010,           //Bit 4
        const DTG_5                             = 0x00000020,           //Bit 5
        const DTG_6                             = 0x00000040,           //Bit 6
        const DTG_7                             = 0x00000080,           //Bit 7
        const LOCK                              = 0x00000300,           //LOCK[1:0] bits (Lock Configuration)
        const LOCK_0                            = 0x00000100,           //Bit 0
        const LOCK_1                            = 0x00000200,           //Bit 1
        const OSSI                              = 0x00000400,           //Off-State Selection for Idle mode
        const OSSR                              = 0x00000800,           //Off-State Selection for Run mode
        const BKE                               = 0x00001000,           //Break enable for Break1
        const BKP                               = 0x00002000,           //Break Polarity for Break1
        const AOE                               = 0x00004000,           //Automatic Output enable
        const MOE                               = 0x00008000,           //Main Output enable
        const BKF                               = 0x000F0000,           //Break Filter for Break1
        const BK2F                              = 0x00F00000,           //Break Filter for Break2
        const BK2E                              = 0x01000000,           //Break enable for Break2
        const BK2P                              = 0x02000000,           //Break Polarity for Break2
    }

    DCR: u16 {
        const DBA                               = 0x001F,               //DBA[4:0] bits (DMA Base Address)
        const DBA_0                             = 0x0001,               //Bit 0
        const DBA_1                             = 0x0002,               //Bit 1
        const DBA_2                             = 0x0004,               //Bit 2
        const DBA_3                             = 0x0008,               //Bit 3
        const DBA_4                             = 0x0010,               //Bit 4
        const DBL                               = 0x1F00,               //DBL[4:0] bits (DMA Burst Length)
        const DBL_0                             = 0x0100,               //Bit 0
        const DBL_1                             = 0x0200,               //Bit 1
        const DBL_2                             = 0x0400,               //Bit 2
        const DBL_3                             = 0x0800,               //Bit 3
        const DBL_4                             = 0x1000,               //Bit 4
    }

    DMAR: u16 {
        const DMAB                              = 0xFFFF,               //DMA register for burst accesses
    }

    OR: u16 {
        const TI1_RMP                           = 0x00C0,               //TI1_RMP[1:0] bits (TIM16 Input 1 remap)
        const TI1_RMP_0                         = 0x0040,               //Bit 0
        const TI1_RMP_1                         = 0x0080,               //Bit 1
        const ETR_RMP                           = 0x000F,               //ETR_RMP[3:0] bits (TIM1 ETR remap)
        const ETR_RMP_0                         = 0x0001,               //Bit 0
        const ETR_RMP_1                         = 0x0002,               //Bit 1
        const ETR_RMP_2                         = 0x0004,               //Bit 2
        const ETR_RMP_3                         = 0x0008,               //Bit 3
    }

    CCMR3: u32 {
        const OC5FE                             = 0x00000004,           //Output Compare 5 Fast enable
        const OC5PE                             = 0x00000008,           //Output Compare 5 Preload enable
        const OC5M                              = 0x00000070,           //OC5M[2:0] bits (Output Compare 5 Mode)
        const OC5M_0                            = 0x00000010,           //Bit 0
        const OC5M_1                            = 0x00000020,           //Bit 1
        const OC5M_2                            = 0x00000040,           //Bit 2
        const OC5M_3                            = 0x00010000,           //Bit 3
        const OC5CE                             = 0x00000080,           //Output Compare 5 Clear Enable
        const OC6FE                             = 0x00000400,           //Output Compare 4 Fast enable
        const OC6PE                             = 0x00000800,           //Output Compare 4 Preload enable
        const OC6M                              = 0x00007000,           //OC4M[2:0] bits (Output Compare 4 Mode)
        const OC6M_0                            = 0x00001000,           //Bit 0
        const OC6M_1                            = 0x00002000,           //Bit 1
        const OC6M_2                            = 0x00004000,           //Bit 2
        const OC6M_3                            = 0x00100000,           //Bit 3
        const OC6CE                             = 0x00008000,           //Output Compare 4 Clear Enable
    }

    CCR5: u32 {
        const CCR5                              = 0xFFFFFFFF,           //Capture/Compare 5 Value
        const GC5C1                             = 0x20000000,           //Group Channel 5 and Channel 1
        const GC5C2                             = 0x40000000,           //Group Channel 5 and Channel 2
        const GC5C3                             = 0x80000000,           //Group Channel 5 and Channel 3
    }

    CCR6: u32 {
        const CCR6                              = 0xFFFF,               //Capture/Compare 6 Value
    }
}

//custom

