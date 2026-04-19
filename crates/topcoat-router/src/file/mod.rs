mod layout;
mod page;
mod segment;

pub use layout::*;
pub use page::*;
pub use segment::*;

use crate::Router;

#[cfg(feature = "discover")]
#[macro_export]
macro_rules! file_router {
    () => {
        ::topcoat::router::FileRouter::new(file!())
            .discover()
            .into::<::topcoat::router::Router>()
    };
}

#[doc(hidden)]
pub struct FileRouter {
    inner: Router,
    file_root: &'static str,
    segments: Segments,
}

impl FileRouter {
    pub fn new(file_root: &'static str) -> Self {
        Self {
            file_root,
            inner: Router::new(),
            segments: Segments::new(),
        }
    }

    pub fn segment(mut self, segment: Segment) -> Self {
        assert!(
            self.inner.is_empty(),
            "`segment` must be called before registering any resource"
        );
        self.segments.register(segment);
        self
    }

    pub fn page(mut self, page: FilePage) -> Self {
        self
    }

    pub fn layout(mut self, layout: FileLayout) -> Self {
        self
    }

    pub fn disover(mut self) -> Self {
        for page in inventory::iter::<FilePage>().cloned() {
            self = self.page(page);
        }
        for layout in inventory::iter::<FileLayout>().cloned() {
            self = self.layout(layout);
        }
        self.inner = self.inner.discover();
        self
    }

    /// Derives an HTTP route path from a Rust source file path.
    ///
    /// # Examples
    ///
    /// - `./src/app/home.rs` → `/home`
    /// - `./src/app/dashboard/_group/settings/mod.rs` → `/dashboard/settings`
    pub(crate) fn path_from_file(&self, file: &str) -> PathBuf {
        let file_root = self
            .file_root
            .as_deref()
            .expect("determining path from file needs file root");
        let file_root = canonical_module_path(file_root);

        let file = file
            .strip_prefix(file_root)
            .expect("file must be under file router's file root");
        let file = canonical_module_path(file);

        file.split(&['\\', '/'])
            .skip(1)
            .map(|s| {
                if s.starts_with("_") {
                    PathSegment::Group(s)
                } else {
                    PathSegment::Static(s)
                }
            })
            .collect()
    }
}

impl From<FileRouter> for Router {
    fn from(value: FileRouter) -> Self {
        value.inner
    }
}

pub fn canonical_module_path(path: &str) -> &str {
    let path = path.strip_suffix(".rs").unwrap_or(path);
    let path = path.strip_suffix("/mod").unwrap_or(path);
    path.strip_suffix("\\mod").unwrap_or(path)
}
