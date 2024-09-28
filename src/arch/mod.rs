// SPDX-License-Identifier: MIT

pub mod x86;

pub fn init() {
    x86::init::init();
}

pub fn halt() -> ! {
    loop {
        x86::cpu::hlt();
    }
}
