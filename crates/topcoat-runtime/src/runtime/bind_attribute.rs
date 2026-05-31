use topcoat_view::runtime::{
    AttributeKeyViewParts, AttributeValueViewParts, AttributeViewParts, ViewPart,
};

use crate::runtime::Expr;

#[derive(Debug, Clone)]
pub struct BindAttribute<K, V> {
    key: K,
    value: Expr<V>,
}

impl<K, V> BindAttribute<K, V> {
    #[inline]
    pub fn new(key: K, value: Expr<V>) -> Self {
        Self { key, value }
    }
}

impl<K, V> AttributeViewParts for BindAttribute<K, V>
where
    K: AttributeKeyViewParts + Clone,
    V: AttributeValueViewParts,
{
    #[inline]
    fn into_view_parts(self) -> impl Iterator<Item = ViewPart> {
        iter::Iter::new(
            self.key.clone().into_view_parts(),
            self.key.into_view_parts(),
            self.value.evaluated.into_view_parts(),
            self.value.js,
        )
    }
}

mod iter {
    use topcoat_view::runtime::{Unescaped, ViewPart};

    pub struct Iter<K1, K2, V> {
        key1: K1,
        key2: K2,
        value: V,
        js: Option<ViewPart>,
        state: State,
    }

    enum State {
        LeadingSpace,
        Key1,
        Equals1,
        Value,
        BindSeparator,
        Key2,
        Equals2,
        Js,
        TrailingClose,
        Done,
    }

    impl<K1, K2, V> Iter<K1, K2, V> {
        #[inline]
        pub(super) fn new(key1: K1, key2: K2, value: V, js: ViewPart) -> Self {
            Self {
                key1,
                key2,
                value,
                js: Some(js),
                state: State::LeadingSpace,
            }
        }
    }

    impl<K1, K2, V> Iterator for Iter<K1, K2, V>
    where
        K1: Iterator<Item = ViewPart>,
        K2: Iterator<Item = ViewPart>,
        V: Iterator<Item = ViewPart>,
    {
        type Item = ViewPart;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                match self.state {
                    State::LeadingSpace => {
                        self.state = State::Key1;
                        return Some(Unescaped::new_unchecked(" ").into());
                    }
                    State::Key1 => match self.key1.next() {
                        Some(part) => return Some(part),
                        None => self.state = State::Equals1,
                    },
                    State::Equals1 => {
                        self.state = State::Value;
                        return Some(Unescaped::new_unchecked("=\"").into());
                    }
                    State::Value => match self.value.next() {
                        Some(part) => return Some(part),
                        None => self.state = State::BindSeparator,
                    },
                    State::BindSeparator => {
                        self.state = State::Key2;
                        return Some(Unescaped::new_unchecked("\" data-topcoat-bind:").into());
                    }
                    State::Key2 => match self.key2.next() {
                        Some(part) => return Some(part),
                        None => self.state = State::Equals2,
                    },
                    State::Equals2 => {
                        self.state = State::Js;
                        return Some(Unescaped::new_unchecked("=\"").into());
                    }
                    State::Js => {
                        self.state = State::TrailingClose;
                        return self.js.take();
                    }
                    State::TrailingClose => {
                        self.state = State::Done;
                        return Some(Unescaped::new_unchecked("\" ").into());
                    }
                    State::Done => return None,
                }
            }
        }
    }
}
