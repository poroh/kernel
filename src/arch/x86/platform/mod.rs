// SPDX-License-Identifier: MIT

pub mod uart16550;
pub mod vga;

pub fn boot_com1() -> uart16550::Serial {
    uart16550::Serial::boot_new(uart16550::BootConfig {
        port: uart16550::Port::Com1,
        rate: uart16550::BaudRate::Rate9600,
    })
}
