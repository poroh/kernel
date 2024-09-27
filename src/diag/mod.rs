// SPDX-License-Identifier: MIT

pub static mut DIAG: Option<&'static mut dyn core::fmt::Write> = None;

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write; // Bring the Write trait into scope
            #[allow(static_mut_refs)]
            if let Some(v) = unsafe { &mut crate::diag::DIAG } {
                writeln!(v, $($arg)*).unwrap();
            }
        }
    };
}

pub unsafe fn init(v: &'static mut dyn core::fmt::Write) {
    DIAG = Some(v)
}
