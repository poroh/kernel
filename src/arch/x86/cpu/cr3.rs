// SPDX-License-Identifier: MIT

use core::arch::asm;

pub struct Value {
    v: u64,
}

impl core::fmt::Debug for Value {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{:x}: ", self.v)?;
        write!(f, "PML4 address: 0x{:x}", self.v)?;
        Ok(())
    }
}

#[inline]
pub fn read() -> Value {
    Value {
        v: unsafe {
            let val: u64;
            asm!("mov %cr3, %rdx", out("rdx") val, options(att_syntax));
            val
        },
    }
}
