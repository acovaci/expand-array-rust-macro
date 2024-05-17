mod arr;
mod builder;
mod converter;
mod definition;
mod tokenizer;
mod traits;
mod types;

pub mod interface {
    pub use super::traits::{ArrayLiteralConverter, IntoTokens, Tokenizer, TypeConverter};

    pub struct SynInterface<T> {
        _marker: std::marker::PhantomData<T>,
    }
}

pub mod array {
    pub use super::builder::{ArrayBuilder, ArrayTokenizer};
}
