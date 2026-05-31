use topcoat_view::runtime::{AttributeKeyViewParts, AttributeViewParts, ViewPart};

use crate::runtime::{Event, Expr};

/// An event handler attribute. Emits a JavaScript closure expression into a
/// `data-topcoat-on:<event>` attribute on the element. The browser scanner
/// wraps it in `new Function('__context', …)` to obtain a real handler.
pub struct EventHandler<K, F> {
    key: K,
    value: Expr<F>,
}

impl<K, F> EventHandler<K, F>
where
    F: Fn(Event),
{
    #[inline]
    pub fn new(key: K, value: Expr<F>) -> Self {
        Self { key, value }
    }
}

impl<K, F> AttributeViewParts for EventHandler<K, F>
where
    K: AttributeKeyViewParts,
{
    #[inline]
    fn into_view_parts(self) -> impl Iterator<Item = ViewPart> {
        iter::Iter::new(self.key.into_view_parts(), self.value.js)
    }
}

mod iter {
    use topcoat_view::runtime::{Unescaped, ViewPart};

    pub struct Iter<K> {
        key: K,
        js: Option<ViewPart>,
        state: State,
    }

    enum State {
        LeadingPrefix,
        Key,
        Equals,
        Js,
        TrailingClose,
        Done,
    }

    impl<K> Iter<K> {
        #[inline]
        pub(super) fn new(key: K, js: ViewPart) -> Self {
            Self {
                key,
                js: Some(js),
                state: State::LeadingPrefix,
            }
        }
    }

    impl<K> Iterator for Iter<K>
    where
        K: Iterator<Item = ViewPart>,
    {
        type Item = ViewPart;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                match self.state {
                    State::LeadingPrefix => {
                        self.state = State::Key;
                        return Some(Unescaped::new_unchecked(" data-topcoat-on:").into());
                    }
                    State::Key => match self.key.next() {
                        Some(part) => return Some(part),
                        None => self.state = State::Equals,
                    },
                    State::Equals => {
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
