pub trait TypeConverter<T> {
    fn from_typ(typ: syn::Type) -> syn::Result<T>;
}

pub trait LiteralConverter<T> {
    fn value_from_lit(lit: syn::ExprLit) -> syn::Result<T>;

    fn value_from_expr(expr: syn::Expr) -> syn::Result<T> {
        match expr {
            syn::Expr::Lit(lit) => Self::value_from_lit(lit),
            _ => Err(syn::Error::new_spanned(
                expr.clone(),
                format!(
                    "Invalid value. Expected a literal for type {} but got {:?} instead",
                    ::std::any::type_name::<T>(),
                    quote::quote! { #expr }.to_string()
                ),
            )),
        }
    }

    fn array_from_array(array: syn::ExprArray) -> syn::Result<Vec<T>> {
        let syn::ExprArray { elems, .. } = array.clone();
        elems
            .into_iter()
            .map(Self::value_from_expr)
            .collect::<syn::Result<Vec<T>>>()
    }
}

pub trait ArrayLiteralConverter<T> {
    fn array_from_lit(lit: syn::ExprLit) -> syn::Result<Vec<T>>;

    fn array_from_expr(expr: syn::Expr) -> syn::Result<Vec<T>> {
        match expr {
            syn::Expr::Lit(lit) => Self::array_from_lit(lit),
            _ => Err(syn::Error::new_spanned(
                expr.clone(),
                format!(
                    "Invalid value. Expected a literal or array for type {} but got {:?} instead",
                    ::std::any::type_name::<T>(),
                    quote::quote! { #expr }.to_string()
                ),
            )),
        }
    }
}

impl<T, U> ArrayLiteralConverter<T> for U
where
    U: LiteralConverter<Vec<T>> + LiteralConverter<T>,
{
    fn array_from_lit(lit: syn::ExprLit) -> syn::Result<Vec<T>> {
        Self::value_from_lit(lit)
    }

    fn array_from_expr(expr: syn::Expr) -> syn::Result<Vec<T>> {
        match expr {
            syn::Expr::Lit(lit) => Self::array_from_lit(lit),
            syn::Expr::Array(array) => Self::array_from_array(array),
            _ => Err(syn::Error::new_spanned(
                expr.clone(),
                format!(
                    "Invalid value. Expected a literal or array for type {} but got {:?} instead",
                    ::std::any::type_name::<T>(),
                    quote::quote! { #expr }.to_string()
                ),
            )),
        }
    }
}

pub trait Tokenizer<T> {
    fn to_tokens(value: T) -> proc_macro2::TokenStream;
    fn all_to_tokens(values: Vec<T>) -> proc_macro2::TokenStream {
        let tokens = values.into_iter().map(Self::to_tokens);
        quote::quote! { [#(#tokens),*] }
    }
}

pub trait IntoTokens {
    fn into_tokens(self) -> proc_macro2::TokenStream;
}

pub trait TryIntoTokens {
    fn try_into_tokens(self) -> syn::Result<proc_macro2::TokenStream>;
}

impl<T> TryIntoTokens for T
where
    T: IntoTokens,
{
    fn try_into_tokens(self) -> syn::Result<proc_macro2::TokenStream> {
        Ok(self.into_tokens())
    }
}
