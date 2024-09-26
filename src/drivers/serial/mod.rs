// SPDX-License-Identifier: MIT

pub mod uart16550;

use crate::cpu;
use core::result::Result;

#[repr(u8)]
pub enum BaudRate {
    Rate9600,
    // other not implemented yet
}

pub enum Error {}

pub trait Device {
    fn poll_read<'a>(&'a self) -> Option<impl Readable>;
    fn poll_write<'a>(&'a self) -> Option<impl Writable>;

    fn write_string(&self, s: &str) -> Result<(), Error> {
        for byte in s.bytes() {
            loop {
                match self.poll_write() {
                    None => (),
                    Some(w) => {
                        w.write(byte)?;
                        break;
                    }
                }
                cpu::relax();
            }
        }
        Ok(())
    }

    fn formattable(&self) -> FmtWritable<'_, Self>
    where
        Self: Sized,
    {
        FmtWritable(&self)
    }
}

pub trait Readable {
    fn read(self) -> Result<u8, Error>;
}

pub trait Writable {
    fn write(self, _: u8) -> Result<(), Error>;
}

pub struct FmtWritable<'a, T>(&'a T)
where
    T: Device;

impl<T> core::fmt::Write for FmtWritable<'_, T>
where
    T: Device,
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0.write_string(s);
        Ok(())
    }
}
