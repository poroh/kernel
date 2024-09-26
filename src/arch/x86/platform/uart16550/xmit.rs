// SPDX-License-Identifier: MIT
//
// 16550 UART / Transmitter Holding Buffer/Receiver Buffer

use crate::arch::x86;
use crate::arch::x86::platform::uart16550::UARTPortBase;

#[inline]
pub fn write(base: &UARTPortBase, val: u8) {
    unsafe { x86::io::outb(&base.thr(), val) };
}

#[inline]
pub fn read(base: &UARTPortBase) -> u8 {
    unsafe { x86::io::inb(&base.rbr()) }
}
