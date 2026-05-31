use crate::runtime::{Unescaped, View, ViewPart};
use core::iter::once;

pub trait NodeViewParts {
    fn into_view_parts(self) -> impl Iterator<Item = ViewPart>;
}

impl NodeViewParts for View {
    #[inline]
    fn into_view_parts(self) -> impl Iterator<Item = ViewPart> {
        once(self.into())
    }
}

macro_rules! impl_primitive {
    ($ty:ty) => {
        impl NodeViewParts for $ty {
            #[inline]
            fn into_view_parts(self) -> impl Iterator<Item = ViewPart> {
                once(self.into())
            }
        }
    };
}

impl_primitive!(bool);
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

impl<T> NodeViewParts for Option<T>
where
    T: NodeViewParts,
{
    #[inline]
    fn into_view_parts(self) -> impl Iterator<Item = ViewPart> {
        self.into_iter().flat_map(NodeViewParts::into_view_parts)
    }
}

impl<T> NodeViewParts for Vec<T>
where
    T: NodeViewParts,
{
    #[inline]
    fn into_view_parts(self) -> impl Iterator<Item = ViewPart> {
        self.into_iter().flat_map(NodeViewParts::into_view_parts)
    }
}

impl<T> NodeViewParts for &T
where
    T: NodeViewParts + Copy,
{
    #[inline]
    fn into_view_parts(self) -> impl Iterator<Item = ViewPart> {
        (*self).into_view_parts()
    }
}
