// SPDX-License-Identifier: MIT

#[allow(dead_code)]
pub fn write(offset: isize, c: core::ascii::Char) {
    write_byte(offset, c.into());
}

#[allow(dead_code)]
pub fn write_hex(offset: isize, v: u8) {
    write_hex_nibble(offset, v >> 4);
    write_hex_nibble(offset + 1, v & 0xF);
}

#[allow(dead_code)]
pub fn write_byte(offset: isize, c: u8) {
    let vga_buffer = 0xb8000 as *mut u8;
    unsafe {
        *vga_buffer.offset(offset * 2) = c;
        *vga_buffer.offset(offset * 2 + 1) = 0xb;
    }
}

#[allow(dead_code)]
fn write_hex_nibble(offset: isize, v: u8) {
    let v = match v {
        0..=9 => v + '0' as u8,
        10..=15 => v - 10 + 'A' as u8,
        _ => '?' as u8,
    };
    write_byte(offset, v);
}
