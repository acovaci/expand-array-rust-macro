use super::interface::SynInterface;
#[allow(unused_imports)]
use super::traits::{ArrayLiteralConverter, LiteralConverter};

impl LiteralConverter<u8> for SynInterface<u8> {
    fn value_from_lit(lit: syn::ExprLit) -> syn::Result<u8> {
        match lit.lit {
            syn::Lit::Int(lit) => {
                let value = lit.base10_parse::<u8>()?;
                Ok(value)
            }
            syn::Lit::Byte(lit) => Ok(lit.value()),
            _ => Err(syn::Error::new_spanned(
                lit.clone(),
                format!(
                    "Invalid value. Expected a literal for type u8 but got {:?} instead",
                    quote::quote! { #lit }.to_string()
                ),
            )),
        }
    }
}

impl LiteralConverter<Vec<u8>> for SynInterface<u8> {
    fn value_from_lit(lit: syn::ExprLit) -> syn::Result<Vec<u8>> {
        match lit.lit {
            syn::Lit::Str(lit) => Ok(lit.value().bytes().collect()),
            syn::Lit::ByteStr(lit) => Ok(lit.value()),
            syn::Lit::CStr(lit) => Ok(lit.value().to_bytes_with_nul().to_vec()),
            _ => Err(syn::Error::new_spanned(
                lit.clone(),
                format!(
                    "Invalid value. Expected a byte string literal for type Vec<u8> but got {:?} instead",
                    quote::quote! { #lit }.to_string()
                ),
            )),
        }
    }
}

impl LiteralConverter<char> for SynInterface<char> {
    fn value_from_lit(lit: syn::ExprLit) -> syn::Result<char> {
        match lit.lit {
            syn::Lit::Char(lit) => Ok(lit.value()),
            _ => Err(syn::Error::new_spanned(
                lit.clone(),
                format!(
                    "Invalid value. Expected a char literal for type char but got {:?} instead",
                    quote::quote! { #lit }.to_string()
                ),
            )),
        }
    }
}

impl LiteralConverter<Vec<char>> for SynInterface<char> {
    fn value_from_lit(lit: syn::ExprLit) -> syn::Result<Vec<char>> {
        match lit.lit {
            syn::Lit::Str(lit) => Ok(lit.value().chars().collect()),
            syn::Lit::ByteStr(lit) => Ok(String::from_utf8_lossy(&lit.value()).chars().collect()),
            syn::Lit::CStr(lit) => Ok(lit.value().to_string_lossy().chars().collect()),
            _ => Err(syn::Error::new_spanned(
                lit.clone(),
                format!(
                    "Invalid value. Expected a string literal for type Vec<char> but got {:?} instead",
                    quote::quote! { #lit }.to_string()
                ),
            )),
        }
    }
}

impl LiteralConverter<::std::ffi::c_char> for SynInterface<::std::ffi::c_char> {
    fn value_from_lit(lit: syn::ExprLit) -> syn::Result<::std::ffi::c_char> {
        match lit.lit {
            syn::Lit::Byte(lit) => Ok(lit.value() as _),
            syn::Lit::Int(lit) => Ok(lit.base10_parse::<u8>()? as _),
            _ => Err(syn::Error::new_spanned(
                lit.clone(),
                format!(
                    "Invalid value. Expected a char literal for type c_char but got {:?} instead",
                    quote::quote! { #lit }.to_string()
                ),
            )),
        }
    }
}

impl LiteralConverter<Vec<::std::ffi::c_char>> for SynInterface<::std::ffi::c_char> {
    fn value_from_lit(lit: syn::ExprLit) -> syn::Result<Vec<::std::ffi::c_char>> {
        match lit.lit {
            syn::Lit::Str(lit) => Ok(lit.value().bytes().map(|b| b as _).collect()),
            syn::Lit::ByteStr(lit) => Ok(lit.value().iter().map(|b| *b as _).collect()),
            syn::Lit::CStr(lit) => Ok(lit.value().to_bytes_with_nul().iter().map(|b| *b as _).collect()),
            _ => Err(syn::Error::new_spanned(
                lit.clone(),
                format!(
                    "Invalid value. Expected a byte string literal for type Vec<c_char> but got {:?} instead",
                    quote::quote! { #lit }.to_string()
                ),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lit_u8() {
        let lit = syn::parse_quote! { 10 };
        assert_eq!(
            <SynInterface::<u8> as LiteralConverter<u8>>::value_from_lit(lit).unwrap(),
            10
        );

        let lit = syn::parse_quote! { 10u8 };
        assert_eq!(
            <SynInterface::<u8> as LiteralConverter<u8>>::value_from_lit(lit).unwrap(),
            10
        );

        let lit = syn::parse_quote! { b'x' };
        assert_eq!(
            <SynInterface::<u8> as LiteralConverter<u8>>::value_from_lit(lit).unwrap(),
            120
        );

        let lit = syn::parse_quote! { b"abc" };
        assert_eq!(
            <SynInterface::<u8> as LiteralConverter<Vec<u8>>>::value_from_lit(lit).unwrap(),
            vec![97, 98, 99]
        );
    }

    #[test]
    fn test_lit_char() {
        let lit = syn::parse_quote! { 'x' };
        assert_eq!(
            <SynInterface::<char> as LiteralConverter<char>>::value_from_lit(lit).unwrap(),
            'x'
        );

        let lit = syn::parse_quote! { "abc" };
        assert_eq!(
            <SynInterface::<char> as LiteralConverter<Vec<char>>>::value_from_lit(lit).unwrap(),
            vec!['a', 'b', 'c']
        );
    }

    #[test]
    fn test_expr_u8() {
        let expr = syn::parse_quote! { [10, 20, 30] };
        assert_eq!(
            SynInterface::<u8>::array_from_expr(expr).unwrap(),
            vec![10, 20, 30]
        );

        let expr = syn::parse_quote! { b"abc" };
        assert_eq!(
            SynInterface::<u8>::array_from_expr(expr).unwrap(),
            vec![97, 98, 99]
        );
    }

    #[test]
    fn test_expr_char() {
        let expr = syn::parse_quote! { ['a', 'b', 'c'] };
        assert_eq!(
            SynInterface::<char>::array_from_expr(expr).unwrap(),
            vec!['a', 'b', 'c']
        );

        let expr = syn::parse_quote! { "abc" };
        assert_eq!(
            SynInterface::<char>::array_from_expr(expr).unwrap(),
            vec!['a', 'b', 'c']
        );
    }

    #[test]
    fn test_expr_c_char() {
        let expr = syn::parse_quote! { c"abc" };
        assert_eq!(
            SynInterface::<::std::ffi::c_char>::array_from_expr(expr).unwrap(),
            vec![97, 98, 99, 0]
        );
    }
}
