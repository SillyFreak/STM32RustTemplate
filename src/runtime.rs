extern crate core;

#[lang="stack_exhausted"]
extern fn stack_exhausted() {}

#[lang="eh_personality"]
extern fn eh_personality() {}

#[lang="panic_fmt"]
pub fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> ! {
    loop {}
}

//TODO stubs that the linker otherwise misses; seems to have to do with assert

#[no_mangle]
pub extern fn _exit() -> ! {
    loop {}
}

#[no_mangle]
pub extern fn _kill() -> ! {
    loop {}
}

#[no_mangle]
pub extern fn _getpid() -> ! {
    loop {}
}

