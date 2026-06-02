use std::fmt::Write;

use ref_cast::RefCast;

use crate::runtime::{ToJs, impl_surrogate_mut, impl_surrogate_ref};

#[derive(Debug, RefCast)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Str(str);

impl_surrogate_ref!(str, Str);
impl_surrogate_mut!(str, Str);

impl ToJs for Str {
    fn to_js(&self, out: &mut String) {
        let inner = &self.0;
        let _ = write!(out, "cx.s.str({inner:?})");
    }
}

impl std::fmt::Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
