// SPDX-License-Identifier: MIT

use crate::arch::x86;
use crate::drivers::serial::Device;
use crate::drivers::serial::FmtWritable;
use crate::kprintln;

static mut COM1: Option<x86::platform::COMPort> = None;
static mut COM1_FORMATTABLE: Option<FmtWritable<x86::platform::COMPort>> = None;

pub fn init() {
    #[allow(static_mut_refs)]
    unsafe {
        COM1 = Some(x86::platform::boot_com1());
        COM1_FORMATTABLE = Some(COM1.as_ref().unwrap().formattable());
        crate::diag::init(COM1_FORMATTABLE.as_mut().unwrap());
    };

    kprintln!("Hello kernel");
    kprintln!("CR0: {:?}", x86::cpu::cr0::read());
    kprintln!("CR3: {:?}", x86::cpu::cr3::read());
}
