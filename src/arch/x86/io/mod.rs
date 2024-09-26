// SPDX-License-Identifier: MIT

use crate::common::TaggedType;
use core::arch::asm;

pub type Port = TaggedType<u16, PortTag>;
pub enum PortTag {}

#[inline]
pub unsafe fn outb(port: &Port, value: u8) {
    asm!("outb %al, %dx", in("al") value, in("dx") *port.inner(), options(att_syntax));
}

#[inline]
pub unsafe fn inb(port: &Port) -> u8 {
    let ret: u8;
    asm!("inb %dx, %al", in("dx") *
         port.inner(), out("al") ret, options(att_syntax));
    ret
}
