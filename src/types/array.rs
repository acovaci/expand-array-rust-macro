use super::RustType;

use crate::parse::interface::{SynInterface, TypeConverter};

#[derive(Debug, Clone)]
pub struct ArrayDefinition {
    size: usize,
    typ: RustType,
}

impl ArrayDefinition {
    pub fn new(size: usize, typ: syn::Type) -> syn::Result<Self> {
        Ok(Self {
            size,
            typ: SynInterface::<RustType>::from_typ(typ)?,
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn element_type(&self) -> &RustType {
        &self.typ
    }
}

#[derive(Debug, Clone)]
pub struct Array<T>
where
    T: Clone,
{
    values: Vec<T>,
}

impl<T> Array<T>
where
    T: Clone + Default,
{
    pub fn new(values: Vec<T>) -> Self {
        Self { values }
    }

    pub fn add(&mut self, value: T) {
        self.values.push(value);
    }

    pub fn check_size(&self, size: usize) -> bool {
        self.values.len() <= size
    }

    pub fn values(&self) -> Vec<T> {
        self.values.iter().cloned().collect()
    }

    pub fn ensure_size(&mut self, size: usize) -> syn::Result<()> {
        if !self.check_size(size) {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Invalid array size. Expected at most {} values", size),
            ));
        }

        if self.values.len() < size {
            self.values = self
                .values
                .iter()
                .chain(::std::iter::repeat(&Default::default()).take(size - self.values.len()))
                .cloned()
                .collect();
        }

        Ok(())
    }
}
