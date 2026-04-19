use std::{borrow::Borrow, fmt::Display, ops::Deref};

use ref_cast::RefCast;

#[derive(Debug, PartialEq, Eq, Hash, RefCast)]
#[repr(transparent)]
pub struct Path {
    inner: str,
}

impl Path {
    pub fn new<S: AsRef<str> + ?Sized>(s: &S) -> &Self {
        let inner = s.as_ref();
        if !inner.starts_with("/") {
            panic!("paths must start with \"/\"");
        }
        Self::ref_cast(inner)
    }

    pub fn segments(&self) -> impl Iterator<Item = PathSegment<'_>> {
        self.inner.split("/").skip(1).map(PathSegment::new)
    }

    pub fn to_axum_path(&self) -> String {
        self.segments()
            .filter(|s| !s.is_group())
            .collect::<PathBuf>()
            .inner
    }

    pub fn starts_with(&self, other: &Path) -> bool {
        if self.inner.len() > other.inner.len() {
            return false;
        }
        return self.segments().zip(other.segments()).all(|(a, b)| a == b);
    }

    pub fn as_str(&self) -> &str {
        &self.inner
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl ToOwned for Path {
    type Owned = PathBuf;

    fn to_owned(&self) -> Self::Owned {
        PathBuf {
            inner: self.inner.to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathBuf {
    inner: String,
}

impl Borrow<Path> for PathBuf {
    fn borrow(&self) -> &Path {
        Path::new(&self.inner)
    }
}

impl Deref for PathBuf {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        Path::ref_cast(&self.inner)
    }
}

impl<'a> FromIterator<PathSegment<'a>> for PathBuf {
    fn from_iter<T: IntoIterator<Item = PathSegment<'a>>>(iter: T) -> Self {
        let mut buf = String::new();
        for segment in iter {
            use std::fmt::Write;
            write!(buf, "/{segment}").unwrap();
        }
        // Root route is represented as "/" in axum.
        if buf.is_empty() {
            buf += "/";
        }
        Self { inner: buf }
    }
}

impl Display for PathBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathSegment<'a> {
    Static(&'a str),
    Group(&'a str),
    Param(&'a str),
    CatchAll(&'a str),
}

impl<'a> PathSegment<'a> {
    pub fn new(s: &'a str) -> Self {
        if s.starts_with('{') {
            if !s.ends_with('}') {
                panic!("invalid segment: missing closing `}}` in `{s}`");
            }
            let inner = &s[1..s.len() - 1];
            if let Some(name) = inner.strip_prefix('*') {
                assert_valid_ident(name, "catch-all", s);
                PathSegment::CatchAll(name)
            } else {
                assert_valid_ident(inner, "param", s);
                PathSegment::Param(inner)
            }
        } else if s.starts_with('(') {
            if !s.ends_with(')') {
                panic!("invalid segment: missing closing `)` in `{s}`");
            }
            let inner = &s[1..s.len() - 1];
            assert_valid_ident(inner, "group", s);
            PathSegment::Group(inner)
        } else {
            if s.contains('{') || s.contains('}') || s.contains('(') || s.contains(')') {
                panic!("invalid segment: unexpected brackets in `{s}`");
            }
            PathSegment::Static(s)
        }
    }

    /// Returns `true` if the segment is [`Static`].
    ///
    /// [`Static`]: Segment::Static
    #[must_use]
    pub fn is_static(&self) -> bool {
        matches!(self, Self::Static(..))
    }

    /// Returns `true` if the segment is [`Group`].
    ///
    /// [`Group`]: Segment::Group
    #[must_use]
    pub fn is_group(&self) -> bool {
        matches!(self, Self::Group(..))
    }

    /// Returns `true` if the segment is [`Param`].
    ///
    /// [`Param`]: Segment::Param
    #[must_use]
    pub fn is_param(&self) -> bool {
        matches!(self, Self::Param(..))
    }

    /// Returns `true` if the segment is [`CatchAll`].
    ///
    /// [`CatchAll`]: Segment::CatchAll
    #[must_use]
    pub fn is_catch_all(&self) -> bool {
        matches!(self, Self::CatchAll(..))
    }

    pub fn as_static(&self) -> Option<&&'a str> {
        if let Self::Static(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_group(&self) -> Option<&&'a str> {
        if let Self::Group(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_param(&self) -> Option<&&'a str> {
        if let Self::Param(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_catch_all(&self) -> Option<&&'a str> {
        if let Self::CatchAll(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

fn assert_valid_ident(name: &str, kind: &str, raw: &str) {
    if name.is_empty() {
        panic!("invalid segment: {kind} name must not be empty in `{raw}`");
    }
    let mut chars = name.chars();
    let first = chars.next().unwrap();
    if !first.is_ascii_alphabetic() && first != '_' {
        panic!(
            "invalid segment: {kind} name `{name}` must start with a letter or underscore in `{raw}`"
        );
    }
    for ch in chars {
        if !ch.is_ascii_alphanumeric() && ch != '_' {
            panic!(
                "invalid segment: {kind} name `{name}` contains invalid character `{ch}` in `{raw}`"
            );
        }
    }
}

impl Display for PathSegment<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Static(inner) => f.write_str(inner),
            Self::Param(inner) => write!(f, "{{{inner}}}"),
            Self::Group(inner) => write!(f, "({inner})"),
            Self::CatchAll(inner) => write!(f, "{{*{inner}}}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_segment() {
        let seg = PathSegment::new("dashboard");
        assert!(seg.is_static());
        assert_eq!(seg.as_static(), Some(&"dashboard"));
    }

    #[test]
    fn param_segment() {
        let seg = PathSegment::new("{id}");
        assert!(seg.is_param());
        assert_eq!(seg.as_param(), Some(&"id"));
    }

    #[test]
    fn param_with_underscore() {
        let seg = PathSegment::new("{user_id}");
        assert!(seg.is_param());
        assert_eq!(seg.as_param(), Some(&"user_id"));
    }

    #[test]
    fn catch_all_segment() {
        let seg = PathSegment::new("{*rest}");
        assert!(matches!(seg, PathSegment::CatchAll("rest")));
    }

    #[test]
    fn group_segment() {
        let seg = PathSegment::new("(auth)");
        assert!(seg.is_group());
        assert_eq!(seg.as_group(), Some(&"auth"));
    }

    #[test]
    fn display_roundtrip() {
        for input in ["dashboard", "{id}", "{*rest}", "(auth)"] {
            assert_eq!(PathSegment::new(input).to_string(), input);
        }
    }

    #[test]
    #[should_panic(expected = "missing closing `}`")]
    fn param_missing_close() {
        PathSegment::new("{id");
    }

    #[test]
    #[should_panic(expected = "missing closing `)`")]
    fn group_missing_close() {
        PathSegment::new("(auth");
    }

    #[test]
    #[should_panic(expected = "segment must not be empty")]
    fn empty_segment() {
        PathSegment::new("");
    }

    #[test]
    #[should_panic(expected = "unexpected brackets")]
    fn static_with_braces() {
        PathSegment::new("foo{bar}");
    }

    #[test]
    #[should_panic(expected = "name must not be empty")]
    fn param_empty_name() {
        PathSegment::new("{}");
    }

    #[test]
    #[should_panic(expected = "name must not be empty")]
    fn group_empty_name() {
        PathSegment::new("()");
    }

    #[test]
    #[should_panic(expected = "name must not be empty")]
    fn catch_all_empty_name() {
        PathSegment::new("{*}");
    }

    #[test]
    #[should_panic(expected = "must start with a letter or underscore")]
    fn param_invalid_start() {
        PathSegment::new("{0id}");
    }

    #[test]
    #[should_panic(expected = "contains invalid character")]
    fn param_invalid_char() {
        PathSegment::new("{id-name}");
    }

    #[test]
    #[should_panic(expected = "must start with a letter or underscore")]
    fn group_invalid_start() {
        PathSegment::new("(0auth)");
    }

    #[test]
    #[should_panic(expected = "contains invalid character")]
    fn group_invalid_char() {
        PathSegment::new("(my-group)");
    }

    #[test]
    fn underscore_leading_ident() {
        let seg = PathSegment::new("{_private}");
        assert!(seg.is_param());
        assert_eq!(seg.as_param(), Some(&"_private"));
    }
}
