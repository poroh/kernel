// SPDX-License-Identifier: MIT

#![no_std]
#![no_main]
#![feature(ascii_char)]
#![feature(ascii_char_variants)]

mod arch;
mod common;

use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    arch::x86::boot::init();
    let vga_buffer = 0xb8000 as *mut u8;
    unsafe {
        *vga_buffer.offset(0) = 0x52;
        *vga_buffer.offset(1) = 0xb;
    }

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
