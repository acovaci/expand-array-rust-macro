use super::interface::SynInterface;
use super::traits::TypeConverter;
use crate::types::RustType;

impl TypeConverter<RustType> for SynInterface<RustType> {
    fn from_typ(typ: syn::Type) -> syn::Result<RustType> {
        let path = match typ {
            syn::Type::Path(syn::TypePath {
                path: syn::Path { segments, .. },
                ..
            }) => segments,
            _ => {
                return Err(syn::Error::new_spanned(
                    typ.clone(),
                    format!(
                        "Invalid type. Expected a path, but got {:?} instead",
                        quote::quote! { #typ }.to_string()
                    ),
                ))
            }
        };

        let segments = path
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect();

        Ok(RustType::new(segments))
    }
}
