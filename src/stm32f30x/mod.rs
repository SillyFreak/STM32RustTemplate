#![allow(dead_code)]

pub mod adc;      //ADC, ADC_Common
pub mod can;      //CAN_TxMailBox, CAN_FIFOMailBox, CAN_FilterRegister, CAN
pub mod comp;     //COMP
pub mod crc;      //CRC
pub mod dac;      //DAC
pub mod dbgmcu;   //DBGMCU
pub mod dma;      //DMA_Channel, DMA
pub mod exti;     //EXTI
pub mod flash;    //FLASH, OB
pub mod fmc;      //FMC_Bank1, FMC_Bank1E, FMC_Bank2, FMC_Bank3, FMC_Bank4
pub mod gpio;     //GPIO
pub mod hrtim;    //HRTIM_Master, HRTIM_Timerx, HRTIM_Common, HRTIM
pub mod i2c;      //I2C
pub mod iwdg;     //IWDG
pub mod opamp;    //OPAMP
pub mod pwr;      //PWR
pub mod rcc;      //RCC
pub mod rtc;      //RTC
pub mod spi;      //SPI
pub mod syscfg;   //SYSCFG
pub mod tim;      //TIM
pub mod tsc;      //TSC
pub mod usart;    //USART
pub mod wwdg;     //WWDG

pub const FLASH_BASE:         usize = 0x08000000;
pub const CCMDATARAM_BASE:    usize = 0x10000000;
pub const SRAM_BASE:          usize = 0x20000000;
pub const PERIPH_BASE:        usize = 0x40000000;
pub const FMC_R_BASE:         usize = 0xA0000000;
pub const CCMDATARAM_BB_BASE: usize = 0x12000000;
pub const SRAM_BB_BASE:       usize = 0x22000000;
pub const PERIPH_BB_BASE:     usize = 0x42000000;

pub const APB1PERIPH_BASE:    usize = PERIPH_BASE + 0x00000000;
pub const APB2PERIPH_BASE:    usize = PERIPH_BASE + 0x00010000;
pub const AHB1PERIPH_BASE:    usize = PERIPH_BASE + 0x00020000;
pub const AHB2PERIPH_BASE:    usize = PERIPH_BASE + 0x08000000;
pub const AHB3PERIPH_BASE:    usize = PERIPH_BASE + 0x10000000;

pub const TIM2_BASE:          usize = APB1PERIPH_BASE + 0x00000000;
pub const TIM3_BASE:          usize = APB1PERIPH_BASE + 0x00000400;
pub const TIM4_BASE:          usize = APB1PERIPH_BASE + 0x00000800;
pub const TIM6_BASE:          usize = APB1PERIPH_BASE + 0x00001000;
pub const TIM7_BASE:          usize = APB1PERIPH_BASE + 0x00001400;
pub const RTC_BASE:           usize = APB1PERIPH_BASE + 0x00002800;
pub const WWDG_BASE:          usize = APB1PERIPH_BASE + 0x00002C00;
pub const IWDG_BASE:          usize = APB1PERIPH_BASE + 0x00003000;
pub const I2S2EXT_BASE:       usize = APB1PERIPH_BASE + 0x00003400;
pub const SPI2_BASE:          usize = APB1PERIPH_BASE + 0x00003800;
pub const SPI3_BASE:          usize = APB1PERIPH_BASE + 0x00003C00;
pub const I2S3EXT_BASE:       usize = APB1PERIPH_BASE + 0x00004000;
pub const USART2_BASE:        usize = APB1PERIPH_BASE + 0x00004400;
pub const USART3_BASE:        usize = APB1PERIPH_BASE + 0x00004800;
pub const UART4_BASE:         usize = APB1PERIPH_BASE + 0x00004C00;
pub const UART5_BASE:         usize = APB1PERIPH_BASE + 0x00005000;
pub const I2C1_BASE:          usize = APB1PERIPH_BASE + 0x00005400;
pub const I2C2_BASE:          usize = APB1PERIPH_BASE + 0x00005800;
pub const CAN1_BASE:          usize = APB1PERIPH_BASE + 0x00006400;
pub const PWR_BASE:           usize = APB1PERIPH_BASE + 0x00007000;
pub const DAC1_BASE:          usize = APB1PERIPH_BASE + 0x00007400;
pub const I2C3_BASE:          usize = APB1PERIPH_BASE + 0x00007800;
pub const DAC2_BASE:          usize = APB1PERIPH_BASE + 0x00009800;

pub const SYSCFG_BASE:        usize = APB2PERIPH_BASE + 0x00000000;
pub const COMP_BASE:          usize = APB2PERIPH_BASE + 0x0000001C;
pub const COMP1_BASE:         usize = APB2PERIPH_BASE + 0x0000001C;
pub const COMP2_BASE:         usize = APB2PERIPH_BASE + 0x00000020;
pub const COMP3_BASE:         usize = APB2PERIPH_BASE + 0x00000024;
pub const COMP4_BASE:         usize = APB2PERIPH_BASE + 0x00000028;
pub const COMP5_BASE:         usize = APB2PERIPH_BASE + 0x0000002C;
pub const COMP6_BASE:         usize = APB2PERIPH_BASE + 0x00000030;
pub const COMP7_BASE:         usize = APB2PERIPH_BASE + 0x00000034;
pub const OPAMP_BASE:         usize = APB2PERIPH_BASE + 0x00000038;
pub const OPAMP1_BASE:        usize = APB2PERIPH_BASE + 0x00000038;
pub const OPAMP2_BASE:        usize = APB2PERIPH_BASE + 0x0000003C;
pub const OPAMP3_BASE:        usize = APB2PERIPH_BASE + 0x00000040;
pub const OPAMP4_BASE:        usize = APB2PERIPH_BASE + 0x00000044;
pub const EXTI_BASE:          usize = APB2PERIPH_BASE + 0x00000400;
pub const TIM1_BASE:          usize = APB2PERIPH_BASE + 0x00002C00;
pub const SPI1_BASE:          usize = APB2PERIPH_BASE + 0x00003000;
pub const TIM8_BASE:          usize = APB2PERIPH_BASE + 0x00003400;
pub const USART1_BASE:        usize = APB2PERIPH_BASE + 0x00003800;
pub const SPI4_BASE:          usize = APB2PERIPH_BASE + 0x00003C00;
pub const TIM15_BASE:         usize = APB2PERIPH_BASE + 0x00004000;
pub const TIM16_BASE:         usize = APB2PERIPH_BASE + 0x00004400;
pub const TIM17_BASE:         usize = APB2PERIPH_BASE + 0x00004800;
pub const TIM20_BASE:         usize = APB2PERIPH_BASE + 0x00005000;
pub const HRTIM1_BASE:        usize = APB2PERIPH_BASE + 0x00007400;
pub const HRTIM1_TIMA_BASE:   usize = HRTIM1_BASE + 0x00000080;
pub const HRTIM1_TIMB_BASE:   usize = HRTIM1_BASE + 0x00000100;
pub const HRTIM1_TIMC_BASE:   usize = HRTIM1_BASE + 0x00000180;
pub const HRTIM1_TIMD_BASE:   usize = HRTIM1_BASE + 0x00000200;
pub const HRTIM1_TIME_BASE:   usize = HRTIM1_BASE + 0x00000280;
pub const HRTIM1_COMMON_BASE: usize = HRTIM1_BASE + 0x00000380;

pub const DMA1_BASE:          usize = AHB1PERIPH_BASE + 0x00000000;
pub const DMA1_CHANNEL1_BASE: usize = AHB1PERIPH_BASE + 0x00000008;
pub const DMA1_CHANNEL2_BASE: usize = AHB1PERIPH_BASE + 0x0000001C;
pub const DMA1_CHANNEL3_BASE: usize = AHB1PERIPH_BASE + 0x00000030;
pub const DMA1_CHANNEL4_BASE: usize = AHB1PERIPH_BASE + 0x00000044;
pub const DMA1_CHANNEL5_BASE: usize = AHB1PERIPH_BASE + 0x00000058;
pub const DMA1_CHANNEL6_BASE: usize = AHB1PERIPH_BASE + 0x0000006C;
pub const DMA1_CHANNEL7_BASE: usize = AHB1PERIPH_BASE + 0x00000080;
pub const DMA2_BASE:          usize = AHB1PERIPH_BASE + 0x00000400;
pub const DMA2_CHANNEL1_BASE: usize = AHB1PERIPH_BASE + 0x00000408;
pub const DMA2_CHANNEL2_BASE: usize = AHB1PERIPH_BASE + 0x0000041C;
pub const DMA2_CHANNEL3_BASE: usize = AHB1PERIPH_BASE + 0x00000430;
pub const DMA2_CHANNEL4_BASE: usize = AHB1PERIPH_BASE + 0x00000444;
pub const DMA2_CHANNEL5_BASE: usize = AHB1PERIPH_BASE + 0x00000458;
pub const RCC_BASE:           usize = AHB1PERIPH_BASE + 0x00001000;
pub const FLASH_R_BASE:       usize = AHB1PERIPH_BASE + 0x00002000;
pub const OB_BASE:            usize = 0x1FFFF800;
pub const CRC_BASE:           usize = AHB1PERIPH_BASE + 0x00003000;
pub const TSC_BASE:           usize = AHB1PERIPH_BASE + 0x00004000;

pub const GPIOA_BASE:         usize = AHB2PERIPH_BASE + 0x0000;
pub const GPIOB_BASE:         usize = AHB2PERIPH_BASE + 0x0400;
pub const GPIOC_BASE:         usize = AHB2PERIPH_BASE + 0x0800;
pub const GPIOD_BASE:         usize = AHB2PERIPH_BASE + 0x0C00;
pub const GPIOE_BASE:         usize = AHB2PERIPH_BASE + 0x1000;
pub const GPIOF_BASE:         usize = AHB2PERIPH_BASE + 0x1400;
pub const GPIOG_BASE:         usize = AHB2PERIPH_BASE + 0x00001800;
pub const GPIOH_BASE:         usize = AHB2PERIPH_BASE + 0x00001C00;

pub const ADC1_BASE:          usize = AHB3PERIPH_BASE + 0x0000;
pub const ADC2_BASE:          usize = AHB3PERIPH_BASE + 0x0100;
pub const ADC1_2_BASE:        usize = AHB3PERIPH_BASE + 0x0300;
pub const ADC3_BASE:          usize = AHB3PERIPH_BASE + 0x0400;
pub const ADC4_BASE:          usize = AHB3PERIPH_BASE + 0x0500;
pub const ADC3_4_BASE:        usize = AHB3PERIPH_BASE + 0x0700;

pub const FMC_BANK1_R_BASE:   usize = FMC_R_BASE + 0x0000;
pub const FMC_BANK1E_R_BASE:  usize = FMC_R_BASE + 0x0104;
pub const FMC_BANK2_R_BASE:   usize = FMC_R_BASE + 0x0060;
pub const FMC_BANK3_R_BASE:   usize = FMC_R_BASE + 0x0080;
pub const FMC_BANK4_R_BASE:   usize = FMC_R_BASE + 0x00A0;

pub const DBGMCU_BASE:        usize = 0xE0042000;

