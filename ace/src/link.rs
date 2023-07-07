#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token<T> {
    pub line: u32,
    pub offset: usize,
    pub column: usize,
    pub content: T,
}

pub type Link<'a> = Token<&'a String>;
pub type Text<'a> = Token<&'a String>;

