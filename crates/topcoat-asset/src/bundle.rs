use std::{
    collections::HashMap,
    ffi::OsStr,
    io,
    path::{Path, PathBuf},
};

use crate::{Asset, MANIFEST_NAME, Manifest};

/// A single entry inside an [`AssetBundle`].
#[derive(Debug, Clone)]
pub struct BundledAsset {
    path: PathBuf,
}

impl BundledAsset {
    /// Absolute path to the bundled file on disk.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Bundled filename (typically `stem-<short-hash>.ext`).
    pub fn name(&self) -> &OsStr {
        self.path
            .file_name()
            .expect("asset file path must have a name")
    }
}

/// A loaded asset bundle: a directory of files plus the mapping from
/// [`Asset`] IDs to those files.
///
/// Built by the [`Bundler`](crate::Bundler) and loaded at runtime via
/// [`AssetBundle::load`].
#[derive(Debug, Default, Clone)]
pub struct AssetBundle {
    dir: PathBuf,
    bundled_assets: HashMap<Asset, BundledAsset>,
}

impl AssetBundle {
    /// Bundle with no assets and no directory; useful as a placeholder.
    pub fn empty() -> Self {
        Default::default()
    }

    /// Load a bundle by reading the `manifest.toml` in `dir`.
    pub fn load(dir: impl AsRef<Path>) -> io::Result<Self> {
        let dir = dir.as_ref().to_path_buf();
        let manifest = Manifest::load(dir.join(MANIFEST_NAME))?;

        let bundled_assets = manifest
            .assets
            .into_iter()
            .map(|entry| {
                (
                    entry.id,
                    BundledAsset {
                        path: dir.join(entry.file),
                    },
                )
            })
            .collect();

        Ok(Self {
            dir,
            bundled_assets,
        })
    }

    /// Directory the bundle was loaded from.
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Look up the bundled file for an [`Asset`] ID.
    pub fn get(&self, id: Asset) -> Option<&BundledAsset> {
        self.bundled_assets.get(&id)
    }

    /// Iterate over every bundled asset in arbitrary order.
    pub fn assets(&self) -> impl Iterator<Item = &BundledAsset> {
        self.bundled_assets.values()
    }
}
