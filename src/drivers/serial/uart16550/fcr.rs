// SPDX-License-Identifier: MIT
//
// 16550 UART / FIFO Control Register

use crate::drivers::io_access::PortOffset;
use crate::drivers::serial::uart16550::IOAccess;

const FCR_OFFSET: u16 = 2;

#[derive(Clone, Copy)]
pub struct Value {
    value: u8,
}

impl Value {
    pub fn no_fifo() -> Value {
        Value { value: 0 }
    }
}

pub fn write<OT, IO>(io: &IO, val: Value)
where
    OT: From<u16>,
    IO: IOAccess<OT> + 'static,
{
    io.write_u8(PortOffset::new(FCR_OFFSET.into()), val.value)
}
