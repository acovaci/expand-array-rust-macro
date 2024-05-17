use crate::types::{Array, ArrayDefinition, RustType};

use super::interface::IntoTokens;

#[derive(Clone)]
pub struct ArrayTokenizer {
    array: syn::Expr,
}

impl ArrayTokenizer {
    pub fn tokens(&self) -> &syn::Expr {
        &self.array
    }
}

pub struct ArrayBuilder {
    size: usize,
    typ: RustType,
    tokenizer: Option<ArrayTokenizer>,
}

impl ArrayBuilder {
    pub fn from_definition(definition: ArrayDefinition) -> Self {
        Self {
            size: definition.size(),
            typ: definition.element_type().clone(),
            tokenizer: None,
        }
    }

    pub fn set_tokenizer(&mut self, parser: ArrayTokenizer) {
        self.tokenizer = Some(parser);
    }

    pub fn build(self) -> syn::Result<proc_macro2::TokenStream> {
        let parser = self.tokenizer.unwrap();

        let values = match self.typ {
            x if (..RustType::from("::std::primitive::u8")).contains(&x) => {
                let mut array = Array::<u8>::parse(parser.tokens().clone())?;
                array.ensure_size(self.size)?;
                array.into_tokens()
            }
            x if (..RustType::from("::std::primitive::char")).contains(&x) => {
                let mut array = Array::<char>::parse(parser.tokens().clone())?;
                array.ensure_size(self.size)?;
                array.into_tokens()
            }
            x if (..RustType::from("::std::ffi::c_char")).contains(&x) => {
                let mut array = Array::<::std::ffi::c_char>::parse(parser.tokens().clone())?;
                array.ensure_size(self.size)?;
                array.into_tokens()
            }
            _ => {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Type {} is not supported", self.typ),
                ));
            }
        };

        Ok(values)
    }
}

impl syn::parse::Parse for ArrayTokenizer {
    fn parse(stream: syn::parse::ParseStream) -> syn::Result<Self> {
        let array = stream.parse()?;
        Ok(Self { array })
    }
}
