use std::{
    any::{Any, TypeId},
    hash::Hash,
    marker::PhantomData,
    ops::Deref,
    sync::{Arc, Mutex},
};

use hashbrown::{Equivalent, HashMap};
use tokio::sync::OnceCell;

pub struct Memoized<'a, T> {
    inner: Arc<T>,
    // We artificially limit the lifetime of a memoized value to be the lifetime of the request
    // context. This is because the `Arc` is an implementation detail of the cache. The user should
    // not be able to hold on to the memoized value as long as they want. Conceptually, the cache
    // only lasts as long as the request context. The implementation might change to be more
    // efficient in the future.
    lifetime: PhantomData<&'a ()>,
}

impl<'a, T> Memoized<'a, T> {
    fn new(inner: Arc<T>) -> Self {
        Self {
            inner,
            lifetime: PhantomData,
        }
    }
}

impl<'a, T> Deref for Memoized<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[doc(hidden)]
pub struct MemoizeCache {
    // TODO: HashDoS
    entries: Mutex<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
}

impl MemoizeCache {
    pub(super) fn new() -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
        }
    }

    pub fn memoize<'a, Q, K, V, F>(&'a self, borrowed_key: Q, key: K, f: F) -> Memoized<'a, V>
    where
        Q: Copy,
        MemoizeKey<Q>: Hash + ToOwnedKey + Equivalent<<MemoizeKey<Q> as ToOwnedKey>::Owned>,
        <MemoizeKey<Q> as ToOwnedKey>::Owned: Hash + Eq + Send + Sync + 'static,
        V: Send + Sync + 'static,
        F: (FnOnce(K) -> V) + 'static,
    {
        let mut guard = self.entries.lock().unwrap();
        let cache = guard.entry(TypeId::of::<F>()).or_insert_with(|| {
            Box::new(HashMap::<<MemoizeKey<Q> as ToOwnedKey>::Owned, Arc<V>>::new())
        });
        let cache = cache
            .downcast_mut::<HashMap<<MemoizeKey<Q> as ToOwnedKey>::Owned, Arc<V>>>()
            .unwrap();

        if let Some(value) = cache.get(&MemoizeKey(borrowed_key)) {
            Memoized::new(value.clone())
        } else {
            let key_owned = MemoizeKey(borrowed_key).to_owned_key();
            let value = Arc::new(f(key));
            cache.insert(key_owned, value.clone());
            Memoized::new(value)
        }
    }

    // pub async fn memoize_async<'a, K, V, F, Fut>(&'a self, key: K, f: F) -> Memoized<'a, V>
    // where
    //     K: MemoizeKey,
    //     <K as MemoizeKey>::Owned: Borrow<K>,
    //     V: Send + Sync + 'static,
    //     F: (FnOnce(K) -> Fut) + 'static,
    //     Fut: Future<Output = V>,
    // {
    //     let cell = {
    //         let mut guard = self.entries.lock().unwrap();
    //         let cache = guard.entry(TypeId::of::<F>()).or_insert_with(|| {
    //             Box::new(HashMap::<<K as MemoizeKey>::Owned, Arc<OnceCell<Arc<V>>>>::new())
    //         });
    //         let cache = cache
    //             .downcast_mut::<HashMap<<K as MemoizeKey>::Owned, Arc<OnceCell<Arc<V>>>>>()
    //             .unwrap();
    //
    //         if let Some(cell) = cache.get(&key) {
    //             cell.clone()
    //         } else {
    //             let cell = Arc::new(OnceCell::new());
    //             let key_owned = key.to_owned_key();
    //             cache.insert(key_owned, cell.clone());
    //             cell
    //         }
    //     };
    //
    //     let value = cell.get_or_init(|| async { Arc::new(f(key).await) }).await;
    //     Memoized::new(value.clone())
    // }
}

impl std::fmt::Debug for MemoizeCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoizeCache").finish()
    }
}
//
// pub trait MemoizeKey {
//     type Owned;
//
//     fn to_owned_key(&self) -> Self::Owned;
//     fn equivalent(&self, key: &Self::Owned);
// }
//
// struct Key<T>(T);
//
// impl<T: MemoizeKey> Equivalent<T::Owned> for Key<T> {
//     fn equivalent(&self, key: &T::Owned) -> bool {
//         MemoizeKey::equivalent(&self, key)
//     }
// }

#[derive(Hash)]
pub struct MemoizeKey<T>(T);

pub trait ToOwnedKey {
    type Owned;
    fn to_owned_key(&self) -> Self::Owned;
}

macro_rules! impl_equivalent_tuple {
    ($(($kty:ident, $qty:ident, $accessor:tt)),*) => {
        impl<$($kty, $qty),*> Equivalent<($($kty,)*)> for MemoizeKey<($(&$qty,)*)>
        where
            $(
                $qty: ?Sized + Equivalent<$kty>,
            )*
        {
            fn equivalent(&self, key: &($($kty,)*)) -> bool {
                $(self.0.$accessor.equivalent(&key.$accessor))&&*
            }
        }

        impl<$($qty),*> super::ToOwnedKey for MemoizeKey<($(&$qty,)*)>
        where
            $($qty: ?Sized + ToOwned,)*
        {
            type Owned = ($($qty::Owned,)*);
            fn to_owned_key(&self) -> Self::Owned {
                ($(self.0.$accessor.to_owned(),)*)
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::{Equivalent, MemoizeKey};

    impl_equivalent_tuple!((K1, Q1, 0));
    impl_equivalent_tuple!((K1, Q1, 0), (K2, Q2, 1));
    impl_equivalent_tuple!((K1, Q1, 0), (K2, Q2, 1), (K3, Q3, 2));
    impl_equivalent_tuple!((K1, Q1, 0), (K2, Q2, 1), (K3, Q3, 2), (K4, Q4, 3));
    impl_equivalent_tuple!((K1, Q1, 0), (K2, Q2, 1), (K3, Q3, 2), (K4, Q4, 3), (K5, Q5, 4));
    impl_equivalent_tuple!((K1, Q1, 0), (K2, Q2, 1), (K3, Q3, 2), (K4, Q4, 3), (K5, Q5, 4), (K6, Q6, 5));
    impl_equivalent_tuple!((K1, Q1, 0), (K2, Q2, 1), (K3, Q3, 2), (K4, Q4, 3), (K5, Q5, 4), (K6, Q6, 5), (K7, Q7, 6));
    impl_equivalent_tuple!((K1, Q1, 0), (K2, Q2, 1), (K3, Q3, 2), (K4, Q4, 3), (K5, Q5, 4), (K6, Q6, 5), (K7, Q7, 6), (K8, Q8, 7));
    impl_equivalent_tuple!((K1, Q1, 0), (K2, Q2, 1), (K3, Q3, 2), (K4, Q4, 3), (K5, Q5, 4), (K6, Q6, 5), (K7, Q7, 6), (K8, Q8, 7), (K9, Q9, 8));
    impl_equivalent_tuple!((K1, Q1, 0), (K2, Q2, 1), (K3, Q3, 2), (K4, Q4, 3), (K5, Q5, 4), (K6, Q6, 5), (K7, Q7, 6), (K8, Q8, 7), (K9, Q9, 8), (K10, Q10, 9));
    impl_equivalent_tuple!((K1, Q1, 0), (K2, Q2, 1), (K3, Q3, 2), (K4, Q4, 3), (K5, Q5, 4), (K6, Q6, 5), (K7, Q7, 6), (K8, Q8, 7), (K9, Q9, 8), (K10, Q10, 9), (K11, Q11, 10));
    impl_equivalent_tuple!((K1, Q1, 0), (K2, Q2, 1), (K3, Q3, 2), (K4, Q4, 3), (K5, Q5, 4), (K6, Q6, 5), (K7, Q7, 6), (K8, Q8, 7), (K9, Q9, 8), (K10, Q10, 9), (K11, Q11, 10), (K12, Q12, 11));
}

// pub trait MemoizeKey: Eq + Hash {
//     type Owned: Eq + Hash + Send + Sync + 'static;
//
//     fn to_owned_key(&self) -> Self::Owned;
// }
//
// macro_rules! impl_memoize_key_tuple {
//     ($(($ty:ident, $accessor:tt)),*) => {
//         impl<$($ty),*> crate::context::MemoizeKey for ($($ty,)*)
//         where
//             $(
//                 $ty: ToOwned + Eq + std::hash::Hash,
//                 <$ty as ToOwned>::Owned: Eq + std::hash::Hash + Send + Sync + 'static,
//             )*
//         {
//             type Owned = ($($ty::Owned,)*);
//
//             fn to_owned_key(&self) -> Self::Owned {
//                 ($(self.$accessor.to_owned(),)*)
//             }
//         }
//     };
// }
//
// #[rustfmt::skip]
// mod impls {
//     impl_memoize_key_tuple!((T1, 0));
//     impl_memoize_key_tuple!((T1, 0), (T2, 1));
//     impl_memoize_key_tuple!((T1, 0), (T2, 1), (T3, 2));
//     impl_memoize_key_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3));
//     impl_memoize_key_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4));
//     impl_memoize_key_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5));
//     impl_memoize_key_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6));
//     impl_memoize_key_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7));
//     impl_memoize_key_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7), (T9, 8));
//     impl_memoize_key_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7), (T9, 8), (T10, 9));
//     impl_memoize_key_tuple!((T0, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7), (T9, 8), (T10, 9), (T11, 10));
//     impl_memoize_key_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7), (T9, 8), (T10, 9), (T11, 10), (T12, 11));
// }
