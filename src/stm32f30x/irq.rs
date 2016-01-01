//TODO specific to STM32F303xC
#[repr(C)]
pub enum IRQn {
    //Cortex-M4 Processor Exceptions Numbers
    NonMaskableInt                              = -14,                  //2 Non Maskable Interrupt
    MemoryManagement                            = -12,                  //4 Cortex-M4 Memory Management Interrupt
    BusFault                                    = -11,                  //5 Cortex-M4 Bus Fault Interrupt
    UsageFault                                  = -10,                  //6 Cortex-M4 Usage Fault Interrupt
    SVCall                                      = -5,                   //11 Cortex-M4 SV Call Interrupt
    DebugMonitor                                = -4,                   //12 Cortex-M4 Debug Monitor Interrupt
    PendSV                                      = -2,                   //14 Cortex-M4 Pend SV Interrupt
    SysTick                                     = -1,                   //15 Cortex-M4 System Tick Interrupt
    //STM32 specific Interrupt Numbers
    WWDG                                        = 0,                    //Window WatchDog Interrupt
    PVD                                         = 1,                    //PVD through EXTI Line detection Interrupt
    TAMPER_STAMP                                = 2,                    //Tamper and TimeStamp interrupts
    RTC_WKUP                                    = 3,                    //RTC Wakeup interrupt through the EXTI lines 17, 19 & 20
    FLASH                                       = 4,                    //FLASH global Interrupt
    RCC                                         = 5,                    //RCC global Interrupt
    EXTI0                                       = 6,                    //EXTI Line0 Interrupt
    EXTI1                                       = 7,                    //EXTI Line1 Interrupt
    EXTI2_TS                                    = 8,                    //EXTI Line2 Interrupt and Touch Sense Interrupt
    EXTI3                                       = 9,                    //EXTI Line3 Interrupt
    EXTI4                                       = 10,                   //EXTI Line4 Interrupt
    DMA1_Channel1                               = 11,                   //DMA1 Channel 1 Interrupt
    DMA1_Channel2                               = 12,                   //DMA1 Channel 2 Interrupt
    DMA1_Channel3                               = 13,                   //DMA1 Channel 3 Interrupt
    DMA1_Channel4                               = 14,                   //DMA1 Channel 4 Interrupt
    DMA1_Channel5                               = 15,                   //DMA1 Channel 5 Interrupt
    DMA1_Channel6                               = 16,                   //DMA1 Channel 6 Interrupt
    DMA1_Channel7                               = 17,                   //DMA1 Channel 7 Interrupt
    ADC1_2                                      = 18,                   //ADC1 & ADC2 Interrupts
    USB_HP_CAN1_TX                              = 19,                   //USB Device High Priority or CAN1 TX Interrupts
    USB_LP_CAN1_RX0                             = 20,                   //USB Device Low Priority or CAN1 RX0 Interrupts  
    CAN1_RX1                                    = 21,                   //CAN1 RX1 Interrupt
    CAN1_SCE                                    = 22,                   //CAN1 SCE Interrupt
    EXTI9_5                                     = 23,                   //External Line[9:5] Interrupts
    TIM1_BRK_TIM15                              = 24,                   //TIM1 Break and TIM15 Interrupts
    TIM1_UP_TIM16                               = 25,                   //TIM1 Update and TIM16 Interrupts
    TIM1_TRG_COM_TIM17                          = 26,                   //TIM1 Trigger and Commutation and TIM17 Interrupt
    TIM1_CC                                     = 27,                   //TIM1 Capture Compare Interrupt
    TIM2                                        = 28,                   //TIM2 global Interrupt
    TIM3                                        = 29,                   //TIM3 global Interrupt
    TIM4                                        = 30,                   //TIM4 global Interrupt
    I2C1_EV                                     = 31,                   //I2C1 Event Interrupt
    I2C1_ER                                     = 32,                   //I2C1 Error Interrupt 
    I2C2_EV                                     = 33,                   //I2C2 Event Interrupt  
    I2C2_ER                                     = 34,                   //I2C2 Error Interrupt
    SPI1                                        = 35,                   //SPI1 global Interrupt
    SPI2                                        = 36,                   //SPI2 global Interrupt
    USART1                                      = 37,                   //USART1 global Interrupt
    USART2                                      = 38,                   //USART2 global Interrupt
    USART3                                      = 39,                   //USART3 global Interrupt  
    EXTI15_10                                   = 40,                   //External Line[15:10] Interrupts
    RTC_Alarm                                   = 41,                   //RTC Alarm (A and B) through EXTI Line Interrupt
    USBWakeUp                                   = 42,                   //USB Wakeup Interrupt
    TIM8_BRK                                    = 43,                   //TIM8 Break Interrupt
    TIM8_UP                                     = 44,                   //TIM8 Update Interrupt
    TIM8_TRG_COM                                = 45,                   //TIM8 Trigger and Commutation Interrupt
    TIM8_CC                                     = 46,                   //TIM8 Capture Compare Interrupt
    ADC3                                        = 47,                   //ADC3 global Interrupt
    SPI3                                        = 51,                   //SPI3 global Interrupt
    UART4                                       = 52,                   //UART4 global Interrupt
    UART5                                       = 53,                   //UART5 global Interrupt
    TIM6_DAC                                    = 54,                   //TIM6 global and DAC1&2 underrun error  interrupts
    TIM7                                        = 55,                   //TIM7 global Interrupt
    DMA2_Channel1                               = 56,                   //DMA2 Channel 1 global Interrupt
    DMA2_Channel2                               = 57,                   //DMA2 Channel 2 global Interrupt
    DMA2_Channel3                               = 58,                   //DMA2 Channel 3 global Interrupt
    DMA2_Channel4                               = 59,                   //DMA2 Channel 4 global Interrupt
    DMA2_Channel5                               = 60,                   //DMA2 Channel 5 global Interrupt
    ADC4                                        = 61,                   //ADC4  global Interrupt
    COMP1_2_3                                   = 64,                   //COMP1, COMP2 and COMP3 global Interrupt
    COMP4_5_6                                   = 65,                   //COMP5, COMP6 and COMP4 global Interrupt
    COMP7                                       = 66,                   //COMP7 global Interrupt
    USB_HP                                      = 74,                   //USB High Priority global Interrupt remap
    USB_LP                                      = 75,                   //USB Low Priority global Interrupt  remap
    USBWakeUp_RMP                               = 76,                   //USB Wakeup Interrupt remap
    FPU                                         = 81,                   //Floating point Interrupt
}

extern {
    fn NVIC_EnableIRQ_wrapper(IRQn: IRQn);
    fn NVIC_SetPriority_wrapper(IRQn: IRQn, priority: u32);
}

pub fn nvic_enable_irq(irqn: IRQn) {
    unsafe {
        NVIC_EnableIRQ_wrapper(irqn);
    }
}

pub fn nvic_set_priority(irqn: IRQn, priority: u32) {
    unsafe {
        NVIC_SetPriority_wrapper(irqn, priority);
    }
}

