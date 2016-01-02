#![feature(lang_items, core_intrinsics, const_fn)]
#![no_std]

mod runtime;
mod hardware;
mod serial;
mod stm32f30x;
mod systick;
mod discovery;

pub use runtime::{_exit, _kill, _getpid};
pub use systick::SysTick_Handler;

use core::intrinsics::{volatile_load, volatile_store};
use stm32f30x::{gpio, usart};

#[no_mangle]
pub fn main() {
    systick::config(72);

    systick::core_clock_update();

    serial::init(&mut usart::USART1, &mut gpio::GPIOA, 10, &mut gpio::GPIOA, 9);
    
    loop {}
}

static mut data: u8 = 0;

#[no_mangle]
#[allow(non_snake_case)]
pub fn USART1_IRQHandler() {
    let usart = &mut *usart::USART1;
    if (usart.ISR & usart::ISR::RXNE) != 0 {
        unsafe {
            data = volatile_load(&usart.RDR) as u8;
            usart.CR1 |= usart::CR1::TCIE;
        }
    } else if (usart.ISR & usart::ISR::TXE) != 0 {
        unsafe {
            volatile_store(&mut usart.TDR, data as u16);
            usart.CR1 &= !usart::CR1::TCIE;
        }
    }
}

//interrupt stubs

#[no_mangle] #[allow(non_snake_case)] pub fn NMI_Handler() {}
#[no_mangle] #[allow(non_snake_case)] pub fn HardFault_Handler() { loop {} }
#[no_mangle] #[allow(non_snake_case)] pub fn MemManage_Handler() { loop {} }
#[no_mangle] #[allow(non_snake_case)] pub fn BusFault_Handler() { loop {} }
#[no_mangle] #[allow(non_snake_case)] pub fn UsageFault_Handler() { loop {} }
#[no_mangle] #[allow(non_snake_case)] pub fn SVC_Handler() {}
#[no_mangle] #[allow(non_snake_case)] pub fn DebugMon_Handler() {}
#[no_mangle] #[allow(non_snake_case)] pub fn PendSV_Handler() {}

