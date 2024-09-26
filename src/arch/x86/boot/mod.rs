// SPDX-License-Identifier: MIT

use crate::arch::x86;
use crate::drivers::serial::Device;
use core::fmt::Write;

pub fn init() {
    let com1 = x86::platform::boot_com1();
    let mut serial = com1.formattable();
    let _ = serial.write_str("Hello kernel\n");
    let cr0 = x86::cpu::cr0::read();
    let _ = serial.write_fmt(format_args!("CR0: {cr0:?}\n"));
    let cr3 = x86::cpu::cr3::read();
    let _ = serial.write_fmt(format_args!("CR3: {cr3:?}\n"));
}
