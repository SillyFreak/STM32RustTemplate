use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct I2C {
    pub CR1:                u32,                                        //I2C Control register 1,            Address offset: 0x00
    pub CR2:                u32,                                        //I2C Control register 2,            Address offset: 0x04
    pub OAR1:               u32,                                        //I2C Own address 1 register,        Address offset: 0x08
    pub OAR2:               u32,                                        //I2C Own address 2 register,        Address offset: 0x0C
    pub TIMINGR:            u32,                                        //I2C Timing register,               Address offset: 0x10
    pub TIMEOUTR:           u32,                                        //I2C Timeout register,              Address offset: 0x14
    pub ISR:                u32,                                        //I2C Interrupt and status register, Address offset: 0x18
    pub ICR:                u32,                                        //I2C Interrupt clear register,      Address offset: 0x1C
    pub PECR:               u32,                                        //I2C PEC register,                  Address offset: 0x20
    pub RXDR:               u32,                                        //I2C Receive data register,         Address offset: 0x24
    pub TXDR:               u32,                                        //I2C Transmit data register,        Address offset: 0x28
}

//register addresses

registers! {
    const I2C1:             I2C                 = I2C1_BASE,
    const I2C2:             I2C                 = I2C2_BASE,
    const I2C3:             I2C                 = I2C3_BASE,
}

//bit definitions

constants! {
    CR1: u32 {
        const PE                                = 0x00000001,           //Peripheral enable
        const TXIE                              = 0x00000002,           //TX interrupt enable
        const RXIE                              = 0x00000004,           //RX interrupt enable
        const ADDRIE                            = 0x00000008,           //Address match interrupt enable
        const NACKIE                            = 0x00000010,           //NACK received interrupt enable
        const STOPIE                            = 0x00000020,           //STOP detection interrupt enable
        const TCIE                              = 0x00000040,           //Transfer complete interrupt enable
        const ERRIE                             = 0x00000080,           //Errors interrupt enable
        const DFN                               = 0x00000F00,           //Digital noise filter
        const ANFOFF                            = 0x00001000,           //Analog noise filter OFF
        const SWRST                             = 0x00002000,           //Software reset
        const TXDMAEN                           = 0x00004000,           //DMA transmission requests enable
        const RXDMAEN                           = 0x00008000,           //DMA reception requests enable
        const SBC                               = 0x00010000,           //Slave byte control
        const NOSTRETCH                         = 0x00020000,           //Clock stretching disable
        const WUPEN                             = 0x00040000,           //Wakeup from STOP enable
        const GCEN                              = 0x00080000,           //General call enable
        const SMBHEN                            = 0x00100000,           //SMBus host address enable
        const SMBDEN                            = 0x00200000,           //SMBus device default address enable
        const ALERTEN                           = 0x00400000,           //SMBus alert enable
        const PECEN                             = 0x00800000,           //PEC enable
    }

    CR2: u32 {
        const SADD                              = 0x000003FF,           //Slave address (master mode)
        const RD_WRN                            = 0x00000400,           //Transfer direction (master mode)
        const ADD10                             = 0x00000800,           //10-bit addressing mode (master mode)
        const HEAD10R                           = 0x00001000,           //10-bit address header only read direction (master mode)
        const START                             = 0x00002000,           //START generation
        const STOP                              = 0x00004000,           //STOP generation (master mode)
        const NACK                              = 0x00008000,           //NACK generation (slave mode)
        const NBYTES                            = 0x00FF0000,           //Number of bytes
        const RELOAD                            = 0x01000000,           //NBYTES reload mode
        const AUTOEND                           = 0x02000000,           //Automatic end mode (master mode)
        const PECBYTE                           = 0x04000000,           //Packet error checking byte
    }

    OAR1: u32 {
        const OA1                               = 0x000003FF,           //Interface own address 1
        const OA1MODE                           = 0x00000400,           //Own address 1 10-bit mode
        const OA1EN                             = 0x00008000,           //Own address 1 enable
    }

    OAR2: u32 {
        const OA2                               = 0x000000FE,           //Interface own address 2
        const OA2MSK                            = 0x00000700,           //Own address 2 masks
        const OA2EN                             = 0x00008000,           //Own address 2 enable
    }

    TIMINGR: u32 {
        const SCLL                              = 0x000000FF,           //SCL low period (master mode)
        const SCLH                              = 0x0000FF00,           //SCL high period (master mode)
        const SDADEL                            = 0x000F0000,           //Data hold time
        const SCLDEL                            = 0x00F00000,           //Data setup time
        const PRESC                             = 0xF0000000,           //Timings prescaler
    }

    TIMEOUTR: u32 {
        const TIMEOUTA                          = 0x00000FFF,           //Bus timeout A
        const TIDLE                             = 0x00001000,           //Idle clock timeout detection
        const TIMOUTEN                          = 0x00008000,           //Clock timeout enable
        const TIMEOUTB                          = 0x0FFF0000,           //Bus timeout B
        const TEXTEN                            = 0x80000000,           //Extended clock timeout enable
    }

    ISR: u32 {
        const TXE                               = 0x00000001,           //Transmit data register empty
        const TXIS                              = 0x00000002,           //Transmit interrupt status
        const RXNE                              = 0x00000004,           //Receive data register not empty
        const ADDR                              = 0x00000008,           //Address matched (slave mode)
        const NACKF                             = 0x00000010,           //NACK received flag
        const STOPF                             = 0x00000020,           //STOP detection flag
        const TC                                = 0x00000040,           //Transfer complete (master mode)
        const TCR                               = 0x00000080,           //Transfer complete reload
        const BERR                              = 0x00000100,           //Bus error
        const ARLO                              = 0x00000200,           //Arbitration lost
        const OVR                               = 0x00000400,           //Overrun/Underrun
        const PECERR                            = 0x00000800,           //PEC error in reception
        const TIMEOUT                           = 0x00001000,           //Timeout or Tlow detection flag
        const ALERT                             = 0x00002000,           //SMBus alert
        const BUSY                              = 0x00008000,           //Bus busy
        const DIR                               = 0x00010000,           //Transfer direction (slave mode)
        const ADDCODE                           = 0x00FE0000,           //Address match code (slave mode)
    }

    ICR: u32 {
        const ADDRCF                            = 0x00000008,           //Address matched clear flag
        const NACKCF                            = 0x00000010,           //NACK clear flag
        const STOPCF                            = 0x00000020,           //STOP detection clear flag
        const BERRCF                            = 0x00000100,           //Bus error clear flag
        const ARLOCF                            = 0x00000200,           //Arbitration lost clear flag
        const OVRCF                             = 0x00000400,           //Overrun/Underrun clear flag
        const PECCF                             = 0x00000800,           //PAC error clear flag
        const TIMOUTCF                          = 0x00001000,           //Timeout clear flag
        const ALERTCF                           = 0x00002000,           //Alert clear flag
    }

    PECR: u32 {
        const PEC                               = 0x000000FF,           //PEC register
    }

    RXDR: u32 {
        const RXDATA                            = 0x000000FF,           //8-bit receive data
    }

    TXDR: u32 {
        const TXDATA                            = 0x000000FF,           //8-bit transmit data
    }
}

//custom

