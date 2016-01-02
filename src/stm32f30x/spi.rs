use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct SPI {
    pub CR1:                u16,                                        //SPI Control register 1 (not used in I2S mode),       Address offset: 0x00
    pub RESERVED0:          u16,                                        //Reserved, 0x02
    pub CR2:                u16,                                        //SPI Control register 2,                              Address offset: 0x04
    pub RESERVED1:          u16,                                        //Reserved, 0x06
    pub SR:                 u16,                                        //SPI Status register,                                 Address offset: 0x08
    pub RESERVED2:          u16,                                        //Reserved, 0x0A
    pub DR:                 u16,                                        //SPI data register,                                   Address offset: 0x0C
    pub RESERVED3:          u16,                                        //Reserved, 0x0E
    pub CRCPR:              u16,                                        //SPI CRC polynomial register (not used in I2S mode),  Address offset: 0x10
    pub RESERVED4:          u16,                                        //Reserved, 0x12
    pub RXCRCR:             u16,                                        //SPI Rx CRC register (not used in I2S mode),          Address offset: 0x14
    pub RESERVED5:          u16,                                        //Reserved, 0x16
    pub TXCRCR:             u16,                                        //SPI Tx CRC register (not used in I2S mode),          Address offset: 0x18
    pub RESERVED6:          u16,                                        //Reserved, 0x1A
    pub I2SCFGR:            u16,                                        //SPI_I2S configuration register,                      Address offset: 0x1C
    pub RESERVED7:          u16,                                        //Reserved, 0x1E
    pub I2SPR:              u16,                                        //SPI_I2S prescaler register,                          Address offset: 0x20
    pub RESERVED8:          u16,                                        //Reserved, 0x22
}

//register addresses

registers! {
    const I2S2EXT:          SPI                 = I2S2EXT_BASE,
    const SPI2:             SPI                 = SPI2_BASE,
    const SPI3:             SPI                 = SPI3_BASE,
    const I2S3EXT:          SPI                 = I2S3EXT_BASE,
    const SPI1:             SPI                 = SPI1_BASE,
    const SPI4:             SPI                 = SPI4_BASE,
}

//bit definitions

constants! {
    CR1: u16 {
        const CPHA                              = 0x0001,               //Clock Phase
        const CPOL                              = 0x0002,               //Clock Polarity
        const MSTR                              = 0x0004,               //Master Selection
        const BR                                = 0x0038,               //BR[2:0] bits (Baud Rate Control)
        const BR_0                              = 0x0008,               //Bit 0
        const BR_1                              = 0x0010,               //Bit 1
        const BR_2                              = 0x0020,               //Bit 2
        const SPE                               = 0x0040,               //SPI Enable
        const LSBFIRST                          = 0x0080,               //Frame Format
        const SSI                               = 0x0100,               //Internal slave select
        const SSM                               = 0x0200,               //Software slave management
        const RXONLY                            = 0x0400,               //Receive only
        const CRCL                              = 0x0800,               //CRC Length
        const CRCNEXT                           = 0x1000,               //Transmit CRC next
        const CRCEN                             = 0x2000,               //Hardware CRC calculation enable
        const BIDIOE                            = 0x4000,               //Output enable in bidirectional mode
        const BIDIMODE                          = 0x8000,               //Bidirectional data mode enable
    }

    CR2: u16 {
        const RXDMAEN                           = 0x0001,               //Rx Buffer DMA Enable
        const TXDMAEN                           = 0x0002,               //Tx Buffer DMA Enable
        const SSOE                              = 0x0004,               //SS Output Enable
        const NSSP                              = 0x0008,               //NSS pulse management Enable
        const FRF                               = 0x0010,               //Frame Format Enable
        const ERRIE                             = 0x0020,               //Error Interrupt Enable
        const RXNEIE                            = 0x0040,               //RX buffer Not Empty Interrupt Enable
        const TXEIE                             = 0x0080,               //Tx buffer Empty Interrupt Enable
        const DS                                = 0x0F00,               //DS[3:0] Data Size
        const DS_0                              = 0x0100,               //Bit 0
        const DS_1                              = 0x0200,               //Bit 1
        const DS_2                              = 0x0400,               //Bit 2
        const DS_3                              = 0x0800,               //Bit 3
        const FRXTH                             = 0x1000,               //FIFO reception Threshold
        const LDMARX                            = 0x2000,               //Last DMA transfer for reception
        const LDMATX                            = 0x4000,               //Last DMA transfer for transmission
    }

    SR: u16 {
        const RXNE                              = 0x0001,               //Receive buffer Not Empty
        const TXE                               = 0x0002,               //Transmit buffer Empty
        const CRCERR                            = 0x0010,               //CRC Error flag
        const MODF                              = 0x0020,               //Mode fault
        const OVR                               = 0x0040,               //Overrun flag
        const BSY                               = 0x0080,               //Busy flag
        const FRE                               = 0x0100,               //TI frame format error
        const FRLVL                             = 0x0600,               //FIFO Reception Level
        const FRLVL_0                           = 0x0200,               //Bit 0
        const FRLVL_1                           = 0x0400,               //Bit 1
        const FTLVL                             = 0x1800,               //FIFO Transmission Level
        const FTLVL_0                           = 0x0800,               //Bit 0
        const FTLVL_1                           = 0x1000,               //Bit 1
    }

    DR: u16 {
        const DR                                = 0xFFFF,               //Data Register
    }

    CRCPR: u16 {
        const CRCPOLY                           = 0xFFFF,               //CRC polynomial register
    }

    RXCRCR: u16 {
        const RXCRC                             = 0xFFFF,               //Rx CRC Register
    }

    TXCRCR: u16 {
        const TXCRC                             = 0xFFFF,               //Tx CRC Register
    }

    I2SCFGR: u16 {
        const CHLEN                             = 0x0001,               //Channel length (number of bits per audio channel)
        const DATLEN                            = 0x0006,               //DATLEN[1:0] bits (Data length to be transferred)
        const DATLEN_0                          = 0x0002,               //Bit 0
        const DATLEN_1                          = 0x0004,               //Bit 1
        const CKPOL                             = 0x0008,               //steady state clock polarity
        const I2SSTD                            = 0x0030,               //I2SSTD[1:0] bits (I2S standard selection)
        const I2SSTD_0                          = 0x0010,               //Bit 0
        const I2SSTD_1                          = 0x0020,               //Bit 1
        const PCMSYNC                           = 0x0080,               //PCM frame synchronization
        const I2SCFG                            = 0x0300,               //I2SCFG[1:0] bits (I2S configuration mode)
        const I2SCFG_0                          = 0x0100,               //Bit 0
        const I2SCFG_1                          = 0x0200,               //Bit 1
        const I2SE                              = 0x0400,               //I2S Enable
        const I2SMOD                            = 0x0800,               //I2S mode selection
    }

    I2SPR: u16 {
        const I2SDIV                            = 0x00FF,               //I2S Linear prescaler
        const ODD                               = 0x0100,               //Odd factor for the prescaler
        const MCKOE                             = 0x0200,               //Master Clock Output Enable
    }
}

//custom

