// SPDX-License-Identifier: MIT

use core::alloc::{GlobalAlloc, Layout};
use core::ptr;

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        ptr::null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Do nothing for deallocation
    }
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;
