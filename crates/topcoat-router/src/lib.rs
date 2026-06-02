mod body;
mod error;
mod fallback;
mod manual;
mod module;
mod path;
mod response;
mod state;

pub use body::*;
pub use error::*;
pub use fallback::*;
pub use manual::*;
pub use module::*;
pub use path::*;
pub use response::*;
pub use state::*;

pub use http::Method;

type Result<T = topcoat_view::runtime::View, E = topcoat_core::error::Error> =
    core::result::Result<T, E>;
