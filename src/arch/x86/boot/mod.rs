// SPDX-License-Identifier: MIT

use crate::arch::x86;
use core::fmt::Write;

pub fn init() {
    let mut serial = x86::platform::boot_com1();
    let _ = serial.write_str("Hello kernel\n");
    let cr0 = x86::cpu::cr0::read();
    let _ = serial.write_fmt(format_args!("CR0: {cr0:?}\n"));
    let cr3 = x86::cpu::cr3::read();
    let _ = serial.write_fmt(format_args!("CR3: {cr3:?}\n"));
}
