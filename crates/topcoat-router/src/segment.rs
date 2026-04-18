use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentKind {
    Static,
    Group,
    Param,
    CatchAll,
}

#[derive(Debug, Clone)]
pub struct Segment {
    file: &'static str,
    kind: Option<SegmentKind>,
    rename: Option<Cow<'static, str>>,
}

impl Segment {
    pub const fn new(
        file: &'static str,
        kind: Option<SegmentKind>,
        rename: Option<Cow<'static, str>>,
    ) -> Self {
        Self { file, kind, rename }
    }

    pub fn file(&self) -> &'static str {
        self.file
    }

    pub fn kind(&self) -> Option<&SegmentKind> {
        self.kind.as_ref()
    }

    pub fn rename(&self) -> Option<&Cow<'static, str>> {
        self.rename.as_ref()
    }
}

#[cfg(feature = "discover")]
inventory::collect!(Segment);
