use super::interface::{SynInterface, Tokenizer};

impl<T> Tokenizer<T> for SynInterface<u8>
where
    T: Into<u8> + quote::ToTokens,
{
    fn to_tokens(value: T) -> proc_macro2::TokenStream {
        quote::quote! { #value }
    }
}

impl<T> Tokenizer<T> for SynInterface<i8>
where
    T: Into<i8> + quote::ToTokens,
{
    fn to_tokens(value: T) -> proc_macro2::TokenStream {
        quote::quote! { #value }
    }
}

impl<T> Tokenizer<T> for SynInterface<char>
where
    T: Into<char> + quote::ToTokens,
{
    fn to_tokens(value: T) -> proc_macro2::TokenStream {
        quote::quote! { #value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8() {
        let value = 10u8;
        assert_eq!(SynInterface::<u8>::to_tokens(value).to_string(), "10u8");
    }

    #[test]
    fn test_char() {
        let value = 'x';
        assert_eq!(SynInterface::<char>::to_tokens(value).to_string(), "'x'");
    }

    #[test]
    fn test_c_char() {
        let value = 10 as ::std::ffi::c_char;
        assert_eq!(
            SynInterface::<u8>::to_tokens(value as u8).to_string(),
            "10u8"
        );
    }
}
