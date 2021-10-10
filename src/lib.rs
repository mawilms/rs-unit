#![warn(clippy::all, clippy::pedantic)]

use proc_macro::TokenStream;

#[proc_macro]
pub fn rs_unit(input: TokenStream) -> TokenStream {
    input
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
