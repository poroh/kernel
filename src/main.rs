// SPDX-License-Identifier: MIT

#![no_std]
#![no_main]
#![feature(ascii_char)]
#![feature(ascii_char_variants)]

extern crate alloc;

mod arch;
mod common;
mod cpu;
mod drivers;
mod memory;

use crate::arch::x86;
use crate::drivers::serial::Device;
use core::fmt::Write;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let com1 = x86::platform::boot_com1();
    let mut serial = com1.formattable();
    let _ = writeln!(serial, "Panic: {info:?}");
    loop {}
}

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static mut bootloader::BootInfo) -> ! {
    arch::x86::boot::init();
    let com1 = x86::platform::boot_com1();
    let mut serial = com1.formattable();
    //let _ = write!(serial, "\x1bc");
    let _ = write!(serial, "\x1b[?7h");
    let _ = writeln!(serial, "boot info: {boot_info:?}");
    let _ = write!(serial, "\x1b[0m");
    let _ = write!(serial, "\x1b[c");
    for mr in boot_info.memory_map.iter() {
        let _ = writeln!(serial, "Region: {mr:?}");
    }
    //    panic!("Test panic");
    let x = alloc::boxed::Box::new(11);
    let _ = writeln!(serial, "Boxed: {x}");
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
