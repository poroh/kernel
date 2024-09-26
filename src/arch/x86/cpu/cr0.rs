// SPDX-License-Identifier: MIT

use crate::common::TaggedType;
use core::arch::asm;

pub struct Value {
    v: u64,
}

pub type ProtectedModeFlag = TaggedType<bool, ProtectedModeFlagTag>;
pub enum ProtectedModeFlagTag {}

impl Value {
    pub fn pe(&self) -> ProtectedModeFlag {
        ProtectedModeFlag::new((self.v & 0x1) != 0)
    }
}

impl core::fmt::Debug for Value {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{:X}: ", self.v)?;
        if self.pe().into_inner() {
            write!(f, "protected mode")?;
        } else {
            write!(f, "real mode")?;
        }
        Ok(())
    }
}

#[inline]
pub fn read() -> Value {
    Value {
        v: unsafe {
            let val: u64;
            asm!("mov %cr0, %rdx", out("rdx") val, options(att_syntax));
            val
        },
    }
}
