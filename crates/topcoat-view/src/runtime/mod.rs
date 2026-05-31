mod attribute;
mod component;
mod element;
mod formatter;
mod fragment;
mod node;
mod unescaped;
mod view;

pub use attribute::*;
pub use component::*;
pub use element::*;
pub use formatter::*;
pub use fragment::*;
pub use node::*;
pub use unescaped::*;
pub use view::*;

/// Macro helpers to shorten the generated source code.
#[doc(hidden)]
pub mod internal {
    use crate::runtime::{
        AttributeKeyViewParts, AttributeValueViewParts, AttributeViewParts, ElementNameViewParts,
        NodeViewParts, ViewPart,
    };

    #[inline(always)]
    pub fn __attributes(attributes: impl AttributeViewParts) -> impl Iterator<Item = ViewPart> {
        attributes.into_view_parts()
    }

    #[inline(always)]
    pub fn __attribute_key(
        attribute_key: impl AttributeKeyViewParts,
    ) -> impl Iterator<Item = ViewPart> {
        attribute_key.into_view_parts()
    }

    #[inline(always)]
    pub fn __attribute_value(
        attribute_value: impl AttributeValueViewParts,
    ) -> impl Iterator<Item = ViewPart> {
        attribute_value.into_view_parts()
    }

    #[inline(always)]
    pub fn __element_name(
        element_name: impl ElementNameViewParts,
    ) -> impl Iterator<Item = ViewPart> {
        element_name.into_view_parts()
    }

    #[inline(always)]
    pub fn __node(node: impl NodeViewParts) -> impl Iterator<Item = ViewPart> {
        node.into_view_parts()
    }
}
