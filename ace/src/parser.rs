use nom::bytes::complete::{tag, take_until};
use nom::character::complete::space0;
use nom::sequence::{delimited, tuple};
use nom::IResult;
pub fn parse(input: &str) -> IResult<&str, &str> {
    delimited(tuple((tag("[["), space0)), take_until("]]"), tuple((space0, tag("]]"))))(input)
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn it_parses_a_link() {
        let test_input: &str = "[[Gjgsdgfjhgasj]]";
        let x = parse(test_input);
        assert_eq!(x, Ok(("", "Gjgsdgfjhgasj")));
    }

    #[test]
    fn it_parses_a_link_with_surrounding_whitespace() {
        let test_input: &str = "[[ Apple HJhksahf ]]";
        let x = parse(test_input);
        assert_eq!(x, Ok(("", " Apple HJhksahf ")));
    }
}
