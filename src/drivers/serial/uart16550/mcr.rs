// SPDX-License-Identifier: MIT
//
// 16550 UART / Modem Control Register

use crate::drivers::io_access::PortOffset;
use crate::drivers::serial::uart16550::IOAccess;

const MCR_OFFSET: u16 = 4;

pub struct Value {
    value: u8,
}

impl Value {
    pub fn dtr_rts() -> Value {
        Value { value: 0x3 }
    }
}

#[inline]
pub fn write<OT, IO>(io: &IO, val: Value)
where
    OT: From<u16>,
    IO: IOAccess<OT> + 'static,
{
    io.write_u8(PortOffset::new(MCR_OFFSET.into()), val.value)
}
