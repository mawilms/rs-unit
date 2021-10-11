use syn::{
    braced,
    parse::{Parse, ParseStream},
    Ident, LitStr, Result,
};

mod kw {
    syn::custom_keyword!(describe);
    syn::custom_keyword!(setup);
    syn::custom_keyword!(test);
    syn::custom_keyword!(teardown);
}

#[derive(Debug)]
pub struct Describe {
    name: String,
    braces: syn::token::Brace,
    setup: Vec<Setup>,
    tests: Vec<Test>,
    teardown: Vec<Teardown>,
}

impl syn::parse::Parse for Describe {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _describe = input.parse::<kw::describe>()?;
        let name = input.parse::<syn::LitStr>()?.value();

        let content;
        let braces = syn::braced!(content in input);

        let mut setup = Vec::<Setup>::new();
        while content.peek(kw::setup) {
            setup.push(content.parse()?);
        }

        let mut tests = Vec::<Test>::new();

        while !content.is_empty() {
            eprintln!("{:#?}", content);
            tests.push(content.parse()?);
        }
        //eprintln!("{:#?}", tests);

        let mut teardown = Vec::<Teardown>::new();
        while content.peek(kw::teardown) {
            teardown.push(content.parse()?);
        }

        Ok(Self {
            name,
            braces,
            setup,
            tests,
            teardown,
        })
    }
}

#[derive(Debug)]
struct Test {
    name: String,
    content: syn::Block,
}

impl syn::parse::Parse for Test {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _test = input.parse::<kw::test>()?;
        let name = input.parse::<syn::LitStr>()?.value();
        Ok(Self {
            name,
            content: input.parse::<syn::Block>()?,
        })
    }
}

#[derive(Debug)]
struct Setup {
    name: String,
    content: syn::Block,
}

impl syn::parse::Parse for Setup {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _test = input.parse::<kw::setup>()?;
        let name = input.parse::<syn::LitStr>()?.value();
        Ok(Self {
            name,
            content: input.parse::<syn::Block>()?,
        })
    }
}

#[derive(Debug)]
struct Teardown {
    name: String,
    content: syn::Block,
}

impl syn::parse::Parse for Teardown {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _test = input.parse::<kw::teardown>()?;
        let name = input.parse::<syn::LitStr>()?.value();
        Ok(Self {
            name,
            content: input.parse::<syn::Block>()?,
        })
    }
}
