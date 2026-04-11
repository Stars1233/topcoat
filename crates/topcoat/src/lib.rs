pub use topcoat_macro::*;

pub mod component {
    pub trait Component {
        fn render(self) -> impl Future<Output = crate::View> + Send;
    }
}

pub mod router {
    pub use topcoat_router::*;
}

pub use topcoat_view::runtime::*;

pub mod axum {
    pub use axum::*;
}
