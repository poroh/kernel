// SPDX-License-Identifier: MIT
//
// 16550 UART / Interrupt Control Register

use crate::drivers::io_access::PortOffset;
use crate::drivers::serial::uart16550::IOAccess;

const IER_OFFSET: u16 = 1;

pub struct Value {
    value: u8,
}

impl Value {
    pub fn no_interrupt() -> Value {
        Value { value: 0 }
    }
}

pub fn write<OT, IO>(io: &IO, val: Value)
where
    OT: From<u16>,
    IO: IOAccess<OT> + 'static,
{
    io.write_u8(PortOffset::new(IER_OFFSET.into()), val.value)
}
