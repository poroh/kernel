// SPDX-License-Identifier: MIT
//
// 16550 UART / Line Status Register

use crate::arch::x86;
use crate::arch::x86::platform::uart16550::UARTPortBase;

pub struct Value {
    value: u8,
}

const XMT_READY_BIT: u8 = 0x20;
const RX_READY_BIT: u8 = 0x01;

impl Value {
    pub fn tx_ready(&self) -> bool {
        (self.value & XMT_READY_BIT) != 0
    }
    pub fn rx_ready(&self) -> bool {
        (self.value & RX_READY_BIT) != 0
    }
}

#[inline]
pub fn read(base: &UARTPortBase) -> Value {
    unsafe {
        Value {
            value: x86::io::inb(&base.lsr()),
        }
    }
}
