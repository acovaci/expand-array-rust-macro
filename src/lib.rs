extern crate proc_macro;

mod input;
mod parse;
mod types;

#[proc_macro]
pub fn arrr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let expanded = _arrr(input).unwrap_or_else(|err| err.to_compile_error());
    proc_macro::TokenStream::from(expanded)
}

fn _arrr(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let input = syn::parse2::<input::Input>(input)?;
    input.build_array()
}
