use std::{fs, io, path::Path};

use sha2::{Digest, Sha256};

use crate::{Asset, MANIFEST_NAME, MANIFEST_VERSION, Manifest, ManifestEntry};

pub struct Bundler {}

impl Bundler {
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

        let manifest = Manifest {
            version: MANIFEST_VERSION,
            assets: entries,
        };
        manifest.save(out_dir.join(MANIFEST_NAME))?;

        Ok(())
    }
}
