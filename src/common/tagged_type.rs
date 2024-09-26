// SPDX-License-Identifier: MIT

pub struct TaggedType<T, M> {
    v: T,
    _marker: core::marker::PhantomData<M>,
}

impl<T, M> TaggedType<T, M> {
    pub fn new(v: T) -> Self {
        TaggedType {
            v,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn inner(&self) -> &T {
        &self.v
    }
}
