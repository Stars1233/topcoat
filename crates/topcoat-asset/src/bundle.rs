use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{Asset, AssetId};

const MANIFEST_NAME: &str = "manifest.toml";

pub struct BundledAsset {
    path: PathBuf,
}

impl BundledAsset {
    pub fn path(&self) -> &Path {
        &self.path
    }
}

pub struct AssetBundle {
    bundled_assets: HashMap<AssetId, BundledAsset>,
}

impl AssetBundle {
    pub fn load(dir: impl AsRef<Path>) -> io::Result<Self> {
        let dir = dir.as_ref();
        let toml_str = fs::read_to_string(dir.join(MANIFEST_NAME))?;
        let manifest: Manifest =
            toml::from_str(&toml_str).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

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

        Ok(Self { bundled_assets })
    }

    pub fn get(&self, id: AssetId) -> Option<&BundledAsset> {
        self.bundled_assets.get(&id)
    }
}

/// Scan `binary` for embedded assets, copy each source file into `out_dir`
/// with a content-hash suffix in its name, and write a `manifest.toml`
/// mapping asset ids to file names.
pub fn bundle(binary: &[u8], out_dir: impl AsRef<Path>) -> io::Result<()> {
    let out_dir = out_dir.as_ref();
    fs::create_dir_all(out_dir)?;

    let assets = Asset::find_in_binary(binary);
    let mut entries = Vec::with_capacity(assets.len());

    for asset in assets {
        let src = asset.resolved_path();
        let bytes = fs::read(&src)?;
        let digest = Sha256::digest(&bytes);
        let short_hash = format!(
            "{:02x}{:02x}{:02x}{:02x}",
            digest[0], digest[1], digest[2], digest[3]
        );

        let stem = src.file_stem().and_then(|s| s.to_str()).unwrap_or("asset");
        let file = match src.extension().and_then(|e| e.to_str()) {
            Some(ext) => format!("{stem}-{short_hash}.{ext}"),
            None => format!("{stem}-{short_hash}"),
        };

        fs::copy(&src, out_dir.join(&file))?;
        entries.push(ManifestEntry {
            id: asset.id(),
            file,
        });
    }

    let manifest = Manifest { assets: entries };
    let toml_str = toml::to_string_pretty(&manifest).map_err(io::Error::other)?;
    fs::write(out_dir.join(MANIFEST_NAME), toml_str)?;

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Manifest {
    assets: Vec<ManifestEntry>,
}

#[derive(Serialize, Deserialize)]
struct ManifestEntry {
    id: AssetId,
    file: String,
}
