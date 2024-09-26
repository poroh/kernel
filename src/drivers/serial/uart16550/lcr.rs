// SPDX-License-Identifier: MIT
//
// 16550 UART / Line Control Register

use crate::drivers::io_access::PortOffset;
use crate::drivers::serial::uart16550::IOAccess;

const LCR_OFFSET: u16 = 3;
const DIVISOR_LATCH_BIT: u8 = 0x80;

pub struct Value {
    value: u8,
}

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
pub fn write<OT, IO>(io: &IO, val: Value)
where
    OT: From<u16>,
    IO: IOAccess<OT> + 'static,
{
    io.write_u8(PortOffset::new(LCR_OFFSET.into()), val.value)
}

#[inline]
pub fn read<OT, IO>(io: &IO) -> Value
where
    OT: From<u16>,
    IO: IOAccess<OT> + 'static,
{
    Value {
        value: io.read_u8(PortOffset::new(LCR_OFFSET.into())),
    }
}
