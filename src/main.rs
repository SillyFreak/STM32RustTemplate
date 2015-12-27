#![feature(lang_items, core_intrinsics, const_fn)]
#![no_std]

mod runtime;
mod hardware;
mod stm32f30x;
mod systick;
mod discovery;

pub use runtime::{_exit, _kill, _getpid};
pub use systick::SysTick_Handler;

use discovery::led::Led;

#[no_mangle]
pub fn main() {
    systick::config(72);

	systick::core_clock_update();

    for i in 0..8 {
        discovery::led::LED[i].led_init();
    }
    
    let off = 1;
    let mut led = 0;
    loop {
        discovery::led::LED[led].led_toggle();
		systick::msleep(125);
		led = (led + off) % 8;
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

