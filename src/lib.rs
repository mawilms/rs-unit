#![warn(clippy::all, clippy::pedantic)]
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use self::keywords::Describe;

mod keywords;

#[proc_macro]
pub fn rs_unit(input: TokenStream) -> TokenStream {
    let describe = parse_macro_input!(input as Describe);
    //eprintln!("{:#?}", describe);

    let expanded = quote! {
        #[cfg(test)]
        mod tests {

        }
    };

    expanded.into()
}

// use rs_unit::rs_unit;

// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn sub(a: i32, b: i32) -> i32 {
//     a - b
// }

// fn main() {
//     rs_unit! {

//         describe "Test addition" {

//             setup "Create database" {

//             }

//             test "success: Add positive numbers" {
//                 let result = add(1,1);
//                 assert_eq!(result, 2);
//             }

//             test "success: Add negative numbers" {
//                 let result = add(-2, -2);
//                 assert_eq!(result, -4);
//             }

//             teardown "Delete database" {

//             }

//         }
//     }
// }
