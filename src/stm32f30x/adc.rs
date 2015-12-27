use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct ADC {
    pub ISR:                u32,
    pub IER:                u32,
    pub CR:                 u32,
    pub CFGR:               u32,
    pub RESERVED0:          u32,
    pub SMPR1:              u32,
    pub SMPR2:              u32,
    pub RESERVED1:          u32,
    pub TR1:                u32,
    pub TR2:                u32,
    pub TR3:                u32,
    pub RESERVED2:          u32,
    pub SQR1:               u32,
    pub SQR2:               u32,
    pub SQR3:               u32,
    pub SQR4:               u32,
    pub DR:                 u32,
    pub RESERVED3:          u32,
    pub RESERVED4:          u32,
    pub JSQR:               u32,
    pub RESERVED5:          [u32; 4],
    pub OFR1:               u32,
    pub OFR2:               u32,
    pub OFR3:               u32,
    pub OFR4:               u32,
    pub RESERVED6:          [u32; 4],
    pub JDR1:               u32,
    pub JDR2:               u32,
    pub JDR3:               u32,
    pub JDR4:               u32,
    pub RESERVED7:          [u32; 4],
    pub AWD2CR:             u32,
    pub AWD3CR:             u32,
    pub RESERVED8:          u32,
    pub RESERVED9:          u32,
    pub DIFSEL:             u32,
    pub CALFACT:            u32,
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct ADC_Common {
    pub CSR:                u32,
    pub RESERVED:           u32,
    pub CCR:                u32,
    pub CDR:                u32,
}

//register addresses

registers! {
    const ADC1:         ADC             = ADC1_BASE,
    const ADC2:         ADC             = ADC2_BASE,
    const ADC3:         ADC             = ADC3_BASE,
    const ADC4:         ADC             = ADC4_BASE,
}

registers! {
    const ADC1_2:       ADC_Common      = ADC1_2_BASE,
    const ADC3_4:       ADC_Common      = ADC3_4_BASE,
}

//custom

