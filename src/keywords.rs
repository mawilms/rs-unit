//! Contains all keywords that are used in `rs_unit`.
use proc_macro2::TokenStream;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    Block, Ident, LitStr, Result,
};

use crate::generate::Generate;

mod kw {
    use syn::custom_keyword;

    custom_keyword!(setup);
    custom_keyword!(teardown);
}

// Parsing entrypoint of the whole application.
#[derive(Debug)]
pub struct Root {
    pub ident: Ident,
    pub describes: Vec<Describe>,
}

// Parses all describe blocks within the `rs_unit!` macro.
impl Parse for Root {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut describes = Vec::<Describe>::new();

        while !input.is_empty() {
            describes.push(input.parse()?);
        }

        let ident = Ident::new("tests", proc_macro2::Span::call_site());

        Ok(Self { ident, describes })
    }
}

// Describe block that contains the actual tests and any pre- and postprocessing blocks.
#[derive(Debug)]
pub struct Describe {
    pub ident: Ident,
    pub setup: TokenStream,
    pub tests: Vec<Test>,
    pub teardown: TokenStream,
}

// Parses the Describe block. The pre- and postprocessing blocks are optional.
//
// # Example
//
// ```
// describe "Addition" {
//     Here are the actual test blocks
// }
impl Parse for Describe {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;
        let name = input
            .parse::<LitStr>()?
            .value()
            .to_lowercase()
            .replace(" ", "_")
            .replace("/", "_")
            .replace(":", "_");

        let content;
        let _braces = braced!(content in input);

        let mut setup = Vec::<Setup>::new();

        if content.peek(kw::setup) {
            setup.push(content.parse()?);
        }

        let mut teardown = Vec::<Teardown>::new();

        if content.peek(kw::teardown) {
            teardown.push(content.parse()?);
        }

        let mut tests = Vec::<Test>::new();

        while !content.is_empty() {
            tests.push(content.parse()?);
        }

        if setup.len() > 1 {
            panic!("More than one setup method");
        }

        if teardown.len() > 1 {
            panic!("More than one teardown method");
        }

        let mut setup_stream = TokenStream::new();
        if let Some(stream) = setup.into_iter().next() {
            setup_stream = stream.generate();
        }

        let mut teardown_stream = TokenStream::new();
        if let Some(stream) = teardown.into_iter().next() {
            teardown_stream = stream.generate();
        }

        Ok(Self {
            ident: Ident::new(&name, ident.span()),
            setup: setup_stream,
            tests,
            teardown: teardown_stream,
        })
    }
}

// Test block that is converted to a test function.
#[derive(Debug)]
pub struct Test {
    pub ident: Ident,
    pub name: String,
    pub content: Block,
}

// Parses a test block.
//
// # Example
//
// ```
// test "success: Add positive numbers" {
//   let result = add(1,1);
//   assert_eq!(result, 2);
// }
// ```
impl Parse for Test {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;
        let name = input
            .parse::<LitStr>()?
            .value()
            .to_lowercase()
            .replace(" ", "_")
            .replace(":", "");

        Ok(Self {
            ident,
            name,
            content: input.parse::<Block>()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Setup {
    pub ident: Ident,
    pub content: Block,
}

impl Parse for Setup {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;

        Ok(Self {
            ident,
            content: input.parse::<Block>()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Teardown {
    pub ident: Ident,
    pub content: Block,
}

impl Parse for Teardown {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;

        Ok(Self {
            ident,
            content: input.parse::<Block>()?,
        })
    }
}
