pub use topcoat_macro::*;

pub mod view {
    pub use topcoat_view::runtime::*;
}

pub mod component {
    pub trait Component {
        type Props;
    }

    pub trait Props {}
}
