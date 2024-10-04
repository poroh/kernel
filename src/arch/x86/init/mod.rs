// SPDX-License-Identifier: MIT

use crate::arch::x86;
use crate::drivers::serial::Device;
use crate::drivers::serial::FmtWritable;
use crate::kprintln;

static mut COM1: Option<x86::platform::COMPort> = None;
static mut COM1_FORMATTABLE: Option<FmtWritable<x86::platform::COMPort>> = None;

core::arch::global_asm!(include_str!("pvh_start.s"));

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[allow(static_mut_refs)]
    unsafe {
        COM1 = Some(x86::platform::boot_com1());
        COM1_FORMATTABLE = Some(COM1.as_ref().unwrap().formattable());
        crate::diag::init(COM1_FORMATTABLE.as_mut().unwrap());
    };

    kprintln!("");
    kprintln!("Hello kernel x86");
    kprintln!("CR0: {:?}", x86::cpu::cr0::read());
    kprintln!("CR3: {:?}", x86::cpu::cr3::read());
    crate::kernel_main();
}
