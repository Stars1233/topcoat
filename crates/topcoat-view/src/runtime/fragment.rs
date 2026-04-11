use crate::runtime::view::View;

pub trait Fragment {
    fn as_str(&self) -> &str;
}

impl<T> Fragment for T
where
    T: AsRef<str>,
{
    fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl Fragment for &View {
    fn as_str(&self) -> &str {
        &self.buf
    }
}

impl Fragment for View {
    fn as_str(&self) -> &str {
        &self.buf
    }
}
