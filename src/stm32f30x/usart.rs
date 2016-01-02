use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct USART {
    pub CR1:                u32,                                        //USART Control register 1,                 Address offset: 0x00
    pub CR2:                u32,                                        //USART Control register 2,                 Address offset: 0x04
    pub CR3:                u32,                                        //USART Control register 3,                 Address offset: 0x08
    pub BRR:                u16,                                        //USART Baud rate register,                 Address offset: 0x0C
    pub RESERVED1:          u16,                                        //Reserved, 0x0E
    pub GTPR:               u16,                                        //USART Guard time and prescaler register,  Address offset: 0x10
    pub RESERVED2:          u16,                                        //Reserved, 0x12
    pub RTOR:               u32,                                        //USART Receiver Time Out register,         Address offset: 0x14
    pub RQR:                u16,                                        //USART Request register,                   Address offset: 0x18
    pub RESERVED3:          u16,                                        //Reserved, 0x1A
    pub ISR:                u32,                                        //USART Interrupt and status register,      Address offset: 0x1C
    pub ICR:                u32,                                        //USART Interrupt flag Clear register,      Address offset: 0x20
    pub RDR:                u16,                                        //USART Receive Data register,              Address offset: 0x24
    pub RESERVED4:          u16,                                        //Reserved, 0x26
    pub TDR:                u16,                                        //USART Transmit Data register,             Address offset: 0x28
    pub RESERVED5:          u16,                                        //Reserved, 0x2A
}

//register addresses

registers! {
    const USART2:           USART               = USART2_BASE,
    const USART3:           USART               = USART3_BASE,
    const UART4:            USART               = UART4_BASE,
    const UART5:            USART               = UART5_BASE,
    const USART1:           USART               = USART1_BASE,
}

//bit definitions

constants! {
    CR1: u32 {
        const UE                                = 0x00000001,           //USART Enable
        const UESM                              = 0x00000002,           //USART Enable in STOP Mode
        const RE                                = 0x00000004,           //Receiver Enable
        const TE                                = 0x00000008,           //Transmitter Enable
        const IDLEIE                            = 0x00000010,           //IDLE Interrupt Enable
        const RXNEIE                            = 0x00000020,           //RXNE Interrupt Enable
        const TCIE                              = 0x00000040,           //Transmission Complete Interrupt Enable
        const TXEIE                             = 0x00000080,           //TXE Interrupt Enable
        const PEIE                              = 0x00000100,           //PE Interrupt Enable
        const PS                                = 0x00000200,           //Parity Selection
        const PCE                               = 0x00000400,           //Parity Control Enable
        const WAKE                              = 0x00000800,           //Receiver Wakeup method
        const M                                 = 0x00001000,           //Word length
        const MME                               = 0x00002000,           //Mute Mode Enable
        const CMIE                              = 0x00004000,           //Character match interrupt enable
        const OVER8                             = 0x00008000,           //Oversampling by 8-bit or 16-bit mode
        const DEDT                              = 0x001F0000,           //DEDT[4:0] bits (Driver Enable Deassertion Time)
        const DEDT_0                            = 0x00010000,           //Bit 0
        const DEDT_1                            = 0x00020000,           //Bit 1
        const DEDT_2                            = 0x00040000,           //Bit 2
        const DEDT_3                            = 0x00080000,           //Bit 3
        const DEDT_4                            = 0x00100000,           //Bit 4
        const DEAT                              = 0x03E00000,           //DEAT[4:0] bits (Driver Enable Assertion Time)
        const DEAT_0                            = 0x00200000,           //Bit 0
        const DEAT_1                            = 0x00400000,           //Bit 1
        const DEAT_2                            = 0x00800000,           //Bit 2
        const DEAT_3                            = 0x01000000,           //Bit 3
        const DEAT_4                            = 0x02000000,           //Bit 4
        const RTOIE                             = 0x04000000,           //Receive Time Out interrupt enable
        const EOBIE                             = 0x08000000,           //End of Block interrupt enable
    }

    CR2: u32 {
        const ADDM7                             = 0x00000010,           //7-bit or 4-bit Address Detection
        const LBDL                              = 0x00000020,           //LIN Break Detection Length
        const LBDIE                             = 0x00000040,           //LIN Break Detection Interrupt Enable
        const LBCL                              = 0x00000100,           //Last Bit Clock pulse
        const CPHA                              = 0x00000200,           //Clock Phase
        const CPOL                              = 0x00000400,           //Clock Polarity
        const CLKEN                             = 0x00000800,           //Clock Enable
        const STOP                              = 0x00003000,           //STOP[1:0] bits (STOP bits)
        const STOP_0                            = 0x00001000,           //Bit 0
        const STOP_1                            = 0x00002000,           //Bit 1
        const LINEN                             = 0x00004000,           //LIN mode enable
        const SWAP                              = 0x00008000,           //SWAP TX/RX pins
        const RXINV                             = 0x00010000,           //RX pin active level inversion
        const TXINV                             = 0x00020000,           //TX pin active level inversion
        const DATAINV                           = 0x00040000,           //Binary data inversion
        const MSBFIRST                          = 0x00080000,           //Most Significant Bit First
        const ABREN                             = 0x00100000,           //Auto Baud-Rate Enable
        const ABRMODE                           = 0x00600000,           //ABRMOD[1:0] bits (Auto Baud-Rate Mode)
        const ABRMODE_0                         = 0x00200000,           //Bit 0
        const ABRMODE_1                         = 0x00400000,           //Bit 1
        const RTOEN                             = 0x00800000,           //Receiver Time-Out enable
        const ADD                               = 0xFF000000,           //Address of the USART node
    }

    CR3: u32 {
        const EIE                               = 0x00000001,           //Error Interrupt Enable
        const IREN                              = 0x00000002,           //IrDA mode Enable
        const IRLP                              = 0x00000004,           //IrDA Low-Power
        const HDSEL                             = 0x00000008,           //Half-Duplex Selection
        const NACK                              = 0x00000010,           //SmartCard NACK enable
        const SCEN                              = 0x00000020,           //SmartCard mode enable
        const DMAR                              = 0x00000040,           //DMA Enable Receiver
        const DMAT                              = 0x00000080,           //DMA Enable Transmitter
        const RTSE                              = 0x00000100,           //RTS Enable
        const CTSE                              = 0x00000200,           //CTS Enable
        const CTSIE                             = 0x00000400,           //CTS Interrupt Enable
        const ONEBIT                            = 0x00000800,           //One sample bit method enable
        const OVRDIS                            = 0x00001000,           //Overrun Disable
        const DDRE                              = 0x00002000,           //DMA Disable on Reception Error
        const DEM                               = 0x00004000,           //Driver Enable Mode
        const DEP                               = 0x00008000,           //Driver Enable Polarity Selection
        const SCARCNT                           = 0x000E0000,           //SCARCNT[2:0] bits (SmartCard Auto-Retry Count)
        const SCARCNT_0                         = 0x00020000,           //Bit 0
        const SCARCNT_1                         = 0x00040000,           //Bit 1
        const SCARCNT_2                         = 0x00080000,           //Bit 2
        const WUS                               = 0x00300000,           //WUS[1:0] bits (Wake UP Interrupt Flag Selection)
        const WUS_0                             = 0x00100000,           //Bit 0
        const WUS_1                             = 0x00200000,           //Bit 1
        const WUFIE                             = 0x00400000,           //Wake Up Interrupt Enable
    }

    BRR: u16 {
        const DIV_FRACTION                      = 0x000F,               //Fraction of USARTDIV
        const DIV_MANTISSA                      = 0xFFF0,               //Mantissa of USARTDIV
    }

    GTPR: u16 {
        const PSC                               = 0x00FF,               //PSC[7:0] bits (Prescaler value)
        const GT                                = 0xFF00,               //GT[7:0] bits (Guard time value)
    }

    RTOR: u32 {
        const RTO                               = 0x00FFFFFF,           //Receiver Time Out Value
        const BLEN                              = 0xFF000000,           //Block Length
    }

    RQR: u16 {
        const ABRRQ                             = 0x0001,               //Auto-Baud Rate Request
        const SBKRQ                             = 0x0002,               //Send Break Request
        const MMRQ                              = 0x0004,               //Mute Mode Request
        const RXFRQ                             = 0x0008,               //Receive Data flush Request
        const TXFRQ                             = 0x0010,               //Transmit data flush Request
    }

    ISR: u32 {
        const PE                                = 0x00000001,           //Parity Error
        const FE                                = 0x00000002,           //Framing Error
        const NE                                = 0x00000004,           //Noise detected Flag
        const ORE                               = 0x00000008,           //OverRun Error
        const IDLE                              = 0x00000010,           //IDLE line detected
        const RXNE                              = 0x00000020,           //Read Data Register Not Empty
        const TC                                = 0x00000040,           //Transmission Complete
        const TXE                               = 0x00000080,           //Transmit Data Register Empty
        const LBD                               = 0x00000100,           //LIN Break Detection Flag
        const CTSIF                             = 0x00000200,           //CTS interrupt flag
        const CTS                               = 0x00000400,           //CTS flag
        const RTOF                              = 0x00000800,           //Receiver Time Out
        const EOBF                              = 0x00001000,           //End Of Block Flag
        const ABRE                              = 0x00004000,           //Auto-Baud Rate Error
        const ABRF                              = 0x00008000,           //Auto-Baud Rate Flag
        const BUSY                              = 0x00010000,           //Busy Flag
        const CMF                               = 0x00020000,           //Character Match Flag
        const SBKF                              = 0x00040000,           //Send Break Flag
        const RWU                               = 0x00080000,           //Receive Wake Up from mute mode Flag
        const WUF                               = 0x00100000,           //Wake Up from stop mode Flag
        const TEACK                             = 0x00200000,           //Transmit Enable Acknowledge Flag
        const REACK                             = 0x00400000,           //Receive Enable Acknowledge Flag
    }

    ICR: u32 {
        const PECF                              = 0x00000001,           //Parity Error Clear Flag
        const FECF                              = 0x00000002,           //Framing Error Clear Flag
        const NCF                               = 0x00000004,           //Noise detected Clear Flag
        const ORECF                             = 0x00000008,           //OverRun Error Clear Flag
        const IDLECF                            = 0x00000010,           //IDLE line detected Clear Flag
        const TCCF                              = 0x00000040,           //Transmission Complete Clear Flag
        const LBDCF                             = 0x00000100,           //LIN Break Detection Clear Flag
        const CTSCF                             = 0x00000200,           //CTS Interrupt Clear Flag
        const RTOCF                             = 0x00000800,           //Receiver Time Out Clear Flag
        const EOBCF                             = 0x00001000,           //End Of Block Clear Flag
        const CMCF                              = 0x00020000,           //Character Match Clear Flag
        const WUCF                              = 0x00100000,           //Wake Up from stop mode Clear Flag
    }

    RDR: u16 {
        const RDR                               = 0x01FF,               //RDR[8:0] bits (Receive Data value)
    }

    TDR: u16 {
        const TDR                               = 0x01FF,               //TDR[8:0] bits (Transmit Data value)
    }
}

//custom

