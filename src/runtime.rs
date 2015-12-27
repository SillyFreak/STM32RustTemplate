extern crate core;

#[lang="stack_exhausted"]
fn stack_exhausted() {}

#[lang="eh_personality"]
fn eh_personality() {}

#[lang="panic_fmt"]
fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> ! {
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

