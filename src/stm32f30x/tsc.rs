use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct TSC {
    pub CR:                 u32,                                        //TSC control register,                                     Address offset: 0x00
    pub IER:                u32,                                        //TSC interrupt enable register,                            Address offset: 0x04
    pub ICR:                u32,                                        //TSC interrupt clear register,                             Address offset: 0x08
    pub ISR:                u32,                                        //TSC interrupt status register,                            Address offset: 0x0C
    pub IOHCR:              u32,                                        //TSC I/O hysteresis control register,                      Address offset: 0x10
    pub RESERVED1:          u32,                                        //Reserved,                                                 Address offset: 0x14
    pub IOASCR:             u32,                                        //TSC I/O analog switch control register,                   Address offset: 0x18
    pub RESERVED2:          u32,                                        //Reserved,                                                 Address offset: 0x1C
    pub IOSCR:              u32,                                        //TSC I/O sampling control register,                        Address offset: 0x20
    pub RESERVED3:          u32,                                        //Reserved,                                                 Address offset: 0x24
    pub IOCCR:              u32,                                        //TSC I/O channel control register,                         Address offset: 0x28
    pub RESERVED4:          u32,                                        //Reserved,                                                 Address offset: 0x2C
    pub IOGCSR:             u32,                                        //TSC I/O group control status register,                    Address offset: 0x30
    pub IOGXCR:             [u32; 8],                                   //TSC I/O group x counter register,                         Address offset: 0x34-50
}

//register addresses

registers! {
    const TSC:              TSC                 = TSC_BASE,
}

//bit definitions

//custom

