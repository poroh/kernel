// SPDX-License-Identifier: MIT

use crate::common::TaggedType;
use crate::drivers::io_access;
use core::arch::asm;

pub type Port = TaggedType<u16, PortTag>;
pub enum PortTag {}

pub struct IOAccess {
    base: Port,
}

impl IOAccess {
    pub const fn new(base: Port) -> Self {
        Self { base }
    }
}

impl io_access::IOAccess<u16> for IOAccess {
    #[inline]
    fn write_u8(&self, offset: io_access::PortOffset<u16>, value: u8) {
        let target: u16 = self.base.inner() + offset.inner();
        unsafe {
            asm!("outb %al, %dx", in("al") value, in("dx") target, options(att_syntax));
        }
    }

    #[inline]
    fn read_u8(&self, offset: io_access::PortOffset<u16>) -> u8 {
        let target: u16 = self.base.inner() + offset.inner();
        let ret: u8;
        unsafe {
            asm!("inb %dx, %al", in("dx")
                 target, out("al") ret, options(att_syntax))
        };
        ret
    }
}
