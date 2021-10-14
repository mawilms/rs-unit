//! `RsUnit` is a unit testing framework for Rust. It's a wrapper around the native `cargo test` interface.
//! `RsUnit` mimics the structure and behavior of [`ExUnit`](https://hexdocs.pm/ex_unit/1.12/ExUnit.html).
//!
//! - **Easy to use** Import the macro and build organized unit tests with the `describe` and `test` blocks.
//! - ** Simple testing setup** Create with `setup`, `setup_all`, `teardown` and `teardown_all` functions that
//! are run once or before every test and keep the rest of your tests organized in blocks.
//!
//! # Example
//!
//! ```rust
//! use rs_unit::rs_unit;
//!
//! fn add(a: i32, b: i32) -> i32 {
//!     a + b
//! }
//!
//! rs_unit! {
//!     describe "Addition" {
//!         test "success: Add positive numbers" {
//!             let result = add(1,1);
//!             assert_eq!(result, 2);
//!         }
//!
//!        test "success: Add negative numbers" {
//!             let result = add(-2, -2);
//!             assert_eq!(result, -4);
//!         }
//!     }
//! }
//! ```
#![warn(clippy::all, clippy::pedantic)]
use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::generate::Generate;

use self::keywords::Root;

mod generate;
mod keywords;

#[proc_macro]
pub fn rs_unit(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as Root);
    let code = root.generate();

    code.into()
}
