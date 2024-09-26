// SPDX-License-Identifier: MIT

#![no_std]
#![no_main]
#![feature(ascii_char)]
#![feature(ascii_char_variants)]

mod arch;
mod common;

use crate::arch::x86;
use core::fmt::Write;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static mut bootloader::BootInfo) -> ! {
    arch::x86::boot::init();
    let mut serial = x86::platform::boot_com1();
    let _ = write!(serial, "\x1bc");
    let _ = write!(serial, "\x1b[?7h");
    let _ = writeln!(serial, "boot info: {boot_info:?}");
    let _ = write!(serial, "\x1b[0m");
    let _ = write!(serial, "\x1b[c");
    loop {
        match serial.try_read().and_then(core::ascii::Char::from_u8) {
            Some(core::ascii::Char::Escape) => (),
            Some(c) => serial.put_ascii(c),
            None => (),
        }
        x86::cpu::relax();
    }
}
