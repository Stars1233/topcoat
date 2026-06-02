use std::{ops::Deref, rc::Rc, sync::Arc};

use topcoat_core::context::Cx;

use crate::runtime::Formatter;

/// A value that can render itself into a [`Formatter`].
///
/// Implement this for custom renderable types. Use the escaped formatter
/// methods for dynamic or user-provided text, and the unescaped methods only
/// for trusted markup.
pub trait Fragment {
    /// Writes this fragment into `f`, escaping content as appropriate.
    fn fmt(&self, cx: &Cx, f: &mut Formatter<'_>);

    /// Returns a lower bound on the number of bytes this fragment will write.
    ///
    /// Used to pre-allocate the output buffer. Implementations should err on
    /// the side of under-estimating; over-estimates waste memory while
    /// under-estimates only cost an extra reallocation.
    #[inline]
    fn size_hint(&self) -> usize {
        0
    }
}

impl<T> Fragment for &T
where
    T: Fragment + ?Sized,
{
    #[inline]
    fn fmt(&self, cx: &Cx, f: &mut Formatter<'_>) {
        (*self).fmt(cx, f)
    }

    #[inline]
    fn size_hint(&self) -> usize {
        (*self).size_hint()
    }
}

impl Fragment for str {
    #[inline]
    fn fmt(&self, _cx: &Cx, f: &mut Formatter<'_>) {
        f.write_str(self)
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl Fragment for String {
    #[inline]
    fn fmt(&self, _cx: &Cx, f: &mut Formatter<'_>) {
        f.write_str(self)
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl<T> Fragment for Option<T>
where
    T: Fragment,
{
    #[inline]
    fn fmt(&self, cx: &Cx, f: &mut Formatter<'_>) {
        if let Some(fragment) = self {
            fragment.fmt(cx, f);
        }
    }

    #[inline]
    fn size_hint(&self) -> usize {
        match self {
            Some(inner) => inner.size_hint(),
            None => 0,
        }
    }
}

struct UnescapedDisplayAdapter<'a, 'b>(&'a mut Formatter<'b>);

impl core::fmt::Write for UnescapedDisplayAdapter<'_, '_> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.write_str_unescaped(s);
        Ok(())
    }

    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.0.write_char_unescaped(c);
        Ok(())
    }
}

macro_rules! impl_with_display {
    ($ty:ty) => {
        impl Fragment for $ty {
            #[inline]
            fn fmt(&self, _cx: &Cx, f: &mut Formatter<'_>) {
                use core::fmt::Write;
                let _ = write!(UnescapedDisplayAdapter(f), "{self}");
            }

            #[inline]
            fn size_hint(&self) -> usize {
                1
            }
        }
    };
}

impl_with_display!(i8);
impl_with_display!(i16);
impl_with_display!(i32);
impl_with_display!(i64);
impl_with_display!(i128);
impl_with_display!(isize);
impl_with_display!(u8);
impl_with_display!(u16);
impl_with_display!(u32);
impl_with_display!(u64);
impl_with_display!(u128);
impl_with_display!(usize);
impl_with_display!(f32);
impl_with_display!(f64);
impl_with_display!(bool);
impl_with_display!(char);

macro_rules! impl_smart_pointer {
    ($name:ident) => {
        impl<T> Fragment for $name<T>
        where
            T: Fragment + ?Sized,
        {
            #[inline]
            fn fmt(&self, cx: &Cx, f: &mut Formatter<'_>) {
                self.deref().fmt(cx, f);
            }

            #[inline]
            fn size_hint(&self) -> usize {
                self.deref().size_hint()
            }
        }
    };
}

impl_smart_pointer!(Box);
impl_smart_pointer!(Rc);
impl_smart_pointer!(Arc);
