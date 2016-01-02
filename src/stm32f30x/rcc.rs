use hardware::registers::RegPtr;
use super::*;

//register structure

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct RCC {
    pub CR:                 u32,                                        //RCC clock control register,                                  Address offset: 0x00
    pub CFGR:               u32,                                        //RCC clock configuration register,                            Address offset: 0x04
    pub CIR:                u32,                                        //RCC clock interrupt register,                                Address offset: 0x08
    pub APB2RSTR:           u32,                                        //RCC APB2 peripheral reset register,                          Address offset: 0x0C
    pub APB1RSTR:           u32,                                        //RCC APB1 peripheral reset register,                          Address offset: 0x10
    pub AHBENR:             u32,                                        //RCC AHB peripheral clock register,                           Address offset: 0x14
    pub APB2ENR:            u32,                                        //RCC APB2 peripheral clock enable register,                   Address offset: 0x18
    pub APB1ENR:            u32,                                        //RCC APB1 peripheral clock enable register,                   Address offset: 0x1C
    pub BDCR:               u32,                                        //RCC Backup domain control register,                          Address offset: 0x20
    pub CSR:                u32,                                        //RCC clock control & status register,                         Address offset: 0x24
    pub AHBRSTR:            u32,                                        //RCC AHB peripheral reset register,                           Address offset: 0x28
    pub CFGR2:              u32,                                        //RCC clock configuration register 2,                          Address offset: 0x2C
    pub CFGR3:              u32,                                        //RCC clock configuration register 3,                          Address offset: 0x30
}

//register addresses

registers! {
    const RCC:              RCC                 = RCC_BASE,
}

//bit definitions

constants! {
    CR: u32 {
        const HSION                             = 0x00000001,
        const HSIRDY                            = 0x00000002,
        const HSITRIM                           = 0x000000F8,
        const HSITRIM_0                         = 0x00000008,           //Bit 0
        const HSITRIM_1                         = 0x00000010,           //Bit 1
        const HSITRIM_2                         = 0x00000020,           //Bit 2
        const HSITRIM_3                         = 0x00000040,           //Bit 3
        const HSITRIM_4                         = 0x00000080,           //Bit 4
        const HSICAL                            = 0x0000FF00,
        const HSICAL_0                          = 0x00000100,           //Bit 0
        const HSICAL_1                          = 0x00000200,           //Bit 1
        const HSICAL_2                          = 0x00000400,           //Bit 2
        const HSICAL_3                          = 0x00000800,           //Bit 3
        const HSICAL_4                          = 0x00001000,           //Bit 4
        const HSICAL_5                          = 0x00002000,           //Bit 5
        const HSICAL_6                          = 0x00004000,           //Bit 6
        const HSICAL_7                          = 0x00008000,           //Bit 7
        const HSEON                             = 0x00010000,
        const HSERDY                            = 0x00020000,
        const HSEBYP                            = 0x00040000,
        const CSSON                             = 0x00080000,
        const PLLON                             = 0x01000000,
        const PLLRDY                            = 0x02000000,
    }

    CFGR: u32 {
        const SW                                = 0x00000003,           //SW[1:0] bits (System clock Switch)
        const SW_0                              = 0x00000001,           //Bit 0
        const SW_1                              = 0x00000002,           //Bit 1
        const SW_HSI                            = 0x00000000,           //HSI selected as system clock
        const SW_HSE                            = 0x00000001,           //HSE selected as system clock
        const SW_PLL                            = 0x00000002,           //PLL selected as system clock
        const SWS                               = 0x0000000C,           //SWS[1:0] bits (System Clock Switch Status)
        const SWS_0                             = 0x00000004,           //Bit 0
        const SWS_1                             = 0x00000008,           //Bit 1
        const SWS_HSI                           = 0x00000000,           //HSI oscillator used as system clock
        const SWS_HSE                           = 0x00000004,           //HSE oscillator used as system clock
        const SWS_PLL                           = 0x00000008,           //PLL used as system clock
        const HPRE                              = 0x000000F0,           //HPRE[3:0] bits (AHB prescaler)
        const HPRE_0                            = 0x00000010,           //Bit 0
        const HPRE_1                            = 0x00000020,           //Bit 1
        const HPRE_2                            = 0x00000040,           //Bit 2
        const HPRE_3                            = 0x00000080,           //Bit 3
        const HPRE_DIV1                         = 0x00000000,           //SYSCLK not divided
        const HPRE_DIV2                         = 0x00000080,           //SYSCLK divided by 2
        const HPRE_DIV4                         = 0x00000090,           //SYSCLK divided by 4
        const HPRE_DIV8                         = 0x000000A0,           //SYSCLK divided by 8
        const HPRE_DIV16                        = 0x000000B0,           //SYSCLK divided by 16
        const HPRE_DIV64                        = 0x000000C0,           //SYSCLK divided by 64
        const HPRE_DIV128                       = 0x000000D0,           //SYSCLK divided by 128
        const HPRE_DIV256                       = 0x000000E0,           //SYSCLK divided by 256
        const HPRE_DIV512                       = 0x000000F0,           //SYSCLK divided by 512
        const PPRE1                             = 0x00000700,           //PRE1[2:0] bits (APB1 prescaler)
        const PPRE1_0                           = 0x00000100,           //Bit 0
        const PPRE1_1                           = 0x00000200,           //Bit 1
        const PPRE1_2                           = 0x00000400,           //Bit 2
        const PPRE1_DIV1                        = 0x00000000,           //HCLK not divided
        const PPRE1_DIV2                        = 0x00000400,           //HCLK divided by 2
        const PPRE1_DIV4                        = 0x00000500,           //HCLK divided by 4
        const PPRE1_DIV8                        = 0x00000600,           //HCLK divided by 8
        const PPRE1_DIV16                       = 0x00000700,           //HCLK divided by 16
        const PPRE2                             = 0x00003800,           //PRE2[2:0] bits (APB2 prescaler)
        const PPRE2_0                           = 0x00000800,           //Bit 0
        const PPRE2_1                           = 0x00001000,           //Bit 1
        const PPRE2_2                           = 0x00002000,           //Bit 2
        const PPRE2_DIV1                        = 0x00000000,           //HCLK not divided
        const PPRE2_DIV2                        = 0x00002000,           //HCLK divided by 2
        const PPRE2_DIV4                        = 0x00002800,           //HCLK divided by 4
        const PPRE2_DIV8                        = 0x00003000,           //HCLK divided by 8
        const PPRE2_DIV16                       = 0x00003800,           //HCLK divided by 16
        const PLLSRC                            = 0x00010000,           //PLL entry clock source
        const PLLXTPRE                          = 0x00020000,           //HSE divider for PLL entry
        const PLLMULL                           = 0x003C0000,           //PLLMUL[3:0] bits (PLL multiplication factor)
        const PLLMULL_0                         = 0x00040000,           //Bit 0
        const PLLMULL_1                         = 0x00080000,           //Bit 1
        const PLLMULL_2                         = 0x00100000,           //Bit 2
        const PLLMULL_3                         = 0x00200000,           //Bit 3
        const PLLSRC_HSI_DIV2                   = 0x00000000,           //HSI clock divided by 2 selected as PLL entry clock source
        const PLLSRC_HSI_PREDIV                 = 0x00008000,
        const PLLSRC_PREDIV1                    = 0x00010000,           //PREDIV1 clock selected as PLL entry clock source
        const PLLXTPRE_PREDIV1                  = 0x00000000,           //PREDIV1 clock not divided for PLL entry
        const PLLXTPRE_PREDIV1_DIV2             = 0x00020000,           //PREDIV1 clock divided by 2 for PLL entry
        const PLLMULL2                          = 0x00000000,           //PLL input clock*2
        const PLLMULL3                          = 0x00040000,           //PLL input clock*3
        const PLLMULL4                          = 0x00080000,           //PLL input clock*4
        const PLLMULL5                          = 0x000C0000,           //PLL input clock*5
        const PLLMULL6                          = 0x00100000,           //PLL input clock*6
        const PLLMULL7                          = 0x00140000,           //PLL input clock*7
        const PLLMULL8                          = 0x00180000,           //PLL input clock*8
        const PLLMULL9                          = 0x001C0000,           //PLL input clock*9
        const PLLMULL10                         = 0x00200000,           //PLL input clock10
        const PLLMULL11                         = 0x00240000,           //PLL input clock*11
        const PLLMULL12                         = 0x00280000,           //PLL input clock*12
        const PLLMULL13                         = 0x002C0000,           //PLL input clock*13
        const PLLMULL14                         = 0x00300000,           //PLL input clock*14
        const PLLMULL15                         = 0x00340000,           //PLL input clock*15
        const PLLMULL16                         = 0x00380000,           //PLL input clock*16
        const USBPRE                            = 0x00400000,           //USB prescaler
        const I2SSRC                            = 0x00800000,           //I2S external clock source selection
        const MCO                               = 0x07000000,           //MCO[2:0] bits (Microcontroller Clock Output)
        const MCO_0                             = 0x01000000,           //Bit 0
        const MCO_1                             = 0x02000000,           //Bit 1
        const MCO_2                             = 0x04000000,           //Bit 2
        const MCO_NOCLOCK                       = 0x00000000,           //No clock
        const MCO_LSI                           = 0x02000000,           //LSI clock selected as MCO source
        const MCO_LSE                           = 0x03000000,           //LSE clock selected as MCO source
        const MCO_SYSCLK                        = 0x04000000,           //System clock selected as MCO source
        const MCO_HSI                           = 0x05000000,           //HSI clock selected as MCO source
        const MCO_HSE                           = 0x06000000,           //HSE clock selected as MCO source
        const MCO_PLL                           = 0x07000000,           //PLL clock divided by 2 selected as MCO source
        const MCOF                              = 0x10000000,           //Microcontroller Clock Output Flag
        const MCO_PRE                           = 0x70000000,           //MCO prescaler
        const MCO_PRE_1                         = 0x00000000,           //MCO is divided by 1
        const MCO_PRE_2                         = 0x10000000,           //MCO is divided by 2
        const MCO_PRE_4                         = 0x20000000,           //MCO is divided by 4
        const MCO_PRE_8                         = 0x30000000,           //MCO is divided by 8
        const MCO_PRE_16                        = 0x40000000,           //MCO is divided by 16
        const MCO_PRE_32                        = 0x50000000,           //MCO is divided by 32
        const MCO_PRE_64                        = 0x60000000,           //MCO is divided by 64
        const MCO_PRE_128                       = 0x70000000,           //MCO is divided by 128
        const PLLNODIV                          = 0x80000000,           //PLL is not divided to MCO
    }

    CIR: u32 {
        const LSIRDYF                           = 0x00000001,           //LSI Ready Interrupt flag
        const LSERDYF                           = 0x00000002,           //LSE Ready Interrupt flag
        const HSIRDYF                           = 0x00000004,           //HSI Ready Interrupt flag
        const HSERDYF                           = 0x00000008,           //HSE Ready Interrupt flag
        const PLLRDYF                           = 0x00000010,           //PLL Ready Interrupt flag
        const CSSF                              = 0x00000080,           //Clock Security System Interrupt flag
        const LSIRDYIE                          = 0x00000100,           //LSI Ready Interrupt Enable
        const LSERDYIE                          = 0x00000200,           //LSE Ready Interrupt Enable
        const HSIRDYIE                          = 0x00000400,           //HSI Ready Interrupt Enable
        const HSERDYIE                          = 0x00000800,           //HSE Ready Interrupt Enable
        const PLLRDYIE                          = 0x00001000,           //PLL Ready Interrupt Enable
        const LSIRDYC                           = 0x00010000,           //LSI Ready Interrupt Clear
        const LSERDYC                           = 0x00020000,           //LSE Ready Interrupt Clear
        const HSIRDYC                           = 0x00040000,           //HSI Ready Interrupt Clear
        const HSERDYC                           = 0x00080000,           //HSE Ready Interrupt Clear
        const PLLRDYC                           = 0x00100000,           //PLL Ready Interrupt Clear
        const CSSC                              = 0x00800000,           //Clock Security System Interrupt Clear
    }

    APB2RSTR: u32 {
        const SYSCFGRST                         = 0x00000001,           //SYSCFG reset
        const TIM1RST                           = 0x00000200,           //TIM1 reset
        const SPI1RST                           = 0x00001000,           //SPI1 reset
        const TIM8RST                           = 0x00002000,           //TIM8 reset
        const USART1RST                         = 0x00004000,           //USART1 reset
        const SPI4RST                           = 0x00008000,           //SPI4 reset
        const TIM15RST                          = 0x00010000,           //TIM15 reset
        const TIM16RST                          = 0x00020000,           //TIM16 reset
        const TIM17RST                          = 0x00040000,           //TIM17 reset
        const TIM20RST                          = 0x00100000,           //TIM20 reset
        const HRTIM1RST                         = 0x20000000,           //HRTIM1 reset
    }

    APB1RSTR: u32 {
        const TIM2RST                           = 0x00000001,           //Timer 2 reset
        const TIM3RST                           = 0x00000002,           //Timer 3 reset
        const TIM4RST                           = 0x00000004,           //Timer 4 reset
        const TIM6RST                           = 0x00000010,           //Timer 6 reset
        const TIM7RST                           = 0x00000020,           //Timer 7 reset
        const WWDGRST                           = 0x00000800,           //Window Watchdog reset
        const SPI2RST                           = 0x00004000,           //SPI2 reset
        const SPI3RST                           = 0x00008000,           //SPI3 reset
        const USART2RST                         = 0x00020000,           //USART 2 reset
        const USART3RST                         = 0x00040000,           //USART 3 reset
        const UART4RST                          = 0x00080000,           //UART 4 reset
        const UART5RST                          = 0x00100000,           //UART 5 reset
        const I2C1RST                           = 0x00200000,           //I2C 1 reset
        const I2C2RST                           = 0x00400000,           //I2C 2 reset
        const USBRST                            = 0x00800000,           //USB reset
        const CAN1RST                           = 0x02000000,           //CAN reset
        const PWRRST                            = 0x10000000,           //PWR reset
        const DAC1RST                           = 0x20000000,           //DAC 1 reset
        const I2C3RST                           = 0x40000000,           //I2C 3 reset
        const DAC2RST                           = 0x04000000,           //DAC 2 reset
    }

    AHBENR: u32 {
        const DMA1EN                            = 0x00000001,           //DMA1 clock enable
        const DMA2EN                            = 0x00000002,           //DMA2 clock enable
        const SRAMEN                            = 0x00000004,           //SRAM interface clock enable
        const FLITFEN                           = 0x00000010,           //FLITF clock enable
        const FMCEN                             = 0x00000020,           //FMC clock enable
        const CRCEN                             = 0x00000040,           //CRC clock enable
        const GPIOHEN                           = 0x00010000,           //GPIOH clock enable
        const GPIOAEN                           = 0x00020000,           //GPIOA clock enable
        const GPIOBEN                           = 0x00040000,           //GPIOB clock enable
        const GPIOCEN                           = 0x00080000,           //GPIOC clock enable
        const GPIODEN                           = 0x00100000,           //GPIOD clock enable
        const GPIOEEN                           = 0x00200000,           //GPIOE clock enable
        const GPIOFEN                           = 0x00400000,           //GPIOF clock enable
        const GPIOGEN                           = 0x00800000,           //GPIOG clock enable
        const TSEN                              = 0x01000000,           //TS clock enable
        const ADC12EN                           = 0x10000000,           //ADC1/ ADC2 clock enable
        const ADC34EN                           = 0x20000000,           //ADC1/ ADC2 clock enable
    }

    APB2ENR: u32 {
        const SYSCFGEN                          = 0x00000001,           //SYSCFG clock enable
        const TIM1EN                            = 0x00000800,           //TIM1 clock enable
        const SPI1EN                            = 0x00001000,           //SPI1 clock enable
        const TIM8EN                            = 0x00002000,           //TIM8 clock enable
        const USART1EN                          = 0x00004000,           //USART1 clock enable
        const SPI4EN                            = 0x00008000,           //SPI4 clock enable
        const TIM15EN                           = 0x00010000,           //TIM15 clock enable
        const TIM16EN                           = 0x00020000,           //TIM16 clock enable
        const TIM17EN                           = 0x00040000,           //TIM17 clock enable
        const TIM20EN                           = 0x00100000,           //TIM20 clock enable
        const HRTIM1                            = 0x20000000,           //HRTIM1 clock enable
    }

    APB1ENR: u32 {
        const TIM2EN                            = 0x00000001,           //Timer 2 clock enable
        const TIM3EN                            = 0x00000002,           //Timer 3 clock enable
        const TIM4EN                            = 0x00000004,           //Timer 4 clock enable
        const TIM6EN                            = 0x00000010,           //Timer 6 clock enable
        const TIM7EN                            = 0x00000020,           //Timer 7 clock enable
        const WWDGEN                            = 0x00000800,           //Window Watchdog clock enable
        const SPI2EN                            = 0x00004000,           //SPI2 clock enable
        const SPI3EN                            = 0x00008000,           //SPI3 clock enable
        const USART2EN                          = 0x00020000,           //USART 2 clock enable
        const USART3EN                          = 0x00040000,           //USART 3 clock enable
        const UART4EN                           = 0x00080000,           //UART 4 clock enable
        const UART5EN                           = 0x00100000,           //UART 5 clock enable
        const I2C1EN                            = 0x00200000,           //I2C 1 clock enable
        const I2C2EN                            = 0x00400000,           //I2C 2 clock enable
        const USBEN                             = 0x00800000,           //USB clock enable
        const CAN1EN                            = 0x02000000,           //CAN clock enable
        const DAC2EN                            = 0x04000000,           //DAC 2 clock enable
        const PWREN                             = 0x10000000,           //PWR clock enable
        const DAC1EN                            = 0x20000000,           //DAC clock enable
        const I2C3EN                            = 0x40000000,           //I2C 3 clock enable
    }

    BDCR: u32 {
        const LSEON                             = 0x00000001,           //External Low Speed oscillator enable
        const LSERDY                            = 0x00000002,           //External Low Speed oscillator Ready
        const LSEBYP                            = 0x00000004,           //External Low Speed oscillator Bypass
        const LSEDRV                            = 0x00000018,           //LSEDRV[1:0] bits (LSE Osc. drive capability)
        const LSEDRV_0                          = 0x00000008,           //Bit 0
        const LSEDRV_1                          = 0x00000010,           //Bit 1
        const RTCSEL                            = 0x00000300,           //RTCSEL[1:0] bits (RTC clock source selection)
        const RTCSEL_0                          = 0x00000100,           //Bit 0
        const RTCSEL_1                          = 0x00000200,           //Bit 1
        const RTCSEL_NOCLOCK                    = 0x00000000,           //No clock
        const RTCSEL_LSE                        = 0x00000100,           //LSE oscillator clock used as RTC clock
        const RTCSEL_LSI                        = 0x00000200,           //LSI oscillator clock used as RTC clock
        const RTCSEL_HSE                        = 0x00000300,           //HSE oscillator clock divided by 32 used as RTC clock
        const RTCEN                             = 0x00008000,           //RTC clock enable
        const BDRST                             = 0x00010000,           //Backup domain software reset
    }

    CSR: u32 {
        const LSION                             = 0x00000001,           //Internal Low Speed oscillator enable
        const LSIRDY                            = 0x00000002,           //Internal Low Speed oscillator Ready
        const RMVF                              = 0x01000000,           //Remove reset flag
        const OBLRSTF                           = 0x02000000,           //OBL reset flag
        const PINRSTF                           = 0x04000000,           //PIN reset flag
        const PORRSTF                           = 0x08000000,           //POR/PDR reset flag
        const SFTRSTF                           = 0x10000000,           //Software Reset flag
        const IWDGRSTF                          = 0x20000000,           //Independent Watchdog reset flag
        const WWDGRSTF                          = 0x40000000,           //Window watchdog reset flag
        const LPWRRSTF                          = 0x80000000,           //Low-Power reset flag
    }

    AHBRSTR: u32 {
        const FMCRST                            = 0x00000020,           //FMC reset
        const GPIOHRST                          = 0x00010000,           //GPIOH reset
        const GPIOARST                          = 0x00020000,           //GPIOA reset
        const GPIOBRST                          = 0x00040000,           //GPIOB reset
        const GPIOCRST                          = 0x00080000,           //GPIOC reset
        const GPIODRST                          = 0x00010000,           //GPIOD reset
        const GPIOERST                          = 0x00200000,           //GPIOE reset
        const GPIOFRST                          = 0x00400000,           //GPIOF reset
        const GPIOGRST                          = 0x00800000,           //GPIOG reset
        const TSRST                             = 0x00100000,           //TS reset
        const ADC12RST                          = 0x01000000,           //ADC1 & ADC2 reset
        const ADC34RST                          = 0x02000000,           //ADC3 & ADC4 reset
    }

    CFGR2: u32 {
        const PREDIV1                           = 0x0000000F,           //PREDIV1[3:0] bits
        const PREDIV1_0                         = 0x00000001,           //Bit 0
        const PREDIV1_1                         = 0x00000002,           //Bit 1
        const PREDIV1_2                         = 0x00000004,           //Bit 2
        const PREDIV1_3                         = 0x00000008,           //Bit 3
        const PREDIV1_DIV1                      = 0x00000000,           //PREDIV1 input clock not divided
        const PREDIV1_DIV2                      = 0x00000001,           //PREDIV1 input clock divided by 2
        const PREDIV1_DIV3                      = 0x00000002,           //PREDIV1 input clock divided by 3
        const PREDIV1_DIV4                      = 0x00000003,           //PREDIV1 input clock divided by 4
        const PREDIV1_DIV5                      = 0x00000004,           //PREDIV1 input clock divided by 5
        const PREDIV1_DIV6                      = 0x00000005,           //PREDIV1 input clock divided by 6
        const PREDIV1_DIV7                      = 0x00000006,           //PREDIV1 input clock divided by 7
        const PREDIV1_DIV8                      = 0x00000007,           //PREDIV1 input clock divided by 8
        const PREDIV1_DIV9                      = 0x00000008,           //PREDIV1 input clock divided by 9
        const PREDIV1_DIV10                     = 0x00000009,           //PREDIV1 input clock divided by 10
        const PREDIV1_DIV11                     = 0x0000000A,           //PREDIV1 input clock divided by 11
        const PREDIV1_DIV12                     = 0x0000000B,           //PREDIV1 input clock divided by 12
        const PREDIV1_DIV13                     = 0x0000000C,           //PREDIV1 input clock divided by 13
        const PREDIV1_DIV14                     = 0x0000000D,           //PREDIV1 input clock divided by 14
        const PREDIV1_DIV15                     = 0x0000000E,           //PREDIV1 input clock divided by 15
        const PREDIV1_DIV16                     = 0x0000000F,           //PREDIV1 input clock divided by 16
        const ADCPRE12                          = 0x000001F0,           //ADCPRE12[8:4] bits
        const ADCPRE12_0                        = 0x00000010,           //Bit 0
        const ADCPRE12_1                        = 0x00000020,           //Bit 1
        const ADCPRE12_2                        = 0x00000040,           //Bit 2
        const ADCPRE12_3                        = 0x00000080,           //Bit 3
        const ADCPRE12_4                        = 0x00000100,           //Bit 4
        const ADCPRE12_NO                       = 0x00000000,           //ADC12 clock disabled, ADC12 can use AHB clock
        const ADCPRE12_DIV1                     = 0x00000100,           //ADC12 PLL clock divided by 1
        const ADCPRE12_DIV2                     = 0x00000110,           //ADC12 PLL clock divided by 2
        const ADCPRE12_DIV4                     = 0x00000120,           //ADC12 PLL clock divided by 4
        const ADCPRE12_DIV6                     = 0x00000130,           //ADC12 PLL clock divided by 6
        const ADCPRE12_DIV8                     = 0x00000140,           //ADC12 PLL clock divided by 8
        const ADCPRE12_DIV10                    = 0x00000150,           //ADC12 PLL clock divided by 10
        const ADCPRE12_DIV12                    = 0x00000160,           //ADC12 PLL clock divided by 12
        const ADCPRE12_DIV16                    = 0x00000170,           //ADC12 PLL clock divided by 16
        const ADCPRE12_DIV32                    = 0x00000180,           //ADC12 PLL clock divided by 32
        const ADCPRE12_DIV64                    = 0x00000190,           //ADC12 PLL clock divided by 64
        const ADCPRE12_DIV128                   = 0x000001A0,           //ADC12 PLL clock divided by 128
        const ADCPRE12_DIV256                   = 0x000001B0,           //ADC12 PLL clock divided by 256
        const ADCPRE34                          = 0x00003E00,           //ADCPRE34[13:5] bits
        const ADCPRE34_0                        = 0x00000200,           //Bit 0
        const ADCPRE34_1                        = 0x00000400,           //Bit 1
        const ADCPRE34_2                        = 0x00000800,           //Bit 2
        const ADCPRE34_3                        = 0x00001000,           //Bit 3
        const ADCPRE34_4                        = 0x00002000,           //Bit 4
        const ADCPRE34_NO                       = 0x00000000,           //ADC34 clock disabled, ADC34 can use AHB clock
        const ADCPRE34_DIV1                     = 0x00002000,           //ADC34 PLL clock divided by 1
        const ADCPRE34_DIV2                     = 0x00002200,           //ADC34 PLL clock divided by 2
        const ADCPRE34_DIV4                     = 0x00002400,           //ADC34 PLL clock divided by 4
        const ADCPRE34_DIV6                     = 0x00002600,           //ADC34 PLL clock divided by 6
        const ADCPRE34_DIV8                     = 0x00002800,           //ADC34 PLL clock divided by 8
        const ADCPRE34_DIV10                    = 0x00002A00,           //ADC34 PLL clock divided by 10
        const ADCPRE34_DIV12                    = 0x00002C00,           //ADC34 PLL clock divided by 12
        const ADCPRE34_DIV16                    = 0x00002E00,           //ADC34 PLL clock divided by 16
        const ADCPRE34_DIV32                    = 0x00003000,           //ADC34 PLL clock divided by 32
        const ADCPRE34_DIV64                    = 0x00003200,           //ADC34 PLL clock divided by 64
        const ADCPRE34_DIV128                   = 0x00003400,           //ADC34 PLL clock divided by 128
        const ADCPRE34_DIV256                   = 0x00003600,           //ADC34 PLL clock divided by 256
    }

    CFGR3: u32 {
        const USART1SW                          = 0x00000003,           //USART1SW[1:0] bits
        const USART1SW_0                        = 0x00000001,           //Bit 0
        const USART1SW_1                        = 0x00000002,           //Bit 1
        const I2CSW                             = 0x00000070,           //I2CSW bits
        const I2C1SW                            = 0x00000010,           //I2C1SW bits
        const I2C2SW                            = 0x00000020,           //I2C2SW bits
        const I2C3SW                            = 0x00000040,           //I2C3SW bits
        const TIMSW                             = 0x00002F00,           //TIMSW bits
        const TIM1SW                            = 0x00000100,           //TIM1SW bits
        const TIM8SW                            = 0x00000200,           //TIM8SW bits
        const TIM15SW                           = 0x00000400,           //TIM15SW bits
        const TIM16SW                           = 0x00000800,           //TIM16SW bits
        const TIM17SW                           = 0x00002000,           //TIM17SW bits
        const TIM20SW                           = 0x00008000,           //TIM20SW bits
        const TIM2SW                            = 0x01000000,           //TIM3SW bits
        const TIM3SW                            = 0x02000000,           //TIM2SW bits
        const HRTIM1SW                          = 0x00001000,           //HRTIM1SW bits
        const USART2SW                          = 0x00030000,           //USART2SW[1:0] bits
        const USART2SW_0                        = 0x00010000,           //Bit 0
        const USART2SW_1                        = 0x00020000,           //Bit 1
        const USART3SW                          = 0x000C0000,           //USART3SW[1:0] bits
        const USART3SW_0                        = 0x00040000,           //Bit 0
        const USART3SW_1                        = 0x00080000,           //Bit 1
        const UART4SW                           = 0x00300000,           //UART4SW[1:0] bits
        const UART4SW_0                         = 0x00100000,           //Bit 0
        const UART4SW_1                         = 0x00200000,           //Bit 1
        const UART5SW                           = 0x00C00000,           //UART5SW[1:0] bits
        const UART5SW_0                         = 0x00400000,           //Bit 0
        const UART5SW_1                         = 0x00800000,           //Bit 1
    }
}

//custom

