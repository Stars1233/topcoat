use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
};

use crate::{AssetId, MANIFEST_NAME, Manifest};

pub struct BundledAsset {
    path: PathBuf,
}

impl BundledAsset {
    pub fn path(&self) -> &Path {
        &self.path
    }
}

pub struct AssetBundle {
    dir: PathBuf,
    bundled_assets: HashMap<AssetId, BundledAsset>,
}

impl AssetBundle {
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

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    pub fn get(&self, id: AssetId) -> Option<&BundledAsset> {
        self.bundled_assets.get(&id)
    }

    pub fn assets(&self) -> impl Iterator<Item = &BundledAsset> {
        self.bundled_assets.values()
    }
}
