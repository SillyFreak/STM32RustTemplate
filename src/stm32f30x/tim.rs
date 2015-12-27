use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct TIM {
    pub CR1:                u16,
    pub RESERVED0:          u16,
    pub SMCR:               u32,
    pub DIER:               u32,
    pub SR:                 u32,
    pub EGR:                u32,
    pub CCMR1:              u32,
    pub CCMR2:              u32,
    pub CCER:               u32,
    pub CNT:                u32,
    pub PSC:                u16,
    pub RESERVED9:          u16,
    pub ARR:                u32,
    pub RCR:                u16,
    pub RESERVED10:         u16,
    pub CCR1:               u32,
    pub CCR2:               u32,
    pub CCR3:               u32,
    pub CCR4:               u32,
    pub BDTR:               u32,
    pub DCR:                u16,
    pub RESERVED12:         u16,
    pub DMAR:               u16,
    pub RESERVED13:         u16,
    pub OR:                 u16,
    pub CCMR3:              u32,
    pub CCR5:               u32,
    pub CCR6:               u32,
}

//register addresses

registers! {
    const TIM2:         TIM             = TIM2_BASE,
    const TIM3:         TIM             = TIM3_BASE,
    const TIM4:         TIM             = TIM4_BASE,
    const TIM6:         TIM             = TIM6_BASE,
    const TIM7:         TIM             = TIM7_BASE,
    const TIM1:         TIM             = TIM1_BASE,
    const TIM8:         TIM             = TIM8_BASE,
    const TIM15:        TIM             = TIM15_BASE,
    const TIM16:        TIM             = TIM16_BASE,
    const TIM17:        TIM             = TIM17_BASE,
    const TIM20:        TIM             = TIM20_BASE,
}

//custom

