use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct USART {
    pub CR1:                u32,
    pub CR2:                u32,
    pub CR3:                u32,
    pub BRR:                u16,
    pub RESERVED1:          u16,
    pub GTPR:               u16,
    pub RESERVED2:          u16,
    pub RTOR:               u32,
    pub RQR:                u16,
    pub RESERVED3:          u16,
    pub ISR:                u32,
    pub ICR:                u32,
    pub RDR:                u16,
    pub RESERVED4:          u16,
    pub TDR:                u16,
    pub RESERVED5:          u16,
}

//register addresses

registers! {
    const USART2:       USART           = USART2_BASE,
    const USART3:       USART           = USART3_BASE,
    const UART4:        USART           = UART4_BASE,
    const UART5:        USART           = UART5_BASE,
    const USART1:       USART           = USART1_BASE,
}

//custom

