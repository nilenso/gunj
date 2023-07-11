#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token<T> {
    pub line: u32,
    pub offset: usize,
    pub column: usize,
    pub content: T,
}

pub type Link<'a> = Token<&'a str>;
pub type Text<'a> = Token<&'a str>;

