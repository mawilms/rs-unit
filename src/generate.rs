//!
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::keywords::{Describe, Root, Setup, Teardown, Test};

pub trait Generate {
    fn generate(&self) -> TokenStream;
}

impl Generate for Root {
    fn generate(&self) -> TokenStream {
        let ident = &self.ident;

        let describe_blocks = self
            .describes
            .iter()
            .map(|d| d.generate())
            .collect::<Vec<_>>();

        eprintln!("{:#?}", self.describes);

        let root_block = quote! {
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

impl Generate for Describe {
    fn generate(&self) -> TokenStream {
        let ident = &self.ident;
        let tests = &self.tests.iter().map(|t| t.generate()).collect::<Vec<_>>();

        let describe_block = quote! {
            mod #ident {
                use super::*;

                #(#tests)*
            }
        };

        describe_block
    }
}

/// Generates a valid Rust test function.
///
/// # Example
///
/// ```rust
/// #[test]
/// fn success_add_positive_numbers() {
///   let result = add(1,1);
///   assert_eq!(result, 2);
/// }
impl Generate for Test {
    fn generate(&self) -> TokenStream {
        let sanitied_name = &format!("test_{}", self.name)
            .to_lowercase()
            .replace(" ", "_")
            .replace(":", "");
        let new_ident = Ident::new(sanitied_name, self.ident.span());

        let block = &self.content;

        let stream = quote! {
            #[test]
            fn #new_ident() {
                #block
            }
        };
        //eprintln!("{:#?}", stream);

        stream
    }
}

impl Generate for Setup {
    fn generate(&self) -> TokenStream {
        todo!()
    }
}

impl Generate for Teardown {
    fn generate(&self) -> TokenStream {
        todo!()
    }
}
