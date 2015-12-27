use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct DMA_Channel {
    pub CCR:                u32,
    pub CNDTR:              u32,
    pub CPAR:               u32,
    pub CMAR:               u32,
}

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct DMA {
    pub ISR:                u32,
    pub IFCR:               u32,
}

//register addresses

registers! {
    const DMA1_CHANNEL1: DMA_Channel     = DMA1_CHANNEL1_BASE,
    const DMA1_CHANNEL2: DMA_Channel     = DMA1_CHANNEL2_BASE,
    const DMA1_CHANNEL3: DMA_Channel     = DMA1_CHANNEL3_BASE,
    const DMA1_CHANNEL4: DMA_Channel     = DMA1_CHANNEL4_BASE,
    const DMA1_CHANNEL5: DMA_Channel     = DMA1_CHANNEL5_BASE,
    const DMA1_CHANNEL6: DMA_Channel     = DMA1_CHANNEL6_BASE,
    const DMA1_CHANNEL7: DMA_Channel     = DMA1_CHANNEL7_BASE,
    const DMA2_CHANNEL1: DMA_Channel     = DMA2_CHANNEL1_BASE,
    const DMA2_CHANNEL2: DMA_Channel     = DMA2_CHANNEL2_BASE,
    const DMA2_CHANNEL3: DMA_Channel     = DMA2_CHANNEL3_BASE,
    const DMA2_CHANNEL4: DMA_Channel     = DMA2_CHANNEL4_BASE,
    const DMA2_CHANNEL5: DMA_Channel     = DMA2_CHANNEL5_BASE,
}

registers! {
    const DMA1:         DMA             = DMA1_BASE,
    const DMA2:         DMA             = DMA2_BASE,
}

//custom

