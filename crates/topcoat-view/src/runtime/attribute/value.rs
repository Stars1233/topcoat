use std::iter::once;

use crate::runtime::{Unescaped, ViewPart};

pub trait AttributeValueViewParts {
    fn attribute_present(&self) -> bool;
    fn into_view_parts(self) -> impl Iterator<Item = ViewPart>;
}

macro_rules! impl_primitive {
    ($ty:ty) => {
        impl AttributeValueViewParts for $ty {
            #[inline]
            fn attribute_present(&self) -> bool {
                true
            }

            #[inline]
            fn into_view_parts(self) -> impl Iterator<Item = ViewPart> {
                once(self.into())
            }
        }
    };
}

impl_primitive!(char);
impl_primitive!(i8);
impl_primitive!(i16);
impl_primitive!(i32);
impl_primitive!(i64);
impl_primitive!(i128);
impl_primitive!(isize);
impl_primitive!(u8);
impl_primitive!(u16);
impl_primitive!(u32);
impl_primitive!(u64);
impl_primitive!(u128);
impl_primitive!(usize);
impl_primitive!(f32);
impl_primitive!(f64);
impl_primitive!(&'static str);
impl_primitive!(String);
impl_primitive!(Unescaped<&'static str>);
impl_primitive!(Unescaped<String>);

impl AttributeValueViewParts for bool {
    #[inline]
    fn attribute_present(&self) -> bool {
        *self
    }

    #[inline]
    fn into_view_parts(self) -> impl Iterator<Item = ViewPart> {
        once(self.into())
    }
}

impl<T> AttributeValueViewParts for Option<T>
where
    T: AttributeValueViewParts,
{
    #[inline]
    fn attribute_present(&self) -> bool {
        self.is_some()
    }

    #[inline]
    fn into_view_parts(self) -> impl Iterator<Item = ViewPart> {
        self.into_iter()
            .flat_map(AttributeValueViewParts::into_view_parts)
    }
}

impl<T> AttributeValueViewParts for &T
where
    T: AttributeValueViewParts + Copy,
{
    #[inline]
    fn attribute_present(&self) -> bool {
        (*self).attribute_present()
    }

    #[inline]
    fn into_view_parts(self) -> impl Iterator<Item = ViewPart> {
        (*self).into_view_parts()
    }
}
