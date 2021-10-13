use syn::{
    braced,
    parse::{Parse, ParseStream},
    token::Brace,
    Block, Ident, LitStr, Result,
};

use std::{
    process::id,
    sync::atomic::{AtomicUsize, Ordering},
};

static GLOBAL_RSUNIT_COUNT: AtomicUsize = AtomicUsize::new(0);

fn get_root_name() -> proc_macro2::Ident {
    let count = GLOBAL_RSUNIT_COUNT.fetch_add(1, Ordering::SeqCst);
    let module_name = format!("rsunit_{}", count);

    syn::Ident::new(&module_name, proc_macro2::Span::call_site())
}

mod kw {
    use syn::custom_keyword;

    custom_keyword!(describe);
    custom_keyword!(setup);
    custom_keyword!(test);
    custom_keyword!(teardown);
}

#[derive(Debug)]
pub struct Root {
    pub ident: Ident,
    pub name: String,
    braces: Brace,
    pub setup: Vec<Setup>,
    pub tests: Vec<Test>,
    pub teardown: Vec<Teardown>,
}

impl Parse for Root {
    fn parse(input: ParseStream) -> Result<Self> {
        let _describe = input.parse::<kw::describe>()?;
        let name = input.parse::<LitStr>()?.value();

        let content;
        let braces = braced!(content in input);

        let mut setup = Vec::<Setup>::new();
        while content.peek(kw::setup) {
            setup.push(content.parse()?);
        }

        let mut tests = Vec::<Test>::new();

        while !content.is_empty() {
            tests.push(content.parse()?);
        }

        let mut teardown = Vec::<Teardown>::new();
        while content.peek(kw::teardown) {
            teardown.push(content.parse()?);
        }

        Ok(Self {
            ident: get_root_name(),
            name,
            braces,
            setup,
            tests,
            teardown,
        })
    }
}

#[derive(Debug)]
pub struct Test {
    pub ident: Ident,
    pub name: String,
    pub content: Block,
}

impl Parse for Test {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;
        let name = input.parse::<LitStr>()?.value();

        Ok(Self {
            ident,
            name,
            content: input.parse::<Block>()?,
        })
    }
}

#[derive(Debug)]
pub struct Setup {
    pub name: String,
    pub content: Block,
}

impl Parse for Setup {
    fn parse(input: ParseStream) -> Result<Self> {
        let _test = input.parse::<kw::setup>()?;
        let name = input.parse::<LitStr>()?.value();
        Ok(Self {
            name,
            content: input.parse::<Block>()?,
        })
    }
}

#[derive(Debug)]
pub struct Teardown {
    pub name: String,
    pub content: Block,
}

impl Parse for Teardown {
    fn parse(input: ParseStream) -> Result<Self> {
        let _test = input.parse::<kw::teardown>()?;
        let name = input.parse::<LitStr>()?.value();
        Ok(Self {
            name,
            content: input.parse::<Block>()?,
        })
    }
}
