// SPDX-License-Identifier: MIT
//
// 16550 UART / Divisor

use crate::drivers::io_access::PortOffset;
use crate::drivers::serial::uart16550::IOAccess;
use crate::drivers::serial::BaudRate;

const DLL_OFFSET: u16 = 0;
const DLH_OFFSET: u16 = 1;

fn divisor(r: BaudRate) -> u16 {
    match r {
        BaudRate::Rate9600 => 12,
    }
}

#[inline]
pub fn write<OT, IO>(io: &IO, val: BaudRate)
where
    OT: From<u16>,
    IO: IOAccess<OT> + 'static,
{
    let v = divisor(val);
    io.write_u8(PortOffset::new(DLL_OFFSET.into()), (v & 0xFF) as u8);
    io.write_u8(PortOffset::new(DLH_OFFSET.into()), (v >> 8) as u8);
}
