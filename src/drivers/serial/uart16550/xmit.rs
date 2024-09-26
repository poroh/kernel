// SPDX-License-Identifier: MIT
//
// 16550 UART / Transmitter Holding Buffer/Receiver Buffer

use crate::drivers::io_access::PortOffset;
use crate::drivers::serial::uart16550::IOAccess;

const THR_OFFSET: u16 = 0;
const RBR_OFFSET: u16 = 0;

#[inline]
pub fn write<OT, IO>(io: &IO, val: u8)
where
    OT: From<u16>,
    IO: IOAccess<OT> + 'static,
{
    io.write_u8(PortOffset::new(THR_OFFSET.into()), val)
}

#[inline]
pub fn read<OT, IO>(io: &IO) -> u8
where
    OT: From<u16>,
    IO: IOAccess<OT> + 'static,
{
    io.read_u8(PortOffset::new(RBR_OFFSET.into()))
}
