// SPDX-License-Identifier: MIT

mod divisor;
mod fcr;
mod ier;
mod lcr;
mod lsr;
mod mcr;
mod xmit;

use crate::arch::x86;
use crate::common::TaggedType;

#[allow(dead_code)]
pub enum Port {
    Com1,
    Com2,
}

pub enum BaudRate {
    Rate9600,
    // other not implemented yet
}

impl BaudRate {
    fn divisor(&self) -> u16 {
        match self {
            Self::Rate9600 => 12,
        }
    }
}

pub type UARTPortBase = TaggedType<x86::io::Port, UARTPortBaseTag>;
pub enum UARTPortBaseTag {}

impl UARTPortBase {
    fn thr(&self) -> x86::io::Port {
        x86::io::Port::new(self.inner().inner() + 0)
    }

    fn rbr(&self) -> x86::io::Port {
        x86::io::Port::new(self.inner().inner() + 0)
    }

    fn dll(&self) -> x86::io::Port {
        x86::io::Port::new(self.inner().inner() + 0)
    }

    fn dlh(&self) -> x86::io::Port {
        x86::io::Port::new(self.inner().inner() + 1)
    }

    fn ier(&self) -> x86::io::Port {
        x86::io::Port::new(self.inner().inner() + 1)
    }

    fn fcr(&self) -> x86::io::Port {
        x86::io::Port::new(self.inner().inner() + 2)
    }

    fn lcr(&self) -> x86::io::Port {
        x86::io::Port::new(self.inner().inner() + 3)
    }

    fn mcr(&self) -> x86::io::Port {
        x86::io::Port::new(self.inner().inner() + 4)
    }

    fn lsr(&self) -> x86::io::Port {
        x86::io::Port::new(self.inner().inner() + 5)
    }
}

pub struct Serial {
    port: Port,
}

pub struct BootConfig {
    pub port: Port,
    pub rate: BaudRate,
}

impl Serial {
    pub fn boot_new(config: BootConfig) -> Serial {
        let s = Serial { port: config.port };
        s.boot_init(&config.rate);
        s
    }

    pub fn put_ascii(&self, c: core::ascii::Char) {
        self.poll_write_ready().map(|r| {
            r.write(c.into());
        });
    }

    pub fn try_read(&self) -> Option<u8> {
        if lsr::read(&self.base()).rx_ready() {
            Some(xmit::read(&self.base()))
        } else {
            None
        }
    }

    fn boot_init(&self, rate: &BaudRate) {
        lcr::write(&self.base(), &lcr::Value::mode_8n1());
        ier::write(&self.base(), &ier::Value::no_interrupt());
        fcr::write(&self.base(), &fcr::Value::no_fifo());
        mcr::write(&self.base(), &mcr::Value::dtr_rts());

        self.with_divisor_latched(|dl| dl.set_baud_rate(rate));
    }

    fn with_divisor_latched<F>(&self, f: F)
    where
        F: FnOnce(DivisorLatched),
    {
        let val = lcr::read(&self.base());
        lcr::write(&self.base(), &val.with_divisor_latched());
        f(DivisorLatched(&self));
        lcr::write(&self.base(), &val);
    }

    fn poll_write_ready(&self) -> Option<WriteReady> {
        let mut timeout = 0xFFFF;
        loop {
            if lsr::read(&self.base()).tx_ready() {
                break Some(WriteReady(&self));
            }
            if timeout == 0 {
                break None;
            }
            timeout -= 1;
            x86::cpu::relax();
        }
    }

    fn write_string(&self, s: &str) {
        for byte in s.bytes() {
            self.poll_write_ready().map(|r| {
                r.write(byte);
            });
        }
    }

    fn base(&self) -> UARTPortBase {
        match self.port {
            Port::Com1 => UARTPortBase::new(x86::io::Port::new(0x3f8)),
            Port::Com2 => UARTPortBase::new(x86::io::Port::new(0x2f8)),
        }
    }
}

impl core::fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

struct DivisorLatched<'a>(&'a Serial);

impl DivisorLatched<'_> {
    pub fn set_baud_rate(&self, rate: &BaudRate) {
        divisor::write(&self.0.base(), rate);
    }
}

struct WriteReady<'a>(&'a Serial);

impl WriteReady<'_> {
    fn write(self, v: u8) {
        xmit::write(&self.0.base(), v)
    }
}
