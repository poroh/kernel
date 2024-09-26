// SPDX-License-Identifier: MIT
//
// 16550 UART / FIFO Control Register

use crate::arch::x86;
use crate::arch::x86::platform::uart16550::UARTPortBase;

pub struct Value {
    value: u8,
}

impl Value {
    pub fn no_fifo() -> Value {
        Value { value: 0 }
    }
}

pub fn write(base: &UARTPortBase, val: &Value) {
    unsafe { x86::io::outb(&base.fcr(), val.value) };
}
