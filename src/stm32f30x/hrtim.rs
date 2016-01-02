use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct HRTIM_Master {
    pub MCR:                u32,                                        //HRTIM Master Timer control register,                     Address offset: 0x00
    pub MISR:               u32,                                        //HRTIM Master Timer interrupt status register,            Address offset: 0x04
    pub MICR:               u32,                                        //HRTIM Master Timer interrupt clear register,              Address offset: 0x08
    pub MDIER:              u32,                                        //HRTIM Master Timer DMA/interrupt enable register         Address offset: 0x0C
    pub MCNTR:              u32,                                        //HRTIM Master Timer counter register,                     Address offset: 0x10
    pub MPER:               u32,                                        //HRTIM Master Timer period register,                      Address offset: 0x14
    pub MREP:               u32,                                        //HRTIM Master Timer repetition register,                  Address offset: 0x18
    pub MCMP1R:             u32,                                        //HRTIM Master Timer compare 1 register,                   Address offset: 0x1C
    pub RESERVED0:          u32,                                        //Reserved,                                                                0x20
    pub MCMP2R:             u32,                                        //HRTIM Master Timer compare 2 register,                   Address offset: 0x24
    pub MCMP3R:             u32,                                        //HRTIM Master Timer compare 3 register,                   Address offset: 0x28
    pub MCMP4R:             u32,                                        //HRTIM Master Timer compare 4 register,                   Address offset: 0x2C
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct HRTIM_Timerx {
    pub TIMxCR:             u32,                                        //HRTIM Timerx control register,                              Address offset: 0x00
    pub TIMxISR:            u32,                                        //HRTIM Timerx interrupt status register,                     Address offset: 0x04
    pub TIMxICR:            u32,                                        //HRTIM Timerx interrupt clear register,                      Address offset: 0x08
    pub TIMxDIER:           u32,                                        //HRTIM Timerx DMA/interrupt enable register,                 Address offset: 0x0C
    pub CNTxR:              u32,                                        //HRTIM Timerx counter register,                              Address offset: 0x10
    pub PERxR:              u32,                                        //HRTIM Timerx period register,                               Address offset: 0x14
    pub REPxR:              u32,                                        //HRTIM Timerx repetition register,                           Address offset: 0x18
    pub CMP1xR:             u32,                                        //HRTIM Timerx compare 1 register,                            Address offset: 0x1C
    pub CMP1CxR:            u32,                                        //HRTIM Timerx compare 1 compound register,                   Address offset: 0x20
    pub CMP2xR:             u32,                                        //HRTIM Timerx compare 2 register,                            Address offset: 0x24
    pub CMP3xR:             u32,                                        //HRTIM Timerx compare 3 register,                            Address offset: 0x28
    pub CMP4xR:             u32,                                        //HRTIM Timerx compare 4 register,                            Address offset: 0x2C
    pub CPT1xR:             u32,                                        //HRTIM Timerx capture 1 register,                            Address offset: 0x30
    pub CPT2xR:             u32,                                        //HRTIM Timerx capture 2 register,                            Address offset: 0x34
    pub DTxR:               u32,                                        //HRTIM Timerx dead time register,                            Address offset: 0x38
    pub SETx1R:             u32,                                        //HRTIM Timerx output 1 set register,                         Address offset: 0x3C
    pub RSTx1R:             u32,                                        //HRTIM Timerx output 1 reset register,                       Address offset: 0x40
    pub SETx2R:             u32,                                        //HRTIM Timerx output 2 set register,                         Address offset: 0x44
    pub RSTx2R:             u32,                                        //HRTIM Timerx output 2 reset register,                       Address offset: 0x48
    pub EEFxR1:             u32,                                        //HRTIM Timerx external event filtering 1 register,           Address offset: 0x4C
    pub EEFxR2:             u32,                                        //HRTIM Timerx external event filtering 2 register,           Address offset: 0x50
    pub RSTxR:              u32,                                        //HRTIM Timerx Reset register,                                Address offset: 0x54
    pub CHPxR:              u32,                                        //HRTIM Timerx Chopper register,                              Address offset: 0x58
    pub CPT1xCR:            u32,                                        //HRTIM Timerx Capture 1 register,                            Address offset: 0x5C
    pub CPT2xCR:            u32,                                        //HRTIM Timerx Capture 2 register,                            Address offset: 0x60
    pub OUTxR:              u32,                                        //HRTIM Timerx Output register,                               Address offset: 0x64
    pub FLTxR:              u32,                                        //HRTIM Timerx Fault register,                                Address offset: 0x68
    pub RESERVED0:          [u32; 5],                                   //Reserved,
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct HRTIM_Common {
    pub CR1:                u32,                                        //HRTIM control register1,                                    Address offset: 0x00
    pub CR2:                u32,                                        //HRTIM control register2,                                    Address offset: 0x04
    pub ISR:                u32,                                        //HRTIM interrupt status register,                            Address offset: 0x08
    pub ICR:                u32,                                        //HRTIM interrupt clear register,                             Address offset: 0x0C
    pub IER:                u32,                                        //HRTIM interrupt enable register,                            Address offset: 0x10
    pub OENR:               u32,                                        //HRTIM Output enable register,                               Address offset: 0x14
    pub DISR:               u32,                                        //HRTIM Output disable register,                              Address offset: 0x18
    pub ODSR:               u32,                                        //HRTIM Output disable status register,                       Address offset: 0x1C
    pub BMCR:               u32,                                        //HRTIM Burst mode control register,                          Address offset: 0x20
    pub BMTRGR:             u32,                                        //HRTIM Busrt mode trigger register,                          Address offset: 0x24
    pub BMCMPR:             u32,                                        //HRTIM Burst mode compare register,                          Address offset: 0x28
    pub BMPER:              u32,                                        //HRTIM Burst mode period register,                           Address offset: 0x2C
    pub EECR1:              u32,                                        //HRTIM Timer external event control register1,               Address offset: 0x30
    pub EECR2:              u32,                                        //HRTIM Timer external event control register2,               Address offset: 0x34
    pub EECR3:              u32,                                        //HRTIM Timer external event control register3,               Address offset: 0x38
    pub ADC1R:              u32,                                        //HRTIM ADC Trigger 1 register,                               Address offset: 0x3C
    pub ADC2R:              u32,                                        //HRTIM ADC Trigger 2 register,                               Address offset: 0x40
    pub ADC3R:              u32,                                        //HRTIM ADC Trigger 3 register,                               Address offset: 0x44
    pub ADC4R:              u32,                                        //HRTIM ADC Trigger 4 register,                               Address offset: 0x48
    pub DLLCR:              u32,                                        //HRTIM DLL control register,                                 Address offset: 0x4C
    pub FLTINxR1:           u32,                                        //HRTIM Fault input register1,                                Address offset: 0x50
    pub FLTINxR2:           u32,                                        //HRTIM Fault input register2,                                Address offset: 0x54
    pub BDMUPDR:            u32,                                        //HRTIM Burst DMA Master Timer update register,               Address offset: 0x58
    pub BDTAUPR:            u32,                                        //HRTIM Burst DMA Timerx update register,                     Address offset: 0x5C
    pub BDTBUPR:            u32,                                        //HRTIM Burst DMA Timerx update register,                     Address offset: 0x60
    pub BDTCUPR:            u32,                                        //HRTIM Burst DMA Timerx update register,                     Address offset: 0x64
    pub BDTDUPR:            u32,                                        //HRTIM Burst DMA Timerx update register,                     Address offset: 0x68
    pub BDTEUPR:            u32,                                        //HRTIM Burst DMA Timerx update register,                     Address offset: 0x6C
    pub BDMADR:             u32,                                        //HRTIM Burst DMA Master Data register,                       Address offset: 0x70
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
    const HRTIM1:           HRTIM               = HRTIM1_BASE,
}

//bit definitions

//custom

