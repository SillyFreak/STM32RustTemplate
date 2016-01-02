use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct ADC {
    pub ISR:                u32,                                        //ADC Interrupt and Status Register,                 Address offset: 0x00
    pub IER:                u32,                                        //ADC Interrupt Enable Register,                     Address offset: 0x04
    pub CR:                 u32,                                        //ADC control register,                              Address offset: 0x08
    pub CFGR:               u32,                                        //ADC Configuration register,                        Address offset: 0x0C
    pub RESERVED0:          u32,                                        //Reserved, 0x010
    pub SMPR1:              u32,                                        //ADC sample time register 1,                        Address offset: 0x14
    pub SMPR2:              u32,                                        //ADC sample time register 2,                        Address offset: 0x18
    pub RESERVED1:          u32,                                        //Reserved, 0x01C
    pub TR1:                u32,                                        //ADC watchdog threshold register 1,                 Address offset: 0x20
    pub TR2:                u32,                                        //ADC watchdog threshold register 2,                 Address offset: 0x24
    pub TR3:                u32,                                        //ADC watchdog threshold register 3,                 Address offset: 0x28
    pub RESERVED2:          u32,                                        //Reserved, 0x02C
    pub SQR1:               u32,                                        //ADC regular sequence register 1,                   Address offset: 0x30
    pub SQR2:               u32,                                        //ADC regular sequence register 2,                   Address offset: 0x34
    pub SQR3:               u32,                                        //ADC regular sequence register 3,                   Address offset: 0x38
    pub SQR4:               u32,                                        //ADC regular sequence register 4,                   Address offset: 0x3C
    pub DR:                 u32,                                        //ADC regular data register,                         Address offset: 0x40
    pub RESERVED3:          u32,                                        //Reserved, 0x044
    pub RESERVED4:          u32,                                        //Reserved, 0x048
    pub JSQR:               u32,                                        //ADC injected sequence register,                    Address offset: 0x4C
    pub RESERVED5:          [u32; 4],                                   //Reserved, 0x050 - 0x05C
    pub OFR1:               u32,                                        //ADC offset register 1,                             Address offset: 0x60
    pub OFR2:               u32,                                        //ADC offset register 2,                             Address offset: 0x64
    pub OFR3:               u32,                                        //ADC offset register 3,                             Address offset: 0x68
    pub OFR4:               u32,                                        //ADC offset register 4,                             Address offset: 0x6C
    pub RESERVED6:          [u32; 4],                                   //Reserved, 0x070 - 0x07C
    pub JDR1:               u32,                                        //ADC injected data register 1,                      Address offset: 0x80
    pub JDR2:               u32,                                        //ADC injected data register 2,                      Address offset: 0x84
    pub JDR3:               u32,                                        //ADC injected data register 3,                      Address offset: 0x88
    pub JDR4:               u32,                                        //ADC injected data register 4,                      Address offset: 0x8C
    pub RESERVED7:          [u32; 4],                                   //Reserved, 0x090 - 0x09C
    pub AWD2CR:             u32,                                        //ADC  Analog Watchdog 2 Configuration Register,     Address offset: 0xA0
    pub AWD3CR:             u32,                                        //ADC  Analog Watchdog 3 Configuration Register,     Address offset: 0xA4
    pub RESERVED8:          u32,                                        //Reserved, 0x0A8
    pub RESERVED9:          u32,                                        //Reserved, 0x0AC
    pub DIFSEL:             u32,                                        //ADC  Differential Mode Selection Register,         Address offset: 0xB0
    pub CALFACT:            u32,                                        //ADC  Calibration Factors,                          Address offset: 0xB4
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct ADC_Common {
    pub CSR:                u32,                                        //ADC Common status register,                  Address offset: ADC1/3 base address + 0x300
    pub RESERVED:           u32,                                        //Reserved, ADC1/3 base address + 0x304
    pub CCR:                u32,                                        //ADC common control register,                 Address offset: ADC1/3 base address + 0x308
    pub CDR:                u32,                                        //ADC common regular data register for dual    Address offset: ADC1/3 base address + 0x30A
}

//register addresses

registers! {
    const ADC1:             ADC                 = ADC1_BASE,
    const ADC2:             ADC                 = ADC2_BASE,
    const ADC3:             ADC                 = ADC3_BASE,
    const ADC4:             ADC                 = ADC4_BASE,
}

registers! {
    const ADC1_2:           ADC_Common          = ADC1_2_BASE,
    const ADC3_4:           ADC_Common          = ADC3_4_BASE,
}

//bit definitions

constants! {
    ISR: u32 {
        const ADRD                              = 0x00000001,           //ADC Ready (ADRDY) flag
        const EOSMP                             = 0x00000002,           //ADC End of Sampling flag
        const EOC                               = 0x00000004,           //ADC End of Regular Conversion flag
        const EOS                               = 0x00000008,           //ADC End of Regular sequence of Conversions flag
        const OVR                               = 0x00000010,           //ADC overrun flag
        const JEOC                              = 0x00000020,           //ADC End of Injected Conversion flag
        const JEOS                              = 0x00000040,           //ADC End of Injected sequence of Conversions flag
        const AWD1                              = 0x00000080,           //ADC Analog watchdog 1 flag
        const AWD2                              = 0x00000100,           //ADC Analog watchdog 2 flag
        const AWD3                              = 0x00000200,           //ADC Analog watchdog 3 flag
        const JQOVF                             = 0x00000400,           //ADC Injected Context Queue Overflow flag
    }

    IER: u32 {
        const RDY                               = 0x00000001,           //ADC Ready (ADRDY) interrupt source
        const EOSMP                             = 0x00000002,           //ADC End of Sampling interrupt source
        const EOC                               = 0x00000004,           //ADC End of Regular Conversion interrupt source
        const EOS                               = 0x00000008,           //ADC End of Regular sequence of Conversions interrupt source
        const OVR                               = 0x00000010,           //ADC overrun interrupt source
        const JEOC                              = 0x00000020,           //ADC End of Injected Conversion interrupt source
        const JEOS                              = 0x00000040,           //ADC End of Injected sequence of Conversions interrupt source
        const AWD1                              = 0x00000080,           //ADC Analog watchdog 1 interrupt source
        const AWD2                              = 0x00000100,           //ADC Analog watchdog 2 interrupt source
        const AWD3                              = 0x00000200,           //ADC Analog watchdog 3 interrupt source
        const JQOVF                             = 0x00000400,           //ADC Injected Context Queue Overflow interrupt source
    }

    CR: u32 {
        const ADEN                              = 0x00000001,           //ADC Enable control
        const ADDIS                             = 0x00000002,           //ADC Disable command
        const ADSTART                           = 0x00000004,           //ADC Start of Regular conversion
        const JADSTART                          = 0x00000008,           //ADC Start of injected conversion
        const ADSTP                             = 0x00000010,           //ADC Stop of Regular conversion
        const JADSTP                            = 0x00000020,           //ADC Stop of injected conversion
        const ADVREGEN                          = 0x30000000,           //ADC Voltage regulator Enable
        const ADVREGEN_0                        = 0x10000000,           //ADC ADVREGEN bit 0
        const ADVREGEN_1                        = 0x20000000,           //ADC ADVREGEN bit 1
        const ADCALDIF                          = 0x40000000,           //ADC Differential Mode for calibration
        const ADCAL                             = 0x80000000,           //ADC Calibration
    }

    CFGR: u32 {
        const DMAEN                             = 0x00000001,           //ADC DMA Enable
        const DMACFG                            = 0x00000002,           //ADC DMA configuration
        const RES                               = 0x00000018,           //ADC Data resolution
        const RES_0                             = 0x00000008,           //ADC RES bit 0
        const RES_1                             = 0x00000010,           //ADC RES bit 1
        const ALIGN                             = 0x00000020,           //ADC Data Alignment
        const EXTSEL                            = 0x000003C0,           //ADC External trigger selection for regular group
        const EXTSEL_0                          = 0x00000040,           //ADC EXTSEL bit 0
        const EXTSEL_1                          = 0x00000080,           //ADC EXTSEL bit 1
        const EXTSEL_2                          = 0x00000100,           //ADC EXTSEL bit 2
        const EXTSEL_3                          = 0x00000200,           //ADC EXTSEL bit 3
        const EXTEN                             = 0x00000C00,           //ADC External trigger enable and polarity selection for regular channels
        const EXTEN_0                           = 0x00000400,           //ADC EXTEN bit 0
        const EXTEN_1                           = 0x00000800,           //ADC EXTEN bit 1
        const OVRMOD                            = 0x00001000,           //ADC overrun mode
        const CONT                              = 0x00002000,           //ADC Single/continuous conversion mode for regular conversion
        const AUTDLY                            = 0x00004000,           //ADC Delayed conversion mode
        const DISCEN                            = 0x00010000,           //ADC Discontinuous mode for regular channels
        const DISCNUM                           = 0x000E0000,           //ADC Discontinuous mode channel count
        const DISCNUM_0                         = 0x00020000,           //ADC DISCNUM bit 0
        const DISCNUM_1                         = 0x00040000,           //ADC DISCNUM bit 1
        const DISCNUM_2                         = 0x00080000,           //ADC DISCNUM bit 2
        const JDISCEN                           = 0x00100000,           //ADC Discontinuous mode on injected channels
        const JQM                               = 0x00200000,           //ADC JSQR Queue mode
        const AWD1SGL                           = 0x00400000,           //Enable the watchdog 1 on a single channel or on all channels
        const AWD1EN                            = 0x00800000,           //ADC Analog watchdog 1 enable on regular Channels
        const JAWD1EN                           = 0x01000000,           //ADC Analog watchdog 1 enable on injected Channels
        const JAUTO                             = 0x02000000,           //ADC Automatic injected group conversion
        const AWD1CH                            = 0x7C000000,           //ADC Analog watchdog 1 Channel selection
        const AWD1CH_0                          = 0x04000000,           //ADC AWD1CH bit 0
        const AWD1CH_1                          = 0x08000000,           //ADC AWD1CH bit 1
        const AWD1CH_2                          = 0x10000000,           //ADC AWD1CH bit 2
        const AWD1CH_3                          = 0x20000000,           //ADC AWD1CH bit 3
        const AWD1CH_4                          = 0x40000000,           //ADC AWD1CH bit 4
    }

    SMPR1: u32 {
        const SMP0                              = 0x00000007,           //ADC Channel 0 Sampling time selection
        const SMP0_0                            = 0x00000001,           //ADC SMP0 bit 0
        const SMP0_1                            = 0x00000002,           //ADC SMP0 bit 1
        const SMP0_2                            = 0x00000004,           //ADC SMP0 bit 2
        const SMP1                              = 0x00000038,           //ADC Channel 1 Sampling time selection
        const SMP1_0                            = 0x00000008,           //ADC SMP1 bit 0
        const SMP1_1                            = 0x00000010,           //ADC SMP1 bit 1
        const SMP1_2                            = 0x00000020,           //ADC SMP1 bit 2
        const SMP2                              = 0x000001C0,           //ADC Channel 2 Sampling time selection
        const SMP2_0                            = 0x00000040,           //ADC SMP2 bit 0
        const SMP2_1                            = 0x00000080,           //ADC SMP2 bit 1
        const SMP2_2                            = 0x00000100,           //ADC SMP2 bit 2
        const SMP3                              = 0x00000E00,           //ADC Channel 3 Sampling time selection
        const SMP3_0                            = 0x00000200,           //ADC SMP3 bit 0
        const SMP3_1                            = 0x00000400,           //ADC SMP3 bit 1
        const SMP3_2                            = 0x00000800,           //ADC SMP3 bit 2
        const SMP4                              = 0x00007000,           //ADC Channel 4 Sampling time selection
        const SMP4_0                            = 0x00001000,           //ADC SMP4 bit 0
        const SMP4_1                            = 0x00002000,           //ADC SMP4 bit 1
        const SMP4_2                            = 0x00004000,           //ADC SMP4 bit 2
        const SMP5                              = 0x00038000,           //ADC Channel 5 Sampling time selection
        const SMP5_0                            = 0x00008000,           //ADC SMP5 bit 0
        const SMP5_1                            = 0x00010000,           //ADC SMP5 bit 1
        const SMP5_2                            = 0x00020000,           //ADC SMP5 bit 2
        const SMP6                              = 0x001C0000,           //ADC Channel 6 Sampling time selection
        const SMP6_0                            = 0x00040000,           //ADC SMP6 bit 0
        const SMP6_1                            = 0x00080000,           //ADC SMP6 bit 1
        const SMP6_2                            = 0x00100000,           //ADC SMP6 bit 2
        const SMP7                              = 0x00E00000,           //ADC Channel 7 Sampling time selection
        const SMP7_0                            = 0x00200000,           //ADC SMP7 bit 0
        const SMP7_1                            = 0x00400000,           //ADC SMP7 bit 1
        const SMP7_2                            = 0x00800000,           //ADC SMP7 bit 2
        const SMP8                              = 0x07000000,           //ADC Channel 8 Sampling time selection
        const SMP8_0                            = 0x01000000,           //ADC SMP8 bit 0
        const SMP8_1                            = 0x02000000,           //ADC SMP8 bit 1
        const SMP8_2                            = 0x04000000,           //ADC SMP8 bit 2
        const SMP9                              = 0x38000000,           //ADC Channel 9 Sampling time selection
        const SMP9_0                            = 0x08000000,           //ADC SMP9 bit 0
        const SMP9_1                            = 0x10000000,           //ADC SMP9 bit 1
        const SMP9_2                            = 0x20000000,           //ADC SMP9 bit 2
    }

    SMPR2: u32 {
        const SMP10                             = 0x00000007,           //ADC Channel 10 Sampling time selection
        const SMP10_0                           = 0x00000001,           //ADC SMP10 bit 0
        const SMP10_1                           = 0x00000002,           //ADC SMP10 bit 1
        const SMP10_2                           = 0x00000004,           //ADC SMP10 bit 2
        const SMP11                             = 0x00000038,           //ADC Channel 11 Sampling time selection
        const SMP11_0                           = 0x00000008,           //ADC SMP11 bit 0
        const SMP11_1                           = 0x00000010,           //ADC SMP11 bit 1
        const SMP11_2                           = 0x00000020,           //ADC SMP11 bit 2
        const SMP12                             = 0x000001C0,           //ADC Channel 12 Sampling time selection
        const SMP12_0                           = 0x00000040,           //ADC SMP12 bit 0
        const SMP12_1                           = 0x00000080,           //ADC SMP12 bit 1
        const SMP12_2                           = 0x00000100,           //ADC SMP12 bit 2
        const SMP13                             = 0x00000E00,           //ADC Channel 13 Sampling time selection
        const SMP13_0                           = 0x00000200,           //ADC SMP13 bit 0
        const SMP13_1                           = 0x00000400,           //ADC SMP13 bit 1
        const SMP13_2                           = 0x00000800,           //ADC SMP13 bit 2
        const SMP14                             = 0x00007000,           //ADC Channel 14 Sampling time selection
        const SMP14_0                           = 0x00001000,           //ADC SMP14 bit 0
        const SMP14_1                           = 0x00002000,           //ADC SMP14 bit 1
        const SMP14_2                           = 0x00004000,           //ADC SMP14 bit 2
        const SMP15                             = 0x00038000,           //ADC Channel 15 Sampling time selection
        const SMP15_0                           = 0x00008000,           //ADC SMP15 bit 0
        const SMP15_1                           = 0x00010000,           //ADC SMP15 bit 1
        const SMP15_2                           = 0x00020000,           //ADC SMP15 bit 2
        const SMP16                             = 0x001C0000,           //ADC Channel 16 Sampling time selection
        const SMP16_0                           = 0x00040000,           //ADC SMP16 bit 0
        const SMP16_1                           = 0x00080000,           //ADC SMP16 bit 1
        const SMP16_2                           = 0x00100000,           //ADC SMP16 bit 2
        const SMP17                             = 0x00E00000,           //ADC Channel 17 Sampling time selection
        const SMP17_0                           = 0x00200000,           //ADC SMP17 bit 0
        const SMP17_1                           = 0x00400000,           //ADC SMP17 bit 1
        const SMP17_2                           = 0x00800000,           //ADC SMP17 bit 2
        const SMP18                             = 0x07000000,           //ADC Channel 18 Sampling time selection
        const SMP18_0                           = 0x01000000,           //ADC SMP18 bit 0
        const SMP18_1                           = 0x02000000,           //ADC SMP18 bit 1
        const SMP18_2                           = 0x04000000,           //ADC SMP18 bit 2
    }

    TR1: u32 {
        const LT1                               = 0x00000FFF,           //ADC Analog watchdog 1 lower threshold
        const LT1_0                             = 0x00000001,           //ADC LT1 bit 0
        const LT1_1                             = 0x00000002,           //ADC LT1 bit 1
        const LT1_2                             = 0x00000004,           //ADC LT1 bit 2
        const LT1_3                             = 0x00000008,           //ADC LT1 bit 3
        const LT1_4                             = 0x00000010,           //ADC LT1 bit 4
        const LT1_5                             = 0x00000020,           //ADC LT1 bit 5
        const LT1_6                             = 0x00000040,           //ADC LT1 bit 6
        const LT1_7                             = 0x00000080,           //ADC LT1 bit 7
        const LT1_8                             = 0x00000100,           //ADC LT1 bit 8
        const LT1_9                             = 0x00000200,           //ADC LT1 bit 9
        const LT1_10                            = 0x00000400,           //ADC LT1 bit 10
        const LT1_11                            = 0x00000800,           //ADC LT1 bit 11
        const HT1                               = 0x0FFF0000,           //ADC Analog watchdog 1 higher threshold
        const HT1_0                             = 0x00010000,           //ADC HT1 bit 0
        const HT1_1                             = 0x00020000,           //ADC HT1 bit 1
        const HT1_2                             = 0x00040000,           //ADC HT1 bit 2
        const HT1_3                             = 0x00080000,           //ADC HT1 bit 3
        const HT1_4                             = 0x00100000,           //ADC HT1 bit 4
        const HT1_5                             = 0x00200000,           //ADC HT1 bit 5
        const HT1_6                             = 0x00400000,           //ADC HT1 bit 6
        const HT1_7                             = 0x00800000,           //ADC HT1 bit 7
        const HT1_8                             = 0x01000000,           //ADC HT1 bit 8
        const HT1_9                             = 0x02000000,           //ADC HT1 bit 9
        const HT1_10                            = 0x04000000,           //ADC HT1 bit 10
        const HT1_11                            = 0x08000000,           //ADC HT1 bit 11
    }

    TR2: u32 {
        const LT2                               = 0x000000FF,           //ADC Analog watchdog 2 lower threshold
        const LT2_0                             = 0x00000001,           //ADC LT2 bit 0
        const LT2_1                             = 0x00000002,           //ADC LT2 bit 1
        const LT2_2                             = 0x00000004,           //ADC LT2 bit 2
        const LT2_3                             = 0x00000008,           //ADC LT2 bit 3
        const LT2_4                             = 0x00000010,           //ADC LT2 bit 4
        const LT2_5                             = 0x00000020,           //ADC LT2 bit 5
        const LT2_6                             = 0x00000040,           //ADC LT2 bit 6
        const LT2_7                             = 0x00000080,           //ADC LT2 bit 7
        const HT2                               = 0x00FF0000,           //ADC Analog watchdog 2 higher threshold
        const HT2_0                             = 0x00010000,           //ADC HT2 bit 0
        const HT2_1                             = 0x00020000,           //ADC HT2 bit 1
        const HT2_2                             = 0x00040000,           //ADC HT2 bit 2
        const HT2_3                             = 0x00080000,           //ADC HT2 bit 3
        const HT2_4                             = 0x00100000,           //ADC HT2 bit 4
        const HT2_5                             = 0x00200000,           //ADC HT2 bit 5
        const HT2_6                             = 0x00400000,           //ADC HT2 bit 6
        const HT2_7                             = 0x00800000,           //ADC HT2 bit 7
    }

    TR3: u32 {
        const LT3                               = 0x000000FF,           //ADC Analog watchdog 3 lower threshold
        const LT3_0                             = 0x00000001,           //ADC LT3 bit 0
        const LT3_1                             = 0x00000002,           //ADC LT3 bit 1
        const LT3_2                             = 0x00000004,           //ADC LT3 bit 2
        const LT3_3                             = 0x00000008,           //ADC LT3 bit 3
        const LT3_4                             = 0x00000010,           //ADC LT3 bit 4
        const LT3_5                             = 0x00000020,           //ADC LT3 bit 5
        const LT3_6                             = 0x00000040,           //ADC LT3 bit 6
        const LT3_7                             = 0x00000080,           //ADC LT3 bit 7
        const HT3                               = 0x00FF0000,           //ADC Analog watchdog 3 higher threshold
        const HT3_0                             = 0x00010000,           //ADC HT3 bit 0
        const HT3_1                             = 0x00020000,           //ADC HT3 bit 1
        const HT3_2                             = 0x00040000,           //ADC HT3 bit 2
        const HT3_3                             = 0x00080000,           //ADC HT3 bit 3
        const HT3_4                             = 0x00100000,           //ADC HT3 bit 4
        const HT3_5                             = 0x00200000,           //ADC HT3 bit 5
        const HT3_6                             = 0x00400000,           //ADC HT3 bit 6
        const HT3_7                             = 0x00800000,           //ADC HT3 bit 7
    }

    SQR1: u32 {
        const L                                 = 0x0000000F,           //ADC regular channel sequence length
        const L_0                               = 0x00000001,           //ADC L bit 0
        const L_1                               = 0x00000002,           //ADC L bit 1
        const L_2                               = 0x00000004,           //ADC L bit 2
        const L_3                               = 0x00000008,           //ADC L bit 3
        const SQ1                               = 0x000007C0,           //ADC 1st conversion in regular sequence
        const SQ1_0                             = 0x00000040,           //ADC SQ1 bit 0
        const SQ1_1                             = 0x00000080,           //ADC SQ1 bit 1
        const SQ1_2                             = 0x00000100,           //ADC SQ1 bit 2
        const SQ1_3                             = 0x00000200,           //ADC SQ1 bit 3
        const SQ1_4                             = 0x00000400,           //ADC SQ1 bit 4
        const SQ2                               = 0x0001F000,           //ADC 2nd conversion in regular sequence
        const SQ2_0                             = 0x00001000,           //ADC SQ2 bit 0
        const SQ2_1                             = 0x00002000,           //ADC SQ2 bit 1
        const SQ2_2                             = 0x00004000,           //ADC SQ2 bit 2
        const SQ2_3                             = 0x00008000,           //ADC SQ2 bit 3
        const SQ2_4                             = 0x00010000,           //ADC SQ2 bit 4
        const SQ3                               = 0x007C0000,           //ADC 3rd conversion in regular sequence
        const SQ3_0                             = 0x00040000,           //ADC SQ3 bit 0
        const SQ3_1                             = 0x00080000,           //ADC SQ3 bit 1
        const SQ3_2                             = 0x00100000,           //ADC SQ3 bit 2
        const SQ3_3                             = 0x00200000,           //ADC SQ3 bit 3
        const SQ3_4                             = 0x00400000,           //ADC SQ3 bit 4
        const SQ4                               = 0x1F000000,           //ADC 4th conversion in regular sequence
        const SQ4_0                             = 0x01000000,           //ADC SQ4 bit 0
        const SQ4_1                             = 0x02000000,           //ADC SQ4 bit 1
        const SQ4_2                             = 0x04000000,           //ADC SQ4 bit 2
        const SQ4_3                             = 0x08000000,           //ADC SQ4 bit 3
        const SQ4_4                             = 0x10000000,           //ADC SQ4 bit 4
    }

    SQR2: u32 {
        const SQ5                               = 0x0000001F,           //ADC 5th conversion in regular sequence
        const SQ5_0                             = 0x00000001,           //ADC SQ5 bit 0
        const SQ5_1                             = 0x00000002,           //ADC SQ5 bit 1
        const SQ5_2                             = 0x00000004,           //ADC SQ5 bit 2
        const SQ5_3                             = 0x00000008,           //ADC SQ5 bit 3
        const SQ5_4                             = 0x00000010,           //ADC SQ5 bit 4
        const SQ6                               = 0x000007C0,           //ADC 6th conversion in regular sequence
        const SQ6_0                             = 0x00000040,           //ADC SQ6 bit 0
        const SQ6_1                             = 0x00000080,           //ADC SQ6 bit 1
        const SQ6_2                             = 0x00000100,           //ADC SQ6 bit 2
        const SQ6_3                             = 0x00000200,           //ADC SQ6 bit 3
        const SQ6_4                             = 0x00000400,           //ADC SQ6 bit 4
        const SQ7                               = 0x0001F000,           //ADC 7th conversion in regular sequence
        const SQ7_0                             = 0x00001000,           //ADC SQ7 bit 0
        const SQ7_1                             = 0x00002000,           //ADC SQ7 bit 1
        const SQ7_2                             = 0x00004000,           //ADC SQ7 bit 2
        const SQ7_3                             = 0x00008000,           //ADC SQ7 bit 3
        const SQ7_4                             = 0x00010000,           //ADC SQ7 bit 4
        const SQ8                               = 0x007C0000,           //ADC 8th conversion in regular sequence
        const SQ8_0                             = 0x00040000,           //ADC SQ8 bit 0
        const SQ8_1                             = 0x00080000,           //ADC SQ8 bit 1
        const SQ8_2                             = 0x00100000,           //ADC SQ8 bit 2
        const SQ8_3                             = 0x00200000,           //ADC SQ8 bit 3
        const SQ8_4                             = 0x00400000,           //ADC SQ8 bit 4
        const SQ9                               = 0x1F000000,           //ADC 9th conversion in regular sequence
        const SQ9_0                             = 0x01000000,           //ADC SQ9 bit 0
        const SQ9_1                             = 0x02000000,           //ADC SQ9 bit 1
        const SQ9_2                             = 0x04000000,           //ADC SQ9 bit 2
        const SQ9_3                             = 0x08000000,           //ADC SQ9 bit 3
        const SQ9_4                             = 0x10000000,           //ADC SQ9 bit 4
    }

    SQR3: u32 {
        const SQ10                              = 0x0000001F,           //ADC 10th conversion in regular sequence
        const SQ10_0                            = 0x00000001,           //ADC SQ10 bit 0
        const SQ10_1                            = 0x00000002,           //ADC SQ10 bit 1
        const SQ10_2                            = 0x00000004,           //ADC SQ10 bit 2
        const SQ10_3                            = 0x00000008,           //ADC SQ10 bit 3
        const SQ10_4                            = 0x00000010,           //ADC SQ10 bit 4
        const SQ11                              = 0x000007C0,           //ADC 11th conversion in regular sequence
        const SQ11_0                            = 0x00000040,           //ADC SQ11 bit 0
        const SQ11_1                            = 0x00000080,           //ADC SQ11 bit 1
        const SQ11_2                            = 0x00000100,           //ADC SQ11 bit 2
        const SQ11_3                            = 0x00000200,           //ADC SQ11 bit 3
        const SQ11_4                            = 0x00000400,           //ADC SQ11 bit 4
        const SQ12                              = 0x0001F000,           //ADC 12th conversion in regular sequence
        const SQ12_0                            = 0x00001000,           //ADC SQ12 bit 0
        const SQ12_1                            = 0x00002000,           //ADC SQ12 bit 1
        const SQ12_2                            = 0x00004000,           //ADC SQ12 bit 2
        const SQ12_3                            = 0x00008000,           //ADC SQ12 bit 3
        const SQ12_4                            = 0x00010000,           //ADC SQ12 bit 4
        const SQ13                              = 0x007C0000,           //ADC 13th conversion in regular sequence
        const SQ13_0                            = 0x00040000,           //ADC SQ13 bit 0
        const SQ13_1                            = 0x00080000,           //ADC SQ13 bit 1
        const SQ13_2                            = 0x00100000,           //ADC SQ13 bit 2
        const SQ13_3                            = 0x00200000,           //ADC SQ13 bit 3
        const SQ13_4                            = 0x00400000,           //ADC SQ13 bit 4
        const SQ14                              = 0x1F000000,           //ADC 14th conversion in regular sequence
        const SQ14_0                            = 0x01000000,           //ADC SQ14 bit 0
        const SQ14_1                            = 0x02000000,           //ADC SQ14 bit 1
        const SQ14_2                            = 0x04000000,           //ADC SQ14 bit 2
        const SQ14_3                            = 0x08000000,           //ADC SQ14 bit 3
        const SQ14_4                            = 0x10000000,           //ADC SQ14 bit 4
    }

    SQR4: u32 {
        const SQ15                              = 0x0000001F,           //ADC 15th conversion in regular sequence
        const SQ15_0                            = 0x00000001,           //ADC SQ15 bit 0
        const SQ15_1                            = 0x00000002,           //ADC SQ15 bit 1
        const SQ15_2                            = 0x00000004,           //ADC SQ15 bit 2
        const SQ15_3                            = 0x00000008,           //ADC SQ15 bit 3
        const SQ15_4                            = 0x00000010,           //ADC SQ105 bit 4
        const SQ16                              = 0x000007C0,           //ADC 16th conversion in regular sequence
        const SQ16_0                            = 0x00000040,           //ADC SQ16 bit 0
        const SQ16_1                            = 0x00000080,           //ADC SQ16 bit 1
        const SQ16_2                            = 0x00000100,           //ADC SQ16 bit 2
        const SQ16_3                            = 0x00000200,           //ADC SQ16 bit 3
        const SQ16_4                            = 0x00000400,           //ADC SQ16 bit 4
    }

    DR: u32 {
        const RDATA                             = 0x0000FFFF,           //ADC regular Data converted
        const RDATA_0                           = 0x00000001,           //ADC RDATA bit 0
        const RDATA_1                           = 0x00000002,           //ADC RDATA bit 1
        const RDATA_2                           = 0x00000004,           //ADC RDATA bit 2
        const RDATA_3                           = 0x00000008,           //ADC RDATA bit 3
        const RDATA_4                           = 0x00000010,           //ADC RDATA bit 4
        const RDATA_5                           = 0x00000020,           //ADC RDATA bit 5
        const RDATA_6                           = 0x00000040,           //ADC RDATA bit 6
        const RDATA_7                           = 0x00000080,           //ADC RDATA bit 7
        const RDATA_8                           = 0x00000100,           //ADC RDATA bit 8
        const RDATA_9                           = 0x00000200,           //ADC RDATA bit 9
        const RDATA_10                          = 0x00000400,           //ADC RDATA bit 10
        const RDATA_11                          = 0x00000800,           //ADC RDATA bit 11
        const RDATA_12                          = 0x00001000,           //ADC RDATA bit 12
        const RDATA_13                          = 0x00002000,           //ADC RDATA bit 13
        const RDATA_14                          = 0x00004000,           //ADC RDATA bit 14
        const RDATA_15                          = 0x00008000,           //ADC RDATA bit 15
    }

    JSQR: u32 {
        const JL                                = 0x00000003,           //ADC injected channel sequence length
        const JL_0                              = 0x00000001,           //ADC JL bit 0
        const JL_1                              = 0x00000002,           //ADC JL bit 1
        const JEXTSEL                           = 0x0000003C,           //ADC external trigger selection for injected group
        const JEXTSEL_0                         = 0x00000004,           //ADC JEXTSEL bit 0
        const JEXTSEL_1                         = 0x00000008,           //ADC JEXTSEL bit 1
        const JEXTSEL_2                         = 0x00000010,           //ADC JEXTSEL bit 2
        const JEXTSEL_3                         = 0x00000020,           //ADC JEXTSEL bit 3
        const JEXTEN                            = 0x000000C0,           //ADC external trigger enable and polarity selection for injected channels
        const JEXTEN_0                          = 0x00000040,           //ADC JEXTEN bit 0
        const JEXTEN_1                          = 0x00000080,           //ADC JEXTEN bit 1
        const JSQ1                              = 0x00001F00,           //ADC 1st conversion in injected sequence
        const JSQ1_0                            = 0x00000100,           //ADC JSQ1 bit 0
        const JSQ1_1                            = 0x00000200,           //ADC JSQ1 bit 1
        const JSQ1_2                            = 0x00000400,           //ADC JSQ1 bit 2
        const JSQ1_3                            = 0x00000800,           //ADC JSQ1 bit 3
        const JSQ1_4                            = 0x00001000,           //ADC JSQ1 bit 4
        const JSQ2                              = 0x0007C000,           //ADC 2nd conversion in injected sequence
        const JSQ2_0                            = 0x00004000,           //ADC JSQ2 bit 0
        const JSQ2_1                            = 0x00008000,           //ADC JSQ2 bit 1
        const JSQ2_2                            = 0x00010000,           //ADC JSQ2 bit 2
        const JSQ2_3                            = 0x00020000,           //ADC JSQ2 bit 3
        const JSQ2_4                            = 0x00040000,           //ADC JSQ2 bit 4
        const JSQ3                              = 0x01F00000,           //ADC 3rd conversion in injected sequence
        const JSQ3_0                            = 0x00100000,           //ADC JSQ3 bit 0
        const JSQ3_1                            = 0x00200000,           //ADC JSQ3 bit 1
        const JSQ3_2                            = 0x00400000,           //ADC JSQ3 bit 2
        const JSQ3_3                            = 0x00800000,           //ADC JSQ3 bit 3
        const JSQ3_4                            = 0x01000000,           //ADC JSQ3 bit 4
        const JSQ4                              = 0x7C000000,           //ADC 4th conversion in injected sequence
        const JSQ4_0                            = 0x04000000,           //ADC JSQ4 bit 0
        const JSQ4_1                            = 0x08000000,           //ADC JSQ4 bit 1
        const JSQ4_2                            = 0x10000000,           //ADC JSQ4 bit 2
        const JSQ4_3                            = 0x20000000,           //ADC JSQ4 bit 3
        const JSQ4_4                            = 0x40000000,           //ADC JSQ4 bit 4
    }

    OFR1: u32 {
        const OFFSET1                           = 0x00000FFF,           //ADC data offset 1 for channel programmed into bits OFFSET1_CH[4:0]
        const OFFSET1_0                         = 0x00000001,           //ADC OFFSET1 bit 0
        const OFFSET1_1                         = 0x00000002,           //ADC OFFSET1 bit 1
        const OFFSET1_2                         = 0x00000004,           //ADC OFFSET1 bit 2
        const OFFSET1_3                         = 0x00000008,           //ADC OFFSET1 bit 3
        const OFFSET1_4                         = 0x00000010,           //ADC OFFSET1 bit 4
        const OFFSET1_5                         = 0x00000020,           //ADC OFFSET1 bit 5
        const OFFSET1_6                         = 0x00000040,           //ADC OFFSET1 bit 6
        const OFFSET1_7                         = 0x00000080,           //ADC OFFSET1 bit 7
        const OFFSET1_8                         = 0x00000100,           //ADC OFFSET1 bit 8
        const OFFSET1_9                         = 0x00000200,           //ADC OFFSET1 bit 9
        const OFFSET1_10                        = 0x00000400,           //ADC OFFSET1 bit 10
        const OFFSET1_11                        = 0x00000800,           //ADC OFFSET1 bit 11
        const OFFSET1_CH                        = 0x7C000000,           //ADC Channel selection for the data offset 1
        const OFFSET1_CH_0                      = 0x04000000,           //ADC OFFSET1_CH bit 0
        const OFFSET1_CH_1                      = 0x08000000,           //ADC OFFSET1_CH bit 1
        const OFFSET1_CH_2                      = 0x10000000,           //ADC OFFSET1_CH bit 2
        const OFFSET1_CH_3                      = 0x20000000,           //ADC OFFSET1_CH bit 3
        const OFFSET1_CH_4                      = 0x40000000,           //ADC OFFSET1_CH bit 4
        const OFFSET1_EN                        = 0x80000000,           //ADC offset 1 enable
    }

    OFR2: u32 {
        const OFFSET2                           = 0x00000FFF,           //ADC data offset 2 for channel programmed into bits OFFSET2_CH[4:0]
        const OFFSET2_0                         = 0x00000001,           //ADC OFFSET2 bit 0
        const OFFSET2_1                         = 0x00000002,           //ADC OFFSET2 bit 1
        const OFFSET2_2                         = 0x00000004,           //ADC OFFSET2 bit 2
        const OFFSET2_3                         = 0x00000008,           //ADC OFFSET2 bit 3
        const OFFSET2_4                         = 0x00000010,           //ADC OFFSET2 bit 4
        const OFFSET2_5                         = 0x00000020,           //ADC OFFSET2 bit 5
        const OFFSET2_6                         = 0x00000040,           //ADC OFFSET2 bit 6
        const OFFSET2_7                         = 0x00000080,           //ADC OFFSET2 bit 7
        const OFFSET2_8                         = 0x00000100,           //ADC OFFSET2 bit 8
        const OFFSET2_9                         = 0x00000200,           //ADC OFFSET2 bit 9
        const OFFSET2_10                        = 0x00000400,           //ADC OFFSET2 bit 10
        const OFFSET2_11                        = 0x00000800,           //ADC OFFSET2 bit 11
        const OFFSET2_CH                        = 0x7C000000,           //ADC Channel selection for the data offset 2
        const OFFSET2_CH_0                      = 0x04000000,           //ADC OFFSET2_CH bit 0
        const OFFSET2_CH_1                      = 0x08000000,           //ADC OFFSET2_CH bit 1
        const OFFSET2_CH_2                      = 0x10000000,           //ADC OFFSET2_CH bit 2
        const OFFSET2_CH_3                      = 0x20000000,           //ADC OFFSET2_CH bit 3
        const OFFSET2_CH_4                      = 0x40000000,           //ADC OFFSET2_CH bit 4
        const OFFSET2_EN                        = 0x80000000,           //ADC offset 2 enable
    }

    OFR3: u32 {
        const OFFSET3                           = 0x00000FFF,           //ADC data offset 3 for channel programmed into bits OFFSET3_CH[4:0]
        const OFFSET3_0                         = 0x00000001,           //ADC OFFSET3 bit 0
        const OFFSET3_1                         = 0x00000002,           //ADC OFFSET3 bit 1
        const OFFSET3_2                         = 0x00000004,           //ADC OFFSET3 bit 2
        const OFFSET3_3                         = 0x00000008,           //ADC OFFSET3 bit 3
        const OFFSET3_4                         = 0x00000010,           //ADC OFFSET3 bit 4
        const OFFSET3_5                         = 0x00000020,           //ADC OFFSET3 bit 5
        const OFFSET3_6                         = 0x00000040,           //ADC OFFSET3 bit 6
        const OFFSET3_7                         = 0x00000080,           //ADC OFFSET3 bit 7
        const OFFSET3_8                         = 0x00000100,           //ADC OFFSET3 bit 8
        const OFFSET3_9                         = 0x00000200,           //ADC OFFSET3 bit 9
        const OFFSET3_10                        = 0x00000400,           //ADC OFFSET3 bit 10
        const OFFSET3_11                        = 0x00000800,           //ADC OFFSET3 bit 11
        const OFFSET3_CH                        = 0x7C000000,           //ADC Channel selection for the data offset 3
        const OFFSET3_CH_0                      = 0x04000000,           //ADC OFFSET3_CH bit 0
        const OFFSET3_CH_1                      = 0x08000000,           //ADC OFFSET3_CH bit 1
        const OFFSET3_CH_2                      = 0x10000000,           //ADC OFFSET3_CH bit 2
        const OFFSET3_CH_3                      = 0x20000000,           //ADC OFFSET3_CH bit 3
        const OFFSET3_CH_4                      = 0x40000000,           //ADC OFFSET3_CH bit 4
        const OFFSET3_EN                        = 0x80000000,           //ADC offset 3 enable
    }

    OFR4: u32 {
        const OFFSET4                           = 0x00000FFF,           //ADC data offset 4 for channel programmed into bits OFFSET4_CH[4:0]
        const OFFSET4_0                         = 0x00000001,           //ADC OFFSET4 bit 0
        const OFFSET4_1                         = 0x00000002,           //ADC OFFSET4 bit 1
        const OFFSET4_2                         = 0x00000004,           //ADC OFFSET4 bit 2
        const OFFSET4_3                         = 0x00000008,           //ADC OFFSET4 bit 3
        const OFFSET4_4                         = 0x00000010,           //ADC OFFSET4 bit 4
        const OFFSET4_5                         = 0x00000020,           //ADC OFFSET4 bit 5
        const OFFSET4_6                         = 0x00000040,           //ADC OFFSET4 bit 6
        const OFFSET4_7                         = 0x00000080,           //ADC OFFSET4 bit 7
        const OFFSET4_8                         = 0x00000100,           //ADC OFFSET4 bit 8
        const OFFSET4_9                         = 0x00000200,           //ADC OFFSET4 bit 9
        const OFFSET4_10                        = 0x00000400,           //ADC OFFSET4 bit 10
        const OFFSET4_11                        = 0x00000800,           //ADC OFFSET4 bit 11
        const OFFSET4_CH                        = 0x7C000000,           //ADC Channel selection for the data offset 4
        const OFFSET4_CH_0                      = 0x04000000,           //ADC OFFSET4_CH bit 0
        const OFFSET4_CH_1                      = 0x08000000,           //ADC OFFSET4_CH bit 1
        const OFFSET4_CH_2                      = 0x10000000,           //ADC OFFSET4_CH bit 2
        const OFFSET4_CH_3                      = 0x20000000,           //ADC OFFSET4_CH bit 3
        const OFFSET4_CH_4                      = 0x40000000,           //ADC OFFSET4_CH bit 4
        const OFFSET4_EN                        = 0x80000000,           //ADC offset 4 enable
    }

    JDR1: u32 {
        const JDATA                             = 0x0000FFFF,           //ADC Injected DATA
        const JDATA_0                           = 0x00000001,           //ADC JDATA bit 0
        const JDATA_1                           = 0x00000002,           //ADC JDATA bit 1
        const JDATA_2                           = 0x00000004,           //ADC JDATA bit 2
        const JDATA_3                           = 0x00000008,           //ADC JDATA bit 3
        const JDATA_4                           = 0x00000010,           //ADC JDATA bit 4
        const JDATA_5                           = 0x00000020,           //ADC JDATA bit 5
        const JDATA_6                           = 0x00000040,           //ADC JDATA bit 6
        const JDATA_7                           = 0x00000080,           //ADC JDATA bit 7
        const JDATA_8                           = 0x00000100,           //ADC JDATA bit 8
        const JDATA_9                           = 0x00000200,           //ADC JDATA bit 9
        const JDATA_10                          = 0x00000400,           //ADC JDATA bit 10
        const JDATA_11                          = 0x00000800,           //ADC JDATA bit 11
        const JDATA_12                          = 0x00001000,           //ADC JDATA bit 12
        const JDATA_13                          = 0x00002000,           //ADC JDATA bit 13
        const JDATA_14                          = 0x00004000,           //ADC JDATA bit 14
        const JDATA_15                          = 0x00008000,           //ADC JDATA bit 15
    }

    JDR2: u32 {
        const JDATA                             = 0x0000FFFF,           //ADC Injected DATA
        const JDATA_0                           = 0x00000001,           //ADC JDATA bit 0
        const JDATA_1                           = 0x00000002,           //ADC JDATA bit 1
        const JDATA_2                           = 0x00000004,           //ADC JDATA bit 2
        const JDATA_3                           = 0x00000008,           //ADC JDATA bit 3
        const JDATA_4                           = 0x00000010,           //ADC JDATA bit 4
        const JDATA_5                           = 0x00000020,           //ADC JDATA bit 5
        const JDATA_6                           = 0x00000040,           //ADC JDATA bit 6
        const JDATA_7                           = 0x00000080,           //ADC JDATA bit 7
        const JDATA_8                           = 0x00000100,           //ADC JDATA bit 8
        const JDATA_9                           = 0x00000200,           //ADC JDATA bit 9
        const JDATA_10                          = 0x00000400,           //ADC JDATA bit 10
        const JDATA_11                          = 0x00000800,           //ADC JDATA bit 11
        const JDATA_12                          = 0x00001000,           //ADC JDATA bit 12
        const JDATA_13                          = 0x00002000,           //ADC JDATA bit 13
        const JDATA_14                          = 0x00004000,           //ADC JDATA bit 14
        const JDATA_15                          = 0x00008000,           //ADC JDATA bit 15
    }

    JDR3: u32 {
        const JDATA                             = 0x0000FFFF,           //ADC Injected DATA
        const JDATA_0                           = 0x00000001,           //ADC JDATA bit 0
        const JDATA_1                           = 0x00000002,           //ADC JDATA bit 1
        const JDATA_2                           = 0x00000004,           //ADC JDATA bit 2
        const JDATA_3                           = 0x00000008,           //ADC JDATA bit 3
        const JDATA_4                           = 0x00000010,           //ADC JDATA bit 4
        const JDATA_5                           = 0x00000020,           //ADC JDATA bit 5
        const JDATA_6                           = 0x00000040,           //ADC JDATA bit 6
        const JDATA_7                           = 0x00000080,           //ADC JDATA bit 7
        const JDATA_8                           = 0x00000100,           //ADC JDATA bit 8
        const JDATA_9                           = 0x00000200,           //ADC JDATA bit 9
        const JDATA_10                          = 0x00000400,           //ADC JDATA bit 10
        const JDATA_11                          = 0x00000800,           //ADC JDATA bit 11
        const JDATA_12                          = 0x00001000,           //ADC JDATA bit 12
        const JDATA_13                          = 0x00002000,           //ADC JDATA bit 13
        const JDATA_14                          = 0x00004000,           //ADC JDATA bit 14
        const JDATA_15                          = 0x00008000,           //ADC JDATA bit 15
    }

    JDR4: u32 {
        const JDATA                             = 0x0000FFFF,           //ADC Injected DATA
        const JDATA_0                           = 0x00000001,           //ADC JDATA bit 0
        const JDATA_1                           = 0x00000002,           //ADC JDATA bit 1
        const JDATA_2                           = 0x00000004,           //ADC JDATA bit 2
        const JDATA_3                           = 0x00000008,           //ADC JDATA bit 3
        const JDATA_4                           = 0x00000010,           //ADC JDATA bit 4
        const JDATA_5                           = 0x00000020,           //ADC JDATA bit 5
        const JDATA_6                           = 0x00000040,           //ADC JDATA bit 6
        const JDATA_7                           = 0x00000080,           //ADC JDATA bit 7
        const JDATA_8                           = 0x00000100,           //ADC JDATA bit 8
        const JDATA_9                           = 0x00000200,           //ADC JDATA bit 9
        const JDATA_10                          = 0x00000400,           //ADC JDATA bit 10
        const JDATA_11                          = 0x00000800,           //ADC JDATA bit 11
        const JDATA_12                          = 0x00001000,           //ADC JDATA bit 12
        const JDATA_13                          = 0x00002000,           //ADC JDATA bit 13
        const JDATA_14                          = 0x00004000,           //ADC JDATA bit 14
        const JDATA_15                          = 0x00008000,           //ADC JDATA bit 15
    }

    AWD2CR: u32 {
        const AWD2CH                            = 0x0007FFFE,           //ADC Analog watchdog 2 channel selection
        const AWD2CH_0                          = 0x00000002,           //ADC AWD2CH bit 0
        const AWD2CH_1                          = 0x00000004,           //ADC AWD2CH bit 1
        const AWD2CH_2                          = 0x00000008,           //ADC AWD2CH bit 2
        const AWD2CH_3                          = 0x00000010,           //ADC AWD2CH bit 3
        const AWD2CH_4                          = 0x00000020,           //ADC AWD2CH bit 4
        const AWD2CH_5                          = 0x00000040,           //ADC AWD2CH bit 5
        const AWD2CH_6                          = 0x00000080,           //ADC AWD2CH bit 6
        const AWD2CH_7                          = 0x00000100,           //ADC AWD2CH bit 7
        const AWD2CH_8                          = 0x00000200,           //ADC AWD2CH bit 8
        const AWD2CH_9                          = 0x00000400,           //ADC AWD2CH bit 9
        const AWD2CH_10                         = 0x00000800,           //ADC AWD2CH bit 10
        const AWD2CH_11                         = 0x00001000,           //ADC AWD2CH bit 11
        const AWD2CH_12                         = 0x00002000,           //ADC AWD2CH bit 12
        const AWD2CH_13                         = 0x00004000,           //ADC AWD2CH bit 13
        const AWD2CH_14                         = 0x00008000,           //ADC AWD2CH bit 14
        const AWD2CH_15                         = 0x00010000,           //ADC AWD2CH bit 15
        const AWD2CH_16                         = 0x00020000,           //ADC AWD2CH bit 16
        const AWD2CH_17                         = 0x00030000,           //ADC AWD2CH bit 17
    }

    AWD3CR: u32 {
        const AWD3CH                            = 0x0007FFFE,           //ADC Analog watchdog 2 channel selection
        const AWD3CH_0                          = 0x00000002,           //ADC AWD3CH bit 0
        const AWD3CH_1                          = 0x00000004,           //ADC AWD3CH bit 1
        const AWD3CH_2                          = 0x00000008,           //ADC AWD3CH bit 2
        const AWD3CH_3                          = 0x00000010,           //ADC AWD3CH bit 3
        const AWD3CH_4                          = 0x00000020,           //ADC AWD3CH bit 4
        const AWD3CH_5                          = 0x00000040,           //ADC AWD3CH bit 5
        const AWD3CH_6                          = 0x00000080,           //ADC AWD3CH bit 6
        const AWD3CH_7                          = 0x00000100,           //ADC AWD3CH bit 7
        const AWD3CH_8                          = 0x00000200,           //ADC AWD3CH bit 8
        const AWD3CH_9                          = 0x00000400,           //ADC AWD3CH bit 9
        const AWD3CH_10                         = 0x00000800,           //ADC AWD3CH bit 10
        const AWD3CH_11                         = 0x00001000,           //ADC AWD3CH bit 11
        const AWD3CH_12                         = 0x00002000,           //ADC AWD3CH bit 12
        const AWD3CH_13                         = 0x00004000,           //ADC AWD3CH bit 13
        const AWD3CH_14                         = 0x00008000,           //ADC AWD3CH bit 14
        const AWD3CH_15                         = 0x00010000,           //ADC AWD3CH bit 15
        const AWD3CH_16                         = 0x00020000,           //ADC AWD3CH bit 16
        const AWD3CH_17                         = 0x00030000,           //ADC AWD3CH bit 17
    }

    DIFSEL: u32 {
        const DIFSEL                            = 0x0007FFFE,           //ADC differential modes for channels 1 to 18
        const DIFSEL_0                          = 0x00000002,           //ADC DIFSEL bit 0
        const DIFSEL_1                          = 0x00000004,           //ADC DIFSEL bit 1
        const DIFSEL_2                          = 0x00000008,           //ADC DIFSEL bit 2
        const DIFSEL_3                          = 0x00000010,           //ADC DIFSEL bit 3
        const DIFSEL_4                          = 0x00000020,           //ADC DIFSEL bit 4
        const DIFSEL_5                          = 0x00000040,           //ADC DIFSEL bit 5
        const DIFSEL_6                          = 0x00000080,           //ADC DIFSEL bit 6
        const DIFSEL_7                          = 0x00000100,           //ADC DIFSEL bit 7
        const DIFSEL_8                          = 0x00000200,           //ADC DIFSEL bit 8
        const DIFSEL_9                          = 0x00000400,           //ADC DIFSEL bit 9
        const DIFSEL_10                         = 0x00000800,           //ADC DIFSEL bit 10
        const DIFSEL_11                         = 0x00001000,           //ADC DIFSEL bit 11
        const DIFSEL_12                         = 0x00002000,           //ADC DIFSEL bit 12
        const DIFSEL_13                         = 0x00004000,           //ADC DIFSEL bit 13
        const DIFSEL_14                         = 0x00008000,           //ADC DIFSEL bit 14
        const DIFSEL_15                         = 0x00010000,           //ADC DIFSEL bit 15
        const DIFSEL_16                         = 0x00020000,           //ADC DIFSEL bit 16
        const DIFSEL_17                         = 0x00030000,           //ADC DIFSEL bit 17
    }

    CALFACT: u32 {
        const CALFACT_S                         = 0x0000007F,           //ADC calibration factors in single-ended mode
        const CALFACT_S_0                       = 0x00000001,           //ADC CALFACT_S bit 0
        const CALFACT_S_1                       = 0x00000002,           //ADC CALFACT_S bit 1
        const CALFACT_S_2                       = 0x00000004,           //ADC CALFACT_S bit 2
        const CALFACT_S_3                       = 0x00000008,           //ADC CALFACT_S bit 3
        const CALFACT_S_4                       = 0x00000010,           //ADC CALFACT_S bit 4
        const CALFACT_S_5                       = 0x00000020,           //ADC CALFACT_S bit 5
        const CALFACT_S_6                       = 0x00000040,           //ADC CALFACT_S bit 6
        const CALFACT_D                         = 0x007F0000,           //ADC calibration factors in differential mode
        const CALFACT_D_0                       = 0x00010000,           //ADC CALFACT_D bit 0
        const CALFACT_D_1                       = 0x00020000,           //ADC CALFACT_D bit 1
        const CALFACT_D_2                       = 0x00040000,           //ADC CALFACT_D bit 2
        const CALFACT_D_3                       = 0x00080000,           //ADC CALFACT_D bit 3
        const CALFACT_D_4                       = 0x00100000,           //ADC CALFACT_D bit 4
        const CALFACT_D_5                       = 0x00200000,           //ADC CALFACT_D bit 5
        const CALFACT_D_6                       = 0x00400000,           //ADC CALFACT_D bit 6
    }
}

//custom

