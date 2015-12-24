#![allow(dead_code)]

pub mod button;
pub mod led;

extern {
    fn SystemCoreClockUpdate();
    fn _delay_ms(delay: u32);
}

pub fn core_clock_update() {
    unsafe {
        SystemCoreClockUpdate();
    }
}

pub fn delay_ms(delay: u32) {
    unsafe {
        _delay_ms(delay);
    }
}

pub mod systick {
    extern {
        fn SysTick_Config_wrapper(ticks: u32) -> u32;
    }

    pub fn config(ticks: u32) -> u32 {
        unsafe {
            return SysTick_Config_wrapper(ticks);
        }
    }
}

