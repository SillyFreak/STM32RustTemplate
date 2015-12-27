use core::intrinsics::volatile_load;

extern {
    fn SystemCoreClockUpdate();
    fn SysTick_Config_wrapper(ticks: u32) -> u32;
}

pub fn core_clock_update() {
    unsafe {
        SystemCoreClockUpdate();
    }
}

pub fn config(ticks: u32) -> u32 {
    unsafe {
        return SysTick_Config_wrapper(ticks);
    }
}

static mut tick: u32 = 0;

pub fn systick() -> u32 {
    return unsafe { volatile_load(&tick) };
}

#[no_mangle]
#[allow(non_snake_case)]
pub fn SysTick_Handler() {
    unsafe {
        tick += 1;
    }
}

pub fn msleep(delay: u32) {
    usleep(delay * 1000);
}

pub fn usleep(delay: u32) {
    let start = systick();
    //`systick() - start` counts from 0 upwards,
    //so avoids overflows that interact strangely with `a < b`
    while systick() - start < delay {}
}

