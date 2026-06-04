use ref_cast::RefCast;
use topcoat_view::runtime::ViewParts;

use crate::runtime::{
    JsViewParts, Signal, Surrogated, impl_surrogate, impl_surrogate_mut, impl_surrogate_ref,
};

#[derive(RefCast)]
#[repr(transparent)]
pub struct WriteSignal<T>(Signal<T>);

impl<T> WriteSignal<T> {
    #[inline]
    pub(crate) const fn new(v: Signal<T>) -> Self {
        Self(v)
    }
}

impl<T> WriteSignal<T>
where
    for<'b> &'b T: Surrogated,
{
    pub fn read(&self) -> <&T as Surrogated>::Surrogate {
        self.0.read().into_surrogate()
    }
}

impl<T> WriteSignal<T>
where
    T: Surrogated + Clone,
{
    pub fn get(&self) -> <T as Surrogated>::Surrogate {
        self.0.get().into_surrogate()
    }
}

impl<T> WriteSignal<T>
where
    T: Surrogated,
{
    pub fn set(&self, _v: T::Surrogate) {
        panic!("expressions in which a signal is written to cannot be run server-side");
    }
}

impl_surrogate!({T} Signal<T>, WriteSignal<T>);
impl_surrogate_ref!({T} Signal<T>, WriteSignal<T>);
impl_surrogate_mut!({T} Signal<T>, WriteSignal<T>);

impl<T> JsViewParts for &WriteSignal<T> {
    fn to_view_parts(&self, parts: &mut ViewParts) {
        parts.push("cx.signal(\"");
        parts.push(self.0.id().to_string());
        parts.push("\")");
    }
}
