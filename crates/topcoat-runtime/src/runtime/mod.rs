mod bind_attribute;
mod event_handler;
mod expr;
mod js;
mod signal;
mod surrogate;

pub use bind_attribute::*;
pub use event_handler::*;
pub use expr::*;
pub use js::*;
pub use signal::*;
pub use surrogate::*;

use topcoat_asset::{Asset, asset};

pub const SCRIPT: Asset = asset!("browser/dist/index.mjs", rename: "topcoat");

/// Macro helpers to shorten the generated source code.
#[doc(hidden)]
pub mod internal {
    use topcoat_view::runtime::{NodeViewParts, Unescaped, ViewParts};

    use crate::runtime::JsViewParts;

    #[inline(always)]
    pub fn __js(parts: &mut ViewParts, js: &(impl JsViewParts + ?Sized)) {
        js.to_view_parts(parts);
    }

    #[inline(always)]
    pub fn __js_unescaped(parts: &mut ViewParts, s: &str) {
        Unescaped::new_unchecked(s).into_view_parts(parts);
    }
}
