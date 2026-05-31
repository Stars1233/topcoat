use topcoat_view::runtime::ViewPart;

#[derive(Debug, Clone)]
pub struct Expr<T> {
    pub(crate) evaluated: T,
    pub(crate) js: ViewPart,
}

impl<T> Expr<T> {
    #[inline]
    pub fn new(evaluated: T, js: ViewPart) -> Self {
        Self { evaluated, js }
    }
}
