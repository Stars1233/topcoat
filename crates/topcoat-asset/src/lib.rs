use const_serialize::{ConstStr, ConstVec, SerializeConst, serialize_const};

pub const ASSET_PREFIX: &str = "TOPCOAT_ASSET";

#[derive(Debug, Clone, PartialEq, SerializeConst)]
pub struct Asset {
    path: ConstStr,
}

impl Asset {
    pub const fn new(path: &str) -> Self {
        Self {
            path: ConstStr::new(path),
        }
    }
}

#[macro_export]
macro_rules! asset {
    () => {};
}

pub const KEK: &[u8] = {
    #[used]
    pub static ASSET: [u8; 1024] = const {
        let mut buffer = ConstVec::new();
        buffer = buffer.extend(ASSET_PREFIX.as_bytes());
        buffer = serialize_const(&Asset::new("./kek.png"), buffer);

        let mut out = [0u8; 1024];
        let src = buffer.as_ref();
        let mut i = 0;
        while i < buffer.len() {
            out[i] = src[i];
            i += 1;
        }
        out
    };
    &ASSET
};
