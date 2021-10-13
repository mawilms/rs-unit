use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::keywords::{Root, Setup, Teardown, Test};

pub trait Generate {
    fn generate(&self) -> TokenStream;
}

impl Generate for Root {
    fn generate(&self) -> TokenStream {
        let ident = &self.ident;
        let tests = &self.tests.iter().map(|t| t.generate()).collect::<Vec<_>>();

        let root_block = quote! {
            mod #ident {
                #[allow(unused_imports)]
                use super::*;

                #(#tests)*
            }
        };
        //eprintln!("{:#?}", root_block);

        root_block
    }
}

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
