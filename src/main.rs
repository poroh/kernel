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

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("panic: {info:?}");
    arch::halt();
}

fn kernel_main() -> ! {
    kprint!("\x1b[?7h");
    kprint!("\x1b[0m");
    kprint!("\x1b[c");
    kprintln!("enter kernel_main and die");
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
