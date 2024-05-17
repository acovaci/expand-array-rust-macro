use crate::parse::array::{ArrayBuilder, ArrayTokenizer};
use crate::types::ArrayDefinition;

pub(crate) struct Input {
    pub(crate) definition: ArrayDefinition,
    pub(crate) tokenizer: ArrayTokenizer,
}

impl syn::parse::Parse for Input {
    fn parse(stream: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: syn::Visibility = stream.parse()?;
        let definition = stream.parse()?;
        stream.parse::<syn::Token![,]>()?;
        let tokenizer = stream.parse()?;

        Ok(Self {
            definition,
            tokenizer,
        })
    }
}

impl Input {
    pub fn build_array(&self) -> syn::Result<proc_macro2::TokenStream> {
        let mut builder = ArrayBuilder::from_definition(self.definition.clone());
        builder.set_tokenizer(self.tokenizer.clone());
        builder.build()
    }
}
