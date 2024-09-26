// SPDX-License-Identifier: MIT

use crate::common::TaggedType;

pub type PortOffset<T> = TaggedType<T, PortOffsetTag>;
pub enum PortOffsetTag {}

pub trait IOAccess<OffsetSize> {
    fn write_u8(&self, offset: PortOffset<OffsetSize>, v: u8);
    fn read_u8(&self, offset: PortOffset<OffsetSize>) -> u8;
}
