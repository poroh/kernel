// SPDX-License-Identifier: MIT
//
// 16550 UART / Line Status Register

use crate::drivers::io_access::PortOffset;
use crate::drivers::serial::uart16550::IOAccess;

const LSR_OFFSET: u16 = 5;

const XMT_READY_BIT: u8 = 0x20;
const RX_READY_BIT: u8 = 0x01;

pub struct Value {
    value: u8,
}

impl Value {
    pub fn tx_ready(&self) -> bool {
        (self.value & XMT_READY_BIT) != 0
    }
    pub fn rx_ready(&self) -> bool {
        (self.value & RX_READY_BIT) != 0
    }
}

#[inline]
pub fn write<OT, IO>(io: &IO, val: Value)
where
    OT: From<u16>,
    IO: IOAccess<OT> + 'static,
{
    io.write_u8(PortOffset::new(LSR_OFFSET.into()), val.value)
}

#[inline]
pub fn read<OT, IO>(io: &IO) -> Value
where
    OT: From<u16>,
    IO: IOAccess<OT> + 'static,
{
    Value {
        value: io.read_u8(PortOffset::new(LSR_OFFSET.into())),
    }
}
