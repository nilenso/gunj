use nom::bytes::complete::{tag, take_until};
use nom::character::complete::space0;
use nom::combinator::map;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use nom_locate::LocatedSpan;

use crate::link::Link;

pub fn parse(input: &str) -> IResult<&str, Link> {
    let span = LocatedSpan::new(input);

    let parsed_span = map(
        delimited(tuple((tag("[["), space0)), take_until("]]"), tag("]]")),
        |s: &str| s.trim_end(),
    )(&span);

    match parsed_span {
        Ok(("", parsed)) => Ok((
            "",
            Link {
                line: parsed.line,
                offset: parsed.offset,
                document: "blah",
            },
        )),
        Err(_) => todo!(),
    }
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
        let test_input: &str = "[[ Apple Bananas ]]";
        let x = parse(test_input);
        assert_eq!(x, Ok(("", "Apple Bananas")));
    }
}
