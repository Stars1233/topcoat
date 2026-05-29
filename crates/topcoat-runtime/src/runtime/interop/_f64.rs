use std::fmt::Write;
use topcoat_view::runtime::IntoViewParts;

use crate::runtime::Interop;

#[allow(non_camel_case_types)]
pub struct f64(core::primitive::f64);

impl Interop for core::primitive::f64 {
    type Surrogate = f64;

    fn to_js(&self, out: &mut String) {
        write!(out, "__context.builtin.f64({self})").unwrap();
    }

    fn into_surrogate(self) -> Self::Surrogate {
        f64(self)
    }
}

impl IntoViewParts for f64 {
    fn into_view_parts(self) -> impl Iterator<Item = topcoat_view::runtime::ViewPart> {
        self.0.into_view_parts()
    }
}

macro_rules! impl_binary_op {
    ($trait:ident, $method:ident, $op:tt) => {
        impl core::ops::$trait for f64 {
            type Output = f64;

            #[inline]
            fn $method(self, rhs: f64) -> f64 {
                f64(self.0 $op rhs.0)
            }
        }
    };
}

impl_binary_op!(Add, add, +);
impl_binary_op!(Sub, sub, -);
impl_binary_op!(Mul, mul, *);
impl_binary_op!(Div, div, /);
