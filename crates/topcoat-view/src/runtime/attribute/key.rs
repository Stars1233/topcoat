use crate::runtime::{Unescaped, ViewParts};

/// Converts a value used as an attribute key into view parts.
///
/// Implement this for custom dynamic attribute-name values accepted by `view!`.
pub trait AttributeKeyViewParts {
    /// Appends this attribute key to `parts`.
    fn into_view_parts(self, parts: &mut ViewParts);
}

macro_rules! impl_primitive {
    ($ty:ty) => {
        impl AttributeKeyViewParts for $ty {
            #[inline]
            fn into_view_parts(self, parts: &mut ViewParts) {
                parts.push(self);
            }
        }
    };
}

impl_primitive!(&'static str);
impl_primitive!(String);
impl_primitive!(Unescaped<&'static str>);
impl_primitive!(Unescaped<String>);
