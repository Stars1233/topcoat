use syn::parse::{Parse, ParseStream};

pub struct ComponentAttr {}

impl Parse for ComponentAttr {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(Self {})
    }
}
