// SPDX-License-Identifier: MIT

pub mod cr0;
pub mod cr3;

use core::arch::asm;

#[inline]
pub fn relax() {
    unsafe { asm!("rep; nop") };
}

pub fn hlt() {
    unsafe { asm!("hlt") };
}
