// SPDX-License-Identifier: MIT

pub struct TaggedType<T, M> {
    v: T,
    _marker: core::marker::PhantomData<M>,
}

impl<T, M> TaggedType<T, M> {
    pub const fn new(v: T) -> Self {
        TaggedType {
            v,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn inner(&self) -> &T {
        &self.v
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.v
    }
}

impl<T, M> Clone for TaggedType<T, M>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.v.clone())
    }
}

impl<T, M> Copy for TaggedType<T, M> where T: Copy {}

impl<T, M> core::fmt::Debug for TaggedType<T, M>
where
    T: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.v.fmt(f)
    }
}

impl<T, M> core::fmt::Display for TaggedType<T, M>
where
    T: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.v.fmt(f)
    }
}
