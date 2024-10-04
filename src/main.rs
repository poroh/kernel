// SPDX-License-Identifier: MIT

#![no_std]
#![no_main]
#![feature(ascii_char)]
#![feature(ascii_char_variants)]
extern crate alloc;

mod arch;
mod common;
mod cpu;
#[allow(static_mut_refs)]
#[macro_use]
mod diag;
mod drivers;
mod memory;

use core::panic::PanicInfo;

core::arch::global_asm!(include_str!("pvh_note.s"));
core::arch::global_asm!(include_str!("pvh_start.s"));

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("Panic: {info:?}");
    arch::halt();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    arch::init();
    kprint!("\x1b[?7h");
    kprint!("\x1b[0m");
    kprint!("\x1b[c");
    //let x = alloc::boxed::Box::new(11);
    //kprintln!("Boxx: {x}");
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
