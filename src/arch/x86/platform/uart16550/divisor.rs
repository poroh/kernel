// SPDX-License-Identifier: MIT
//
// 16550 UART / Divisor

use crate::arch::x86;
use crate::arch::x86::platform::uart16550::BaudRate;
use crate::arch::x86::platform::uart16550::UARTPortBase;

pub fn write(base: &UARTPortBase, val: &BaudRate) {
    let v = val.divisor();
    unsafe {
        x86::io::outb(&base.dll(), (v & 0xff) as u8);
        x86::io::outb(&base.dlh(), (v >> 8) as u8);
    };
}
