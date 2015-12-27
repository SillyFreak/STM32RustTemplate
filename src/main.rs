#![feature(lang_items, core_intrinsics, const_fn)]
#![no_std]

mod runtime;
mod hardware;
mod stm32f30x;
mod systick;
mod discovery;

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

//interrupts

#[no_mangle]
#[allow(non_snake_case)]
pub fn SysTick_Handler() {
    systick::systick_handler();
}

//TODO stubs that the linker otherwise misses; seems to have to do with assert

#[no_mangle]
pub fn _exit() -> ! {
    loop {}
}

#[no_mangle]
pub fn _kill() -> ! {
    loop {}
}

#[no_mangle]
pub fn _getpid() -> ! {
    loop {}
}

