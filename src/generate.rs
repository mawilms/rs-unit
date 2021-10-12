use proc_macro2::{Ident, TokenStream};
use quote::quote_spanned;

use crate::keywords::{Root, Setup, Teardown, Test};

pub trait Generate {
    fn generate(&self) -> TokenStream;
}

impl Generate for Root {
    fn generate(&self) -> TokenStream {
        let ident = &self.ident;
        let name = &self.name;
        let tests = &self.tests.iter().map(|t| t.generate()).collect::<Vec<_>>();

        let root_block = quote_spanned! ( ident.span()=>
            mod #name {
                #[allow(unused_imports)]
                use super::*;

                #(#tests)*
            }
        );

        root_block
    }
}

impl Generate for Test {
    fn generate(&self) -> TokenStream {
        let sanitied_name = &format!("test_{}", self.name)
            .to_lowercase()
            .replace(" ", "_")
            .replace(":", "");
        let name = Ident::new(sanitied_name, self.ident.span());
        let stmts = &self.content.stmts;

        let stream = quote_spanned!(name.span()=>
            #[test]
            fn #name() {
                #(#stmts)*
            }
        );
        stream
    }
    // #[test]
    //     fn add_positive_numbers() {
    //         let result = add(1, 1);
    //         assert_eq!(result, 2);
    //     }
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
