use crate::types::Array;

use super::interface::{ArrayLiteralConverter, IntoTokens, SynInterface, Tokenizer};

impl<T> Array<T>
where
    T: Clone + Default,
    SynInterface<T>: ArrayLiteralConverter<T>,
{
    pub fn parse(expr: syn::Expr) -> syn::Result<Self> {
        let items = SynInterface::<T>::array_from_expr(expr)?;

        Ok(Self::new(items))
    }
}

impl From<Array<::std::ffi::c_char>> for Array<u8> {
    fn from(array: Array<::std::ffi::c_char>) -> Self {
        Self::new(array.values().iter().map(|&x| x as u8).collect())
    }
}

impl<T> IntoTokens for Array<T>
where
    T: Clone + Default,
    SynInterface<T>: Tokenizer<T>,
{
    fn into_tokens(self) -> proc_macro2::TokenStream {
        SynInterface::<T>::all_to_tokens(self.values())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let expr = syn::parse_quote! { [10, 20, 30] };
        let array = Array::<u8>::parse(expr).unwrap();

        assert_eq!(array.values(), vec![10, 20, 30]);
    }

    #[test]
    fn test_into() {
        let array = Array::new(vec![10 as ::std::ffi::c_char, 20, 30]);
        let array: Array<u8> = array.into();

        assert_eq!(array.values(), vec![10, 20, 30]);
    }

    #[test]
    fn test_into_tokens() {
        let array = Array::new(vec![10i8, 20i8, 30i8]);
        let tokens = Array::<u8>::from(array).into_tokens().to_string();

        assert_eq!(tokens, "[10u8 , 20u8 , 30u8]");
    }
}
