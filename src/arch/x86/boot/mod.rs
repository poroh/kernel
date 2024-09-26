// SPDX-License-Identifier: MIT

use crate::arch::x86;
use crate::arch::x86::platform::uart16550;

pub fn init() {
    let serial = x86::platform::com1();
    serial.boot_init(&uart16550::BaudRate::Rate9600);
    serial.put_ascii(core::ascii::Char::CapitalQ);
    serial.put_ascii(core::ascii::Char::CapitalZ);
    serial.put_ascii(core::ascii::Char::CapitalA);
    loop {
        match serial.try_read().and_then(core::ascii::Char::from_u8) {
            Some(c) => serial.put_ascii(c),
            None => (),
        }
    }
}
