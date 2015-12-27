use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct HRTIM_Master {
    pub MCR:                u32,
    pub MISR:               u32,
    pub MICR:               u32,
    pub MDIER:              u32,
    pub MCNTR:              u32,
    pub MPER:               u32,
    pub MREP:               u32,
    pub MCMP1R:             u32,
    pub RESERVED0:          u32,
    pub MCMP2R:             u32,
    pub MCMP3R:             u32,
    pub MCMP4R:             u32,
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct HRTIM_Timerx {
    pub TIMxCR:             u32,
    pub TIMxISR:            u32,
    pub TIMxICR:            u32,
    pub TIMxDIER:           u32,
    pub CNTxR:              u32,
    pub PERxR:              u32,
    pub REPxR:              u32,
    pub CMP1xR:             u32,
    pub CMP1CxR:            u32,
    pub CMP2xR:             u32,
    pub CMP3xR:             u32,
    pub CMP4xR:             u32,
    pub CPT1xR:             u32,
    pub CPT2xR:             u32,
    pub DTxR:               u32,
    pub SETx1R:             u32,
    pub RSTx1R:             u32,
    pub SETx2R:             u32,
    pub RSTx2R:             u32,
    pub EEFxR1:             u32,
    pub EEFxR2:             u32,
    pub RSTxR:              u32,
    pub CHPxR:              u32,
    pub CPT1xCR:            u32,
    pub CPT2xCR:            u32,
    pub OUTxR:              u32,
    pub FLTxR:              u32,
    pub RESERVED0:          [u32; 5],
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct HRTIM_Common {
    pub CR1:                u32,
    pub CR2:                u32,
    pub ISR:                u32,
    pub ICR:                u32,
    pub IER:                u32,
    pub OENR:               u32,
    pub DISR:               u32,
    pub ODSR:               u32,
    pub BMCR:               u32,
    pub BMTRGR:             u32,
    pub BMCMPR:             u32,
    pub BMPER:              u32,
    pub EECR1:              u32,
    pub EECR2:              u32,
    pub EECR3:              u32,
    pub ADC1R:              u32,
    pub ADC2R:              u32,
    pub ADC3R:              u32,
    pub ADC4R:              u32,
    pub DLLCR:              u32,
    pub FLTINxR1:           u32,
    pub FLTINxR2:           u32,
    pub BDMUPDR:            u32,
    pub BDTAUPR:            u32,
    pub BDTBUPR:            u32,
    pub BDTCUPR:            u32,
    pub BDTDUPR:            u32,
    pub BDTEUPR:            u32,
    pub BDMADR:             u32,
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct HRTIM {
    pub HRTIM_MASTER:       HRTIM_Master,
    pub RESERVED0:          [u32; 20],
    pub HRTIM_TIMERx:       [HRTIM_Timerx; 5],
    pub RESERVED1:          [u32; 32],
    pub HRTIM_COMMON:       HRTIM_Common,
}

//register addresses

registers! {
    const HRTIM1_COMMON: HRTIM_Common    = HRTIM1_COMMON_BASE,
}

registers! {
    const HRTIM1:       HRTIM           = HRTIM1_BASE,
}

//custom

