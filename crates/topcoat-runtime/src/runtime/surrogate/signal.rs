use std::fmt::Write;

use ref_cast::RefCast;

use crate::runtime::{
    Signal, Surrogated, ToJs, impl_surrogate, impl_surrogate_mut, impl_surrogate_ref,
};

#[derive(RefCast, Clone, Copy)]
#[repr(transparent)]
pub struct WriteSignal<'a, T>(Signal<'a, T>);

impl<'a, T> WriteSignal<'a, T> {
    #[inline]
    pub(crate) const fn new(v: Signal<'a, T>) -> Self {
        Self(v)
    }
}

impl<'a, T> WriteSignal<'a, T>
where
    T: Surrogated,
    for<'b> &'b T: Surrogated,
{
    pub fn read(&self) -> <&T as Surrogated>::Surrogate {
        self.0.read().into_surrogate()
    }

    pub fn set(&self, _v: T::Surrogate) {
        panic!("expressions in which a signal is written to cannot be run server-side");
    }
}

impl_surrogate!({'a, T} Signal<'a, T>, WriteSignal<'a, T>);
impl_surrogate_ref!({'a, T} Signal<'a, T>, WriteSignal<'a, T>);
impl_surrogate_mut!({'a, T} Signal<'a, T>, WriteSignal<'a, T>);

impl<'a, T> ToJs for WriteSignal<'a, T> {
    fn to_js(&self, out: &mut String) {
        let id = self.0.id();
        let _ = write!(out, "cx.signal(\"{id}\")");
    }
}
