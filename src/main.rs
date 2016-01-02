#![feature(lang_items, core_intrinsics, const_fn)]
#![no_std]

mod discovery;
mod hardware;
mod ringbuffer;
mod runtime;
mod serial;
mod stm32f30x;
mod systick;

pub use runtime::{_exit, _kill, _getpid};
pub use systick::SysTick_Handler;

use core::intrinsics::{volatile_load, volatile_store};
use ringbuffer::Ringbuffer;
use stm32f30x::{gpio, usart};

#[no_mangle]
pub fn main() {
    systick::config(72);

    systick::core_clock_update();

    serial::init(&mut usart::USART1, &mut gpio::GPIOA, 10, &mut gpio::GPIOA, 9);

    loop {}
}

static mut ringbuf: Ringbuffer<u8> = Ringbuffer::new([0; ringbuffer::SIZE]);

#[no_mangle]
#[allow(non_snake_case)]
pub fn USART1_IRQHandler() {
    let usart = &mut *usart::USART1;
    if (usart.ISR & usart::ISR::RXNE) != 0 {
        unsafe {
            let data = volatile_load(&usart.RDR) as u8;
            if ringbuf.free_slots() > 0 {
                ringbuf.push(data);
                usart.CR1 |= usart::CR1::TCIE;
            }
        }
    } else if (usart.ISR & usart::ISR::TXE) != 0 {
        unsafe {
            let data = ringbuf.pop();
            volatile_store(&mut usart.TDR, data as u16);
            if ringbuf.filled_slots() == 0 {
                usart.CR1 &= !usart::CR1::TCIE;
            }
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

