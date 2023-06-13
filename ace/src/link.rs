pub struct Link<'a> {
    pub line: u32,
    pub offset: u32,
    pub document: &'a str,
}
