use std::collections::HashMap;

use crate::runtime::{Attribute, AttributeValueViewParts, AttributeViewParts, ViewPart, ViewParts};

#[derive(Debug, Default, Clone)]
pub struct Attributes {
    map: HashMap<String, ViewPart>,
}

impl Attributes {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn contains_key(&self, k: impl AsRef<str>) -> bool {
        self.map.contains_key(k.as_ref())
    }

    #[inline]
    pub fn get(&mut self, k: impl AsRef<str>) -> Option<&ViewPart> {
        self.map.get(k.as_ref())
    }

    #[inline]
    pub fn insert(
        &mut self,
        k: impl Into<String>,
        v: impl AttributeValueViewParts,
    ) -> Option<ViewPart> {
        let mut view_parts = ViewParts::new();
        v.into_view_parts(&mut view_parts);
        self.map.insert(k.into(), view_parts.into())
    }

    #[inline]
    pub fn clear(&mut self) {
        self.map.clear();
    }

    #[inline]
    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

impl AttributeViewParts for Attributes {
    fn into_view_parts(self, parts: &mut ViewParts) {
        for (key, value) in self {
            Attribute::new(key, value).into_view_parts(parts);
        }
    }
}

impl IntoIterator for Attributes {
    type Item = (String, ViewPart);
    type IntoIter = std::collections::hash_map::IntoIter<String, ViewPart>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl<'a> IntoIterator for &'a Attributes {
    type Item = (&'a String, &'a ViewPart);
    type IntoIter = std::collections::hash_map::Iter<'a, String, ViewPart>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}
