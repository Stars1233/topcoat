use std::borrow::Cow;

use crate::cursor::{ConstReader, ConstWriter};

/// Options that control how an asset is bundled.
///
/// Usually set via the [`asset!`](crate::asset) or
/// [`asset_options!`](crate::asset_options) macros rather than
/// constructed directly. See the [`asset!`](crate::asset) docs for what
/// each field does.
#[derive(Debug, Clone, PartialEq)]
pub struct AssetOptions {
    pub rename: Option<Cow<'static, str>>,
    pub extension: Option<Cow<'static, str>>,
    pub hash: Option<Cow<'static, str>>,
}

impl AssetOptions {
    /// All options unset.
    pub const NONE: Self = Self {
        rename: None,
        extension: None,
        hash: None,
    };

    pub fn rename(&self) -> Option<&str> {
        self.rename.as_deref()
    }

    pub fn extension(&self) -> Option<&str> {
        self.extension.as_deref()
    }

    pub fn hash(&self) -> Option<&str> {
        self.hash.as_deref()
    }

    pub(crate) const fn encode_into(&self, w: &mut ConstWriter<'_>) {
        w.write_str_opt(cow_as_str(&self.rename));
        w.write_str_opt(cow_as_str(&self.extension));
        w.write_str_opt(cow_as_str(&self.hash));
    }

    pub(crate) fn decode_from(r: &mut ConstReader<'_>) -> Option<Self> {
        Some(Self {
            rename: r.read_str_opt()?.map(|s| Cow::Owned(s.to_owned())),
            extension: r.read_str_opt()?.map(|s| Cow::Owned(s.to_owned())),
            hash: r.read_str_opt()?.map(|s| Cow::Owned(s.to_owned())),
        })
    }
}

const fn cow_as_str<'a>(c: &'a Option<Cow<'static, str>>) -> Option<&'a str> {
    match c {
        None => None,
        Some(Cow::Borrowed(s)) => Some(s),
        Some(Cow::Owned(s)) => Some(s.as_str()),
    }
}

/// Build an [`AssetOptions`] from a comma-separated list of fields.
///
/// Each field is either `name: "literal"` to set that option, or a bare
/// `name` (which expects a const string in scope of the same name).
/// Omitted fields stay `None`.
///
/// ```ignore
/// use topcoat_asset::{asset_options, AssetOptions};
///
/// const OPTS: AssetOptions = asset_options!(rename: "primary", extension: "woff2");
/// ```
#[macro_export]
macro_rules! asset_options {
    ($($field:ident $(: $expr:expr)?),*) => {{
        #[allow(clippy::needless_update)]
        $crate::AssetOptions {
            $($field: ::core::option::Option::Some(::std::borrow::Cow::Borrowed($($expr)?)),)*
            ..$crate::AssetOptions::NONE
        }
    }};
}
