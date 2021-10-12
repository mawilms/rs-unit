use syn::{
    braced,
    parse::{Parse, ParseStream},
    token::Brace,
    Block, Ident, LitStr, Result,
};

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
            ident: input.parse::<Ident>()?,
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
        let _test = input.parse::<kw::test>()?;
        let name = input.parse::<LitStr>()?.value();
        Ok(Self {
            ident: input.parse::<Ident>()?,
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
