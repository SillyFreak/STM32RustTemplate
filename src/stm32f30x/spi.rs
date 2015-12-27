use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct SPI {
    pub CR1:                u16,
    pub RESERVED0:          u16,
    pub CR2:                u16,
    pub RESERVED1:          u16,
    pub SR:                 u16,
    pub RESERVED2:          u16,
    pub DR:                 u16,
    pub RESERVED3:          u16,
    pub CRCPR:              u16,
    pub RESERVED4:          u16,
    pub RXCRCR:             u16,
    pub RESERVED5:          u16,
    pub TXCRCR:             u16,
    pub RESERVED6:          u16,
    pub I2SCFGR:            u16,
    pub RESERVED7:          u16,
    pub I2SPR:              u16,
    pub RESERVED8:          u16,
}

//register addresses

registers! {
    const I2S2EXT:      SPI             = I2S2EXT_BASE,
    const SPI2:         SPI             = SPI2_BASE,
    const SPI3:         SPI             = SPI3_BASE,
    const I2S3EXT:      SPI             = I2S3EXT_BASE,
    const SPI1:         SPI             = SPI1_BASE,
    const SPI4:         SPI             = SPI4_BASE,
}

//custom

