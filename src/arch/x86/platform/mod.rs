// SPDX-License-Identifier: MIT

use crate::arch::x86::io;
use crate::drivers::serial;

pub mod vga;

static COM1_IO_ACCESS: io::IOAccess = io::IOAccess::new(io::Port::new(0x3f8));

pub type COMPort = serial::uart16550::Uart16550<u16, io::IOAccess>;

pub fn boot_com1() -> COMPort {
    serial::uart16550::boot_new(&COM1_IO_ACCESS, serial::BaudRate::Rate9600)
}
