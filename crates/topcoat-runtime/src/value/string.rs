use std::string::String as StdString;

use ref_cast::{RefCastCustom, ref_cast_custom};

use crate::{JsCallable, Value};

/// A string carried through `expr!` reactive expressions. The inherent method
/// surface is intentionally narrow — only methods that also have a JS
/// equivalent live here, so calls Rust can't lower to JS fail at compile time
/// instead of panicking in the browser.
#[derive(RefCastCustom)]
#[repr(transparent)]
pub struct String {
    inner: StdString,
}

impl String {
    pub(crate) fn new(inner: StdString) -> Self {
        Self { inner }
    }

    #[ref_cast_custom]
    fn ref_cast(s: &StdString) -> &Self;

    #[allow(clippy::should_implement_trait)]
    pub fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl From<&String> for StdString {
    fn from(value: &String) -> Self {
        value.inner.clone()
    }
}

impl JsCallable for String {
    fn js_call(method: &str, _out: &mut StdString) {
        match method {
            // Strings are value-typed in JS; `.clone()` is the identity, so
            // we append nothing to the already-emitted receiver.
            "clone" => {}
            _ => unreachable!(),
        }
    }
}

impl Value for StdString {
    type Surrogate = String;

    fn ref_cast(&self) -> &Self::Surrogate {
        Self::Surrogate::ref_cast(self)
    }
}
