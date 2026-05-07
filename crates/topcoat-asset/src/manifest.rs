use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};

use crate::AssetId;

pub const MANIFEST_NAME: &str = "manifest.toml";
pub const MANIFEST_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    pub version: u32,
    pub assets: Vec<ManifestEntry>,
}

impl Manifest {
    pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let toml_str = fs::read_to_string(path)?;
        let manifest: Manifest =
            toml::from_str(&toml_str).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        if manifest.version != MANIFEST_VERSION {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "unsupported manifest version {} (expected {})",
                    manifest.version, MANIFEST_VERSION
                ),
            ));
        }

        Ok(manifest)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let toml_str = toml::to_string_pretty(self).map_err(io::Error::other)?;
        fs::write(path, toml_str)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ManifestEntry {
    pub id: AssetId,
    pub file: String,
}
