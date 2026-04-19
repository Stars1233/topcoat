use std::{borrow::Cow, collections::HashMap};

use crate::file::canonical_module_path;

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

#[derive(Default)]
pub struct Segments {
    segments: HashMap<&'static str, Segment>,
}

impl Segments {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn register(&mut self, segment: Segment) {
        self.segments
            .insert(canonical_module_path(segment.file), segment);
    }

    pub fn get(&self, canonical_module_path: &str) -> Option<&Segment> {
        self.segments.get(canonical_module_path)
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }
}
