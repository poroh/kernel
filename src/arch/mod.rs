// SPDX-License-Identifier: MIT

pub mod x86;

pub fn halt() -> ! {
    loop {
        x86::cpu::hlt();
    }
}
