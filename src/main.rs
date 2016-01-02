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

use serial::Serial;
use stm32f30x::{gpio, usart};

#[no_mangle]
pub fn main() {
    systick::config(72);

    systick::core_clock_update();

    unsafe {
        serial.init();
    }

    loop {
        unsafe {
            while serial.rx.filled_slots() > 0 && serial.tx.free_slots() > 0 {
                serial.tx.push(serial.rx.pop())
            }
            serial.check();
        }
    }
}

static mut serial: Serial = Serial::new(usart::USART1, gpio::GPIOA, 10, gpio::GPIOA, 9);

#[no_mangle]
#[allow(non_snake_case)]
pub fn USART1_IRQHandler() {
    unsafe {
        serial.handler();
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

