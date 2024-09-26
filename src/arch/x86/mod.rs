// SPDX-License-Identifier: MIT

pub mod boot;
pub mod io;
pub mod platform;

use core::arch::asm;

#[inline]
pub fn cpu_relax() {
    unsafe { asm!("rep; nop") };
}
