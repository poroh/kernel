// SPDX-License-Identifier: MIT

use crate::arch::x86;
use core::fmt::Write;

pub fn init() {
    let mut serial = x86::platform::boot_com1();
    let _ = serial.write_fmt(format_args!("hello kernel {}", 0xff));
    loop {
        match serial.try_read().and_then(core::ascii::Char::from_u8) {
            Some(c) => serial.put_ascii(c),
            None => (),
        }
    }
}
