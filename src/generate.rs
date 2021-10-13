//! Contains methods to generate the manipulated `Tokenstreams` based of the parsed AST.
use proc_macro2::{Ident, TokenStream};
use quote::quote_spanned;

use crate::keywords::{Describe, Root, Setup, Teardown, Test};

pub trait Generate {
    fn generate(&self) -> TokenStream;

    fn generate_test(&self, _setup: &TokenStream, _teardown: &TokenStream) -> TokenStream {
        TokenStream::new()
    }
}

/// Generates the outer wrapper test wrapper.
///
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     Here are the describe blocks
/// }
/// ```
impl Generate for Root {
    fn generate(&self) -> TokenStream {
        let ident = &self.ident;

        let describe_blocks = self
            .describes
            .iter()
            .map(|d| d.generate())
            .collect::<Vec<_>>();

        let root_block = quote_spanned! {ident.span()=>
            #[cfg(test)]
            mod #ident {
                #[allow(unused_imports)]
                use super::*;

                #(#describe_blocks)*
            }
        };

        root_block
    }
}

/// Generates a module block that groups related tests. These modules are located in the `Root` block.
///
/// ```rust
/// mod add_numbers {
///     Here are your tests
/// }
/// ```
impl Generate for Describe {
    fn generate(&self) -> TokenStream {
        let ident = &self.ident;
        let setup = &self.setup;
        let teardown = &self.teardown;
        let tests = &self
            .tests
            .iter()
            .map(|t| t.generate_test(setup, teardown))
            .collect::<Vec<_>>();

        let describe_block = quote_spanned! {ident.span()=>
            mod #ident {
                use super::*;

                #(#tests)*
            }
        };

        describe_block
    }
}

/// Generates a valid Rust test function. These function are located within the modules where they belong to.
///
/// # Example
///
/// ```rust
/// #[test]
/// fn success_add_positive_numbers() {
///   let result = add(1,1);
///   assert_eq!(result, 2);
/// }
/// ```
impl Generate for Test {
    fn generate(&self) -> TokenStream {
        let sanitied_name = &self
            .name
            .to_string()
            .to_lowercase()
            .replace(" ", "_")
            .replace(":", "");
        let new_ident = Ident::new(sanitied_name, self.ident.span());

        let block = &self.content;

        let test_block = quote_spanned! {new_ident.span()=>
            #[test]
            fn #new_ident() {
                #block
            }
        };

        test_block
    }

    fn generate_test(&self, setup: &TokenStream, teardown: &TokenStream) -> TokenStream {
        let sanitied_name = &self
            .name
            .to_string()
            .to_lowercase()
            .replace(" ", "_")
            .replace(":", "");
        let new_ident = Ident::new(sanitied_name, self.ident.span());

        let block = &self.content;

        let test_block = quote_spanned! {new_ident.span()=>
            #[test]
            fn #new_ident() {
                #setup

                #block

                #teardown
            }
        };

        test_block
    }
}

impl Generate for Setup {
    fn generate(&self) -> TokenStream {
        let ident = &self.ident;
        let block = &self.content;

        let setup_block = quote_spanned! (ident.span()=> #block);

        setup_block
    }
}

impl Generate for Teardown {
    fn generate(&self) -> TokenStream {
        let ident = &self.ident;
        let block = &self.content;

        let teardown_block = quote_spanned! (ident.span()=> #block);

        teardown_block
    }
}
