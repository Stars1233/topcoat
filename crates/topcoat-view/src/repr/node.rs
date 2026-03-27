use std::borrow::Cow;

use crate::repr::Element;

pub enum Node {
    Text(Cow<'static, str>),
    Element(Element),
}
