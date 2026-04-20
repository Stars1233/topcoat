use std::{borrow::Cow, collections::HashMap};

/// The kind of a file-router path segment, set via the `segment!` macro.
///
/// When using the file router, each module maps to a URL segment. By default,
/// regular modules are `Static` and `_`-prefixed modules are `Group`. Use
/// `segment!(kind = ...)` in a module to override the default.
///
/// | Kind       | URL format   | Use case                                       |
/// |------------|--------------|------------------------------------------------|
/// | `Static`   | `/name`      | Default for regular modules                    |
/// | `Group`    | *(hidden)*   | Default for `_`-prefixed modules; layout-only  |
/// | `Param`    | `/{name}`    | Dynamic path parameter                         |
/// | `CatchAll` | `/{*name}`   | Matches all remaining path segments            |
///
/// # Examples
///
/// ```rust,ignore
/// // In a file-router module (e.g. src/app/users/id/mod.rs):
/// topcoat::router::segment!(kind = Param);
/// // This module now maps to /users/{id}
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SegmentKind {
    /// A literal URL segment (e.g. `/users`). Default for regular modules.
    Static,
    /// A layout-only grouping that doesn't appear in the URL. Default for `_`-prefixed modules.
    Group,
    /// A dynamic path parameter (e.g. `/{id}`).
    Param,
    /// A wildcard tail that matches all remaining path segments (e.g. `/{*path}`).
    CatchAll,
}

/// A file-router segment declaration, produced by the `segment!` macro.
#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct Segment {
    /// Source file path (set automatically by the `segment!` macro via `file!()`).
    file: &'static str,
    /// Overridden segment kind, or `None` to use the default (static / group).
    kind: Option<SegmentKind>,
    /// Overridden URL name, or `None` to derive from the module name.
    rename: Option<Cow<'static, str>>,
}

impl Segment {
    /// Creates a new segment. Called by the expanded `segment!` macro.
    pub const fn new(
        file: &'static str,
        kind: Option<SegmentKind>,
        rename: Option<Cow<'static, str>>,
    ) -> Self {
        Self { file, kind, rename }
    }

    /// Returns the source file that declared this segment.
    pub fn file(&self) -> &'static str {
        self.file
    }

    /// Returns the overridden [`SegmentKind`], if any.
    pub fn kind(&self) -> Option<&SegmentKind> {
        self.kind.as_ref()
    }

    /// Returns the overridden URL name, if any.
    pub fn rename(&self) -> Option<&str> {
        self.rename.as_deref()
    }
}

#[cfg(feature = "discover")]
inventory::collect!(Segment);

/// Registry of [`Segment`] declarations, keyed by module path.
///
/// The file router builds a `Segments` map from all `segment!` invocations,
/// then consults it while walking the module tree to determine each module's
/// URL contribution.
#[doc(hidden)]
#[derive(Debug, Default, Clone)]
pub(crate) struct Segments {
    segments: HashMap<&'static str, Segment>,
}

impl Segments {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Default::default()
    }

    /// Registers a segment for a module path. Panics on duplicates.
    pub fn register(&mut self, path: &'static str, segment: Segment) {
        if let Some(existing) = self.segments.insert(path, segment) {
            panic!("duplicate segment specifier in `{}`", existing.file())
        }
    }

    /// Looks up the segment declaration for a module path.
    pub fn get(&self, path: &str) -> Option<&Segment> {
        self.segments.get(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_segment() -> Segment {
        Segment::new("test.rs", Some(SegmentKind::Static), None)
    }

    #[test]
    fn register_and_get() {
        let mut segments = Segments::new();
        segments.register("foo", test_segment());

        let seg = segments.get("foo").unwrap();
        assert_eq!(seg.file(), "test.rs");
        assert_eq!(seg.kind(), Some(&SegmentKind::Static));
        assert_eq!(seg.rename(), None);
    }

    #[test]
    fn get_missing_returns_none() {
        let segments = Segments::new();
        assert!(segments.get("nope").is_none());
    }

    #[test]
    #[should_panic(expected = "duplicate segment specifier")]
    fn register_duplicate_panics() {
        let mut segments = Segments::new();
        segments.register("foo", test_segment());
        segments.register("foo", test_segment());
    }
}
