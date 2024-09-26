// SPDX-License-Identifier: MIT
//
// 16550 UART / Modem Control Register

use crate::arch::x86;
use crate::arch::x86::platform::uart16550::UARTPortBase;

pub struct Value {
    value: u8,
}

impl Value {
    pub fn dtr_rts() -> Value {
        Value { value: 0x3 }
    }
}

pub fn write(base: &UARTPortBase, val: &Value) {
    unsafe { x86::io::outb(&base.mcr(), val.value) };
}
