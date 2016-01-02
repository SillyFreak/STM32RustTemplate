use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct DMA_Channel {
    pub CCR:                u32,                                        //DMA channel x configuration register
    pub CNDTR:              u32,                                        //DMA channel x number of data register
    pub CPAR:               u32,                                        //DMA channel x peripheral address register
    pub CMAR:               u32,                                        //DMA channel x memory address register
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct DMA {
    pub ISR:                u32,                                        //DMA interrupt status register,      Address offset: 0x00
    pub IFCR:               u32,                                        //DMA interrupt clear flag register,  Address offset: 0x04
}

//register addresses

registers! {
    const DMA1_CHANNEL1:    DMA_Channel         = DMA1_CHANNEL1_BASE,
    const DMA1_CHANNEL2:    DMA_Channel         = DMA1_CHANNEL2_BASE,
    const DMA1_CHANNEL3:    DMA_Channel         = DMA1_CHANNEL3_BASE,
    const DMA1_CHANNEL4:    DMA_Channel         = DMA1_CHANNEL4_BASE,
    const DMA1_CHANNEL5:    DMA_Channel         = DMA1_CHANNEL5_BASE,
    const DMA1_CHANNEL6:    DMA_Channel         = DMA1_CHANNEL6_BASE,
    const DMA1_CHANNEL7:    DMA_Channel         = DMA1_CHANNEL7_BASE,
    const DMA2_CHANNEL1:    DMA_Channel         = DMA2_CHANNEL1_BASE,
    const DMA2_CHANNEL2:    DMA_Channel         = DMA2_CHANNEL2_BASE,
    const DMA2_CHANNEL3:    DMA_Channel         = DMA2_CHANNEL3_BASE,
    const DMA2_CHANNEL4:    DMA_Channel         = DMA2_CHANNEL4_BASE,
    const DMA2_CHANNEL5:    DMA_Channel         = DMA2_CHANNEL5_BASE,
}

registers! {
    const DMA1:             DMA                 = DMA1_BASE,
    const DMA2:             DMA                 = DMA2_BASE,
}

//bit definitions

constants! {
    ISR: u32 {
        const GIF1                              = 0x00000001,           //Channel 1 Global interrupt flag
        const TCIF1                             = 0x00000002,           //Channel 1 Transfer Complete flag
        const HTIF1                             = 0x00000004,           //Channel 1 Half Transfer flag
        const TEIF1                             = 0x00000008,           //Channel 1 Transfer Error flag
        const GIF2                              = 0x00000010,           //Channel 2 Global interrupt flag
        const TCIF2                             = 0x00000020,           //Channel 2 Transfer Complete flag
        const HTIF2                             = 0x00000040,           //Channel 2 Half Transfer flag
        const TEIF2                             = 0x00000080,           //Channel 2 Transfer Error flag
        const GIF3                              = 0x00000100,           //Channel 3 Global interrupt flag
        const TCIF3                             = 0x00000200,           //Channel 3 Transfer Complete flag
        const HTIF3                             = 0x00000400,           //Channel 3 Half Transfer flag
        const TEIF3                             = 0x00000800,           //Channel 3 Transfer Error flag
        const GIF4                              = 0x00001000,           //Channel 4 Global interrupt flag
        const TCIF4                             = 0x00002000,           //Channel 4 Transfer Complete flag
        const HTIF4                             = 0x00004000,           //Channel 4 Half Transfer flag
        const TEIF4                             = 0x00008000,           //Channel 4 Transfer Error flag
        const GIF5                              = 0x00010000,           //Channel 5 Global interrupt flag
        const TCIF5                             = 0x00020000,           //Channel 5 Transfer Complete flag
        const HTIF5                             = 0x00040000,           //Channel 5 Half Transfer flag
        const TEIF5                             = 0x00080000,           //Channel 5 Transfer Error flag
        const GIF6                              = 0x00100000,           //Channel 6 Global interrupt flag
        const TCIF6                             = 0x00200000,           //Channel 6 Transfer Complete flag
        const HTIF6                             = 0x00400000,           //Channel 6 Half Transfer flag
        const TEIF6                             = 0x00800000,           //Channel 6 Transfer Error flag
        const GIF7                              = 0x01000000,           //Channel 7 Global interrupt flag
        const TCIF7                             = 0x02000000,           //Channel 7 Transfer Complete flag
        const HTIF7                             = 0x04000000,           //Channel 7 Half Transfer flag
        const TEIF7                             = 0x08000000,           //Channel 7 Transfer Error flag
    }

    IFCR: u32 {
        const CGIF1                             = 0x00000001,           //Channel 1 Global interrupt clear
        const CTCIF1                            = 0x00000002,           //Channel 1 Transfer Complete clear
        const CHTIF1                            = 0x00000004,           //Channel 1 Half Transfer clear
        const CTEIF1                            = 0x00000008,           //Channel 1 Transfer Error clear
        const CGIF2                             = 0x00000010,           //Channel 2 Global interrupt clear
        const CTCIF2                            = 0x00000020,           //Channel 2 Transfer Complete clear
        const CHTIF2                            = 0x00000040,           //Channel 2 Half Transfer clear
        const CTEIF2                            = 0x00000080,           //Channel 2 Transfer Error clear
        const CGIF3                             = 0x00000100,           //Channel 3 Global interrupt clear
        const CTCIF3                            = 0x00000200,           //Channel 3 Transfer Complete clear
        const CHTIF3                            = 0x00000400,           //Channel 3 Half Transfer clear
        const CTEIF3                            = 0x00000800,           //Channel 3 Transfer Error clear
        const CGIF4                             = 0x00001000,           //Channel 4 Global interrupt clear
        const CTCIF4                            = 0x00002000,           //Channel 4 Transfer Complete clear
        const CHTIF4                            = 0x00004000,           //Channel 4 Half Transfer clear
        const CTEIF4                            = 0x00008000,           //Channel 4 Transfer Error clear
        const CGIF5                             = 0x00010000,           //Channel 5 Global interrupt clear
        const CTCIF5                            = 0x00020000,           //Channel 5 Transfer Complete clear
        const CHTIF5                            = 0x00040000,           //Channel 5 Half Transfer clear
        const CTEIF5                            = 0x00080000,           //Channel 5 Transfer Error clear
        const CGIF6                             = 0x00100000,           //Channel 6 Global interrupt clear
        const CTCIF6                            = 0x00200000,           //Channel 6 Transfer Complete clear
        const CHTIF6                            = 0x00400000,           //Channel 6 Half Transfer clear
        const CTEIF6                            = 0x00800000,           //Channel 6 Transfer Error clear
        const CGIF7                             = 0x01000000,           //Channel 7 Global interrupt clear
        const CTCIF7                            = 0x02000000,           //Channel 7 Transfer Complete clear
        const CHTIF7                            = 0x04000000,           //Channel 7 Half Transfer clear
        const CTEIF7                            = 0x08000000,           //Channel 7 Transfer Error clear
    }
}

//custom

