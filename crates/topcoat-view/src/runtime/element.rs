use crate::runtime::{Unescaped, ViewPart};
use core::iter::once;

pub trait ElementNameViewParts {
    fn into_view_parts(self) -> impl Iterator<Item = ViewPart>;
}

macro_rules! impl_primitive {
    ($ty:ty) => {
        impl ElementNameViewParts for $ty {
            #[inline]
            fn into_view_parts(self) -> impl Iterator<Item = ViewPart> {
                once(self.into())
            }
        }
    };
}

impl_primitive!(&'static str);
impl_primitive!(String);
impl_primitive!(Unescaped<&'static str>);
impl_primitive!(Unescaped<String>);
