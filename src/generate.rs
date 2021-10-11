use proc_macro::TokenStream;

pub trait Generate {
    fn generate() -> TokenStream;
}
