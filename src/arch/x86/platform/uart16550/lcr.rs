// SPDX-License-Identifier: MIT
//
// 16550 UART / Line Control Register

use crate::arch::x86;
use crate::arch::x86::platform::uart16550::UARTPortBase;

pub struct Value {
    value: u8,
}

const DIVISOR_LATCH_BIT: u8 = 0x80;

impl Value {
    pub fn mode_8n1() -> Value {
        Value { value: 0x3 }
    }
    pub fn with_divisor_latched(&self) -> Value {
        Value {
            value: self.value | DIVISOR_LATCH_BIT,
        }
    }
}

#[inline]
pub fn write(base: &UARTPortBase, val: &Value) {
    unsafe { x86::io::outb(&base.lcr(), val.value) };
}

#[inline]
pub fn read(base: &UARTPortBase) -> Value {
    unsafe {
        Value {
            value: x86::io::inb(&base.lcr()),
        }
    }
}
