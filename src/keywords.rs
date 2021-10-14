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
    custom_keyword!(test);
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

        let contents;
        let _braces = braced!(contents in input);

        let mut setup = None::<Setup>;
        let mut teardown = None::<Teardown>;
        let mut tests = Vec::<Test>::new();
        while !contents.is_empty() {
            let snoopy = contents.lookahead1();
            if snoopy.peek(kw::setup) {
                let prev = setup.replace(contents.parse()?);
                if prev.is_some() {
                    return Err(contents.error("At most one `setup` can be provided"));
                }
            } else if snoopy.peek(kw::teardown) {
                let prev = teardown.replace(contents.parse()?);
                if prev.is_some() {
                    return Err(contents.error("At most one `teardown` can be provided"));
                }
            } else if snoopy.peek(kw::test) {
                tests.push(contents.parse()?);
            } else {
                return Err(snoopy.error());
            }
        }

        let mut setup_stream = TokenStream::new();
        if let Some(setup) = setup {
            setup_stream = setup.generate();
        }

        let mut teardown_stream = TokenStream::new();
        if let Some(teardown) = teardown {
            teardown_stream = teardown.generate();
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
