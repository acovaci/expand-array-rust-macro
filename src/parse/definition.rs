use crate::types::{Array, ArrayDefinition};

impl syn::parse::Parse for ArrayDefinition {
    fn parse(stream: syn::parse::ParseStream) -> syn::Result<Self> {
        let target = stream.parse()?;
        let (typ, len) = match target {
            syn::Type::Array(syn::TypeArray {
                bracket_token: _,
                elem,
                semi_token: _,
                len,
            }) => (elem, len),
            _ => {
                return Err(syn::Error::new_spanned(
                    target,
                    "Invalid target type. Provide a fixed-sized array",
                ))
            }
        };
        let len = match len {
            syn::Expr::Lit(syn::ExprLit {
                attrs: _,
                lit: syn::Lit::Int(len),
            }) => len.base10_parse::<usize>()?,
            _ => {
                return Err(syn::Error::new_spanned(
                    len,
                    "Invalid target length. Provide a valid integer literal",
                ))
            }
        };
        Ok(Self::new(len, *typ)?)
    }
}
