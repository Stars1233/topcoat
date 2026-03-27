use std::borrow::Cow;

pub struct Attribute {
    name: Cow<'static, str>,
    value: Cow<'static, str>,
}

impl Attribute {
    pub fn new(name: Cow<'static, str>, value: Cow<'static, str>) -> Self {
        Self { name, value }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

pub struct Attributes {
    items: Vec<Attribute>,
}

impl Attributes {
    pub fn new(items: Vec<Attribute>) -> Self {
        Self { items }
    }

    pub fn items(&self) -> &[Attribute] {
        &self.items
    }
}
