mod bind_attribute;
mod event_handler;
mod expr;
mod signal;
mod surrogate;

pub use bind_attribute::*;
pub use event_handler::*;
pub use expr::*;
pub use signal::*;
pub use surrogate::*;

use topcoat_asset::{Asset, asset};

pub const SCRIPT: Asset = asset!("browser/dist/index.mjs", rename: "topcoat");
