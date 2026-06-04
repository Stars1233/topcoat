mod reactive_scope;
mod shard;

pub use reactive_scope::*;
pub use shard::*;

use std::{iter::empty, ops::Deref};

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use topcoat_view::runtime::{NodeViewParts, Unescaped, ViewParts};
use uuid::Uuid;

use crate::runtime::{JsViewParts, Surrogated};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SignalId(Uuid);

impl SignalId {
    #[inline]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SignalId {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SignalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub struct Signal<T> {
    id: SignalId,
    value: T,
}

impl<T> Signal<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            id: SignalId::new(),
            value,
        }
    }

    pub(crate) fn id(&self) -> SignalId {
        self.id
    }

    pub(crate) fn read(&self) -> &T {
        &self.value
    }
}

impl<T> Signal<T>
where
    T: Clone,
{
    pub(crate) fn get(&self) -> T {
        self.value.clone()
    }
}

pub struct SignalDeclaration<'a, T>(&'a Signal<T>);

impl<'a, T> SignalDeclaration<'a, T> {
    #[inline]
    pub fn new(signal: &'a Signal<T>) -> Self {
        Self(signal)
    }
}

impl<T> NodeViewParts for SignalDeclaration<'_, T>
where
    for<'a> &'a T: Surrogated,
    for<'a> <&'a T as Surrogated>::Surrogate: JsViewParts,
{
    fn into_view_parts(self, parts: &mut ViewParts) {
        let id = self.0.id().to_string();
        parts.push(Unescaped::new_unchecked("<!-- ::topcoat::signal("));
        parts.push(Unescaped::new_unchecked(format!("{id:?}")));
        parts.push(Unescaped::new_unchecked(", \""));
        (&self.0.value).into_surrogate().to_view_parts(parts);
        parts.push(Unescaped::new_unchecked("\") -->"));
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReadSignal<T> {
    id: SignalId,
    value: T,
}

impl<T> ReadSignal<T> {
    pub fn new(signal: &Signal<T>) -> Self
    where
        T: Clone,
    {
        Self {
            id: signal.id,
            value: signal.value.clone(),
        }
    }

    pub fn id(&self) -> SignalId {
        self.id
    }
}

impl<T> Deref for ReadSignal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub trait Signals: Sized {
    fn ids(&self) -> impl Iterator<Item = SignalId>;
    fn decode(encoded_signals: EncodedSignals) -> Self;
}

impl Signals for () {
    fn ids(&self) -> impl Iterator<Item = SignalId> {
        empty()
    }

    fn decode(_encoded_signals: EncodedSignals) -> Self {}
}

macro_rules! impl_signals_for_tuple {
    ($($n:tt $t:ident),+) => {
        impl<$($t),+> Signals for ($(ReadSignal<$t>,)+)
        where
            $($t: DeserializeOwned,)+
        {
            fn ids(&self) -> impl Iterator<Item = SignalId> {
                [$(self.$n.id),+].into_iter()
            }

            fn decode(encoded_signals: EncodedSignals) -> Self {
                serde_json::from_str(&encoded_signals.0).unwrap()
            }
        }
    };
}

impl_signals_for_tuple!(0 T0);
impl_signals_for_tuple!(0 T0, 1 T1);
impl_signals_for_tuple!(0 T0, 1 T1, 2 T2);
impl_signals_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3);
impl_signals_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4);
impl_signals_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5);
impl_signals_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6);
impl_signals_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7);
impl_signals_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8);
impl_signals_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9);
impl_signals_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10);
impl_signals_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11);

pub struct EncodedSignals(String);

impl EncodedSignals {
    pub fn new(inner: impl Into<String>) -> Self {
        Self(inner.into())
    }
}
