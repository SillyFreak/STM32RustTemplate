use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct I2C {
    pub CR1:                u32,
    pub CR2:                u32,
    pub OAR1:               u32,
    pub OAR2:               u32,
    pub TIMINGR:            u32,
    pub TIMEOUTR:           u32,
    pub ISR:                u32,
    pub ICR:                u32,
    pub PECR:               u32,
    pub RXDR:               u32,
    pub TXDR:               u32,
}

//register addresses

registers! {
    const I2C1:         I2C             = I2C1_BASE,
    const I2C2:         I2C             = I2C2_BASE,
    const I2C3:         I2C             = I2C3_BASE,
}

//custom

