// SPDX-License-Identifier: MIT

#![no_std]
#![no_main]
#![feature(ascii_char)]
#![feature(ascii_char_variants)]

extern crate alloc;

mod arch;
mod common;
mod cpu;
#[macro_use]
mod diag;
mod drivers;
mod memory;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("Panic: {info:?}");
    arch::halt();
}

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static mut bootloader::BootInfo) -> ! {
    arch::init();
    //let _ = write!(serial, "\x1bc");
    kprint!("\x1b[?7h");
    kprintln!("boot info: {boot_info:?}");
    kprint!("\x1b[0m");
    kprint!("\x1b[c");
    for mr in boot_info.memory_map.iter() {
        kprintln!("Region: {mr:?}");
    }
    //    panic!("Test panic");
    let x = alloc::boxed::Box::new(11);
    kprintln!("Boxed: {x}");
    loop {
        /*
            match serial.try_read().and_then(core::ascii::Char::from_u8) {
                Some(core::ascii::Char::Escape) => (),
                Some(c) => serial.put_ascii(c),
                None => (),
        }
            */
        cpu::relax();
    }
}
