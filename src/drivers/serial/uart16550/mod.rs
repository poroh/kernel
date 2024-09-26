// SPDX-License-Identifier: MIT

use crate::drivers::io_access::IOAccess;
use crate::drivers::serial;

mod divisor;
mod fcr;
mod ier;
mod lcr;
mod lsr;
mod mcr;
mod xmit;

pub struct Uart16550<OT, IO>
where
    OT: From<u16>,
    IO: IOAccess<OT> + 'static,
{
    io: &'static IO,
    _marker: core::marker::PhantomData<OT>,
}

pub enum Error {}

pub fn boot_new<OT, IO>(io: &'static IO, rate: serial::BaudRate) -> Uart16550<OT, IO>
where
    OT: From<u16>,
    IO: IOAccess<OT> + 'static,
{
    let v = Uart16550 {
        io,
        _marker: core::marker::PhantomData::default(),
    };
    v.boot_init(rate);
    v
}

impl<OT, IO> Uart16550<OT, IO>
where
    OT: From<u16>,
    IO: IOAccess<OT> + 'static,
{
    fn boot_init(&self, rate: serial::BaudRate) {
        lcr::write(self.io, lcr::Value::mode_8n1());
        ier::write(self.io, ier::Value::no_interrupt());
        fcr::write(self.io, fcr::Value::no_fifo());
        mcr::write(self.io, mcr::Value::dtr_rts());

        self.with_divisor_latched(|dl| dl.set_baud_rate(rate));
    }

    fn with_divisor_latched<F>(&self, f: F)
    where
        F: FnOnce(DivisorLatched<'_, OT, IO>),
    {
        let val = lcr::read(self.io);
        lcr::write(self.io, val.with_divisor_latched());
        f(DivisorLatched(&self));
        lcr::write(self.io, val);
    }
}

impl<OT, IO> serial::Device for Uart16550<OT, IO>
where
    OT: From<u16>,
    IO: IOAccess<OT> + 'static,
{
    fn poll_read(&self) -> Option<impl serial::Readable> {
        if lsr::read(self.io).rx_ready() {
            Some(ReadableUart16550(&self))
        } else {
            None
        }
    }
    fn poll_write(&self) -> Option<impl serial::Writable> {
        if lsr::read(self.io).tx_ready() {
            Some(WritableUart16550(&self))
        } else {
            None
        }
    }
}

struct ReadableUart16550<'a, OT, IO>(&'a Uart16550<OT, IO>)
where
    OT: From<u16>,
    IO: IOAccess<OT> + 'static;

impl<OT, IO> serial::Readable for ReadableUart16550<'_, OT, IO>
where
    OT: From<u16>,
    IO: IOAccess<OT> + 'static,
{
    fn read(self) -> Result<u8, serial::Error> {
        Ok(xmit::read(self.0.io))
    }
}

struct WritableUart16550<'a, OT, IO>(&'a Uart16550<OT, IO>)
where
    OT: From<u16>,
    IO: IOAccess<OT> + 'static;

impl<OT, IO> serial::Writable for WritableUart16550<'_, OT, IO>
where
    OT: From<u16>,
    IO: IOAccess<OT> + 'static,
{
    fn write(self, val: u8) -> Result<(), serial::Error> {
        xmit::write(self.0.io, val);
        Ok(())
    }
}

struct DivisorLatched<'a, OT, IO>(&'a Uart16550<OT, IO>)
where
    OT: From<u16>,
    IO: IOAccess<OT> + 'static;

impl<OT, IO> DivisorLatched<'_, OT, IO>
where
    OT: From<u16>,
    IO: IOAccess<OT> + 'static,
{
    pub fn set_baud_rate(&self, rate: serial::BaudRate) {
        divisor::write(self.0.io, rate);
    }
}
