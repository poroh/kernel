// SPDX-License-Identifier: MIT

pub mod uart16550;
pub mod vga;

pub fn com1() -> uart16550::Serial {
    uart16550::Serial::new(uart16550::Port::Com1)
}
