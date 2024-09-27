// SPDX-License-Identifier: MIT

static mut DIAG: Option<&'static mut dyn core::fmt::Write> = None;
static mut DIAG_DUMMY: DummyWriter = DummyWriter::new();

pub fn diag() -> &'static mut dyn core::fmt::Write {
    if let Some(v) = unsafe { &mut crate::diag::DIAG } {
        v
    } else {
        unsafe { &mut DIAG_DUMMY }
    }
}

#[macro_export]
macro_rules! kprintln {
    ($($arg:tt)*) => {
        {
            writeln!(crate::diag::diag(), $($arg)*).unwrap();
        }
    };
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {
        {
            write!(crate::diag::diag(), $($arg)*).unwrap();
        }
    };
}

pub unsafe fn init(v: &'static mut dyn core::fmt::Write) {
    DIAG = Some(v)
}

struct DummyWriter {}

impl DummyWriter {
    const fn new() -> Self {
        DummyWriter {}
    }
}

impl core::fmt::Write for DummyWriter {
    fn write_str(&mut self, _: &str) -> core::fmt::Result {
        Ok(())
    }
}
