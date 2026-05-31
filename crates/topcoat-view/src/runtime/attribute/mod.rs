mod attributes;
mod key;
mod value;

pub use attributes::*;
pub use key::*;
pub use value::*;

use crate::runtime::ViewPart;

#[derive(Debug, Clone)]
pub struct Attribute<K, V> {
    key: K,
    value: V,
}

impl<K, V> Attribute<K, V> {
    #[inline]
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

pub trait AttributeViewParts {
    fn into_view_parts(self) -> impl Iterator<Item = ViewPart>;
}

impl<K, V> AttributeViewParts for Attribute<K, V>
where
    K: AttributeKeyViewParts,
    V: AttributeValueViewParts,
{
    #[inline]
    fn into_view_parts(self) -> impl Iterator<Item = ViewPart> {
        if self.value.attribute_present() {
            iter::Iter::new(self.key.into_view_parts(), self.value.into_view_parts())
        } else {
            iter::Iter::empty()
        }
    }
}

impl<T> AttributeViewParts for Option<T>
where
    T: AttributeViewParts,
{
    #[inline]
    fn into_view_parts(self) -> impl Iterator<Item = ViewPart> {
        self.into_iter()
            .flat_map(AttributeViewParts::into_view_parts)
    }
}

impl<T> AttributeViewParts for Vec<T>
where
    T: AttributeViewParts,
{
    #[inline]
    fn into_view_parts(self) -> impl Iterator<Item = ViewPart> {
        self.into_iter()
            .flat_map(AttributeViewParts::into_view_parts)
    }
}

mod iter {
    use crate::runtime::{Unescaped, ViewPart};

    pub struct Iter<K, V> {
        inner: Option<Inner<K, V>>,
    }

    struct Inner<K, V> {
        key: K,
        value: V,
        state: State,
    }

    enum State {
        LeadingSpace,
        Key,
        Equals,
        Value,
        Closing,
        Done,
    }

    impl<K, V> Iter<K, V> {
        #[inline]
        pub(super) fn new(key: K, value: V) -> Self {
            Self {
                inner: Some(Inner {
                    key,
                    value,
                    state: State::LeadingSpace,
                }),
            }
        }

        #[inline]
        pub(super) fn empty() -> Self {
            Self { inner: None }
        }
    }

    impl<K, V> Iterator for Iter<K, V>
    where
        K: Iterator<Item = ViewPart>,
        V: Iterator<Item = ViewPart>,
    {
        type Item = ViewPart;

        fn next(&mut self) -> Option<Self::Item> {
            let inner = self.inner.as_mut()?;
            loop {
                match inner.state {
                    State::LeadingSpace => {
                        inner.state = State::Key;
                        return Some(Unescaped::new_unchecked(" ").into());
                    }
                    State::Key => match inner.key.next() {
                        Some(part) => return Some(part),
                        None => inner.state = State::Equals,
                    },
                    State::Equals => {
                        inner.state = State::Value;
                        return Some(Unescaped::new_unchecked("=\"").into());
                    }
                    State::Value => match inner.value.next() {
                        Some(part) => return Some(part),
                        None => inner.state = State::Closing,
                    },
                    State::Closing => {
                        inner.state = State::Done;
                        return Some(Unescaped::new_unchecked("\"").into());
                    }
                    State::Done => {
                        self.inner = None;
                        return None;
                    }
                }
            }
        }
    }
}
