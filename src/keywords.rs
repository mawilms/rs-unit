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
    custom_keyword!(setup_all);
    custom_keyword!(test);
    custom_keyword!(teardown);
    custom_keyword!(teardown_all);
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
    pub setup_all: TokenStream,
    pub setup: TokenStream,
    pub tests: Vec<Test>,
    pub teardown_all: TokenStream,
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

        let mut setup_all = None::<SetupAll>;
        let mut setup = None::<Setup>;
        //let mut teardown_all = None::<TeardownAll>;
        let mut teardown = None::<Teardown>;
        let mut tests = Vec::<Test>::new();
        while !contents.is_empty() {
            let look_ahead = contents.lookahead1();
            if look_ahead.peek(kw::setup_all) {
                let prev = setup_all.replace(contents.parse()?);
                if prev.is_some() {
                    return Err(contents.error("At most one `setup_all` can be provided"));
                }
            } else if look_ahead.peek(kw::setup) {
                let prev = setup.replace(contents.parse()?);
                if prev.is_some() {
                    return Err(contents.error("At most one `setup` can be provided"));
                }
            } else if look_ahead.peek(kw::teardown) {
                let prev = teardown.replace(contents.parse()?);
                if prev.is_some() {
                    return Err(contents.error("At most one `teardown` can be provided"));
                }
            } else if look_ahead.peek(kw::test) {
                tests.push(contents.parse()?);
            } else {
                return Err(look_ahead.error());
            }
            // else if look_ahead.peek(kw::teardown_all) {
            //     let prev = teardown_all.replace(contents.parse()?);
            //     if prev.is_some() {
            //         return Err(contents.error("At most one `teardown_all` can be provided"));
            //     }
            // }
        }

        let mut setup_all_stream = TokenStream::new();
        if let Some(setup_all) = setup_all {
            setup_all_stream = setup_all.generate();
        }

        let mut setup_stream = TokenStream::new();
        if let Some(setup) = setup {
            setup_stream = setup.generate();
        }

        //let mut teardown_all_stream = TokenStream::new();
        // if let Some(teardown_all) = teardown_all {
        //     teardown_all_stream = teardown_all.generate();
        // }

        let mut teardown_stream = TokenStream::new();
        if let Some(teardown) = teardown {
            teardown_stream = teardown.generate();
        }

        Ok(Self {
            ident: Ident::new(&name, ident.span()),
            setup_all: setup_all_stream,
            setup: setup_stream,
            tests,
            teardown_all: TokenStream::new(),
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

// Setup block that is converted to a setup function.
#[derive(Debug, Clone)]
pub struct Setup {
    pub ident: Ident,
    pub content: Block,
}

// Parses a setup block.
//
// # Example
//
// ```
// setup {
//   some_setup_logic();
// }
// ```
impl Parse for Setup {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;

        Ok(Self {
            ident,
            content: input.parse::<Block>()?,
        })
    }
}

// Setup block that is converted to a teardown function.
#[derive(Debug, Clone)]
pub struct Teardown {
    pub ident: Ident,
    pub content: Block,
}

// Parses a teardown block.
//
// # Example
//
// ```
// teardown {
//   some_teardown_logic();
// }
// ```
impl Parse for Teardown {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;

        Ok(Self {
            ident,
            content: input.parse::<Block>()?,
        })
    }
}

// Setup_all block that is converted to a teardown function.
#[derive(Debug, Clone)]
pub struct SetupAll {
    pub ident: Ident,
    pub content: Block,
}

// Parses a setup_all block.
//
// # Example
//
// ```
// setup_all {
//   run_logic_once();
// }
// ```
impl Parse for SetupAll {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;

        Ok(Self {
            ident,
            content: input.parse::<Block>()?,
        })
    }
}

// Teardown_all block that is converted to a teardown function.
#[derive(Debug, Clone)]
pub struct TeardownAll {
    pub ident: Ident,
    pub content: Block,
}

// Parses a teardown_all block.
//
// # Example
//
// ```
// teardown_all {
//   run_logic_once();
// }
// ```
impl Parse for TeardownAll {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;

        Ok(Self {
            ident,
            content: input.parse::<Block>()?,
        })
    }
}
