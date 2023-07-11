use nom::bytes::complete::{tag, take_until};
use nom::branch::alt;
use nom::character::complete::space0;
use nom::combinator::{map, rest};
use nom::sequence::{delimited, tuple};
use nom::multi::separated_list0;
use nom::IResult;
use nom_locate::LocatedSpan;

use crate::link::{Link, Text};

pub type Span<'a> = LocatedSpan<&'a str>;

pub fn parse(input: Span) -> IResult<Span, Vec<Link>> {
    delimited(parse_text, separated_list0(parse_text, parse_link), parse_text)(input)
}

pub fn parse_text(input: Span) -> IResult<Span, Text> {
    // consume everything but [[Text]]
    map(alt((take_until("[["), rest)), |s: Span| Text {
        line: s.location_line(),
        offset: s.location_offset(),
        column: s.get_utf8_column(),
        content: s.fragment()
    })(input)
}

pub fn parse_link(input: Span) -> IResult<Span, Link> {
    // consume [[    Text   ]]
    map(
        delimited(tuple((tag("[["), space0)), take_until("]]"), tag("]]")),
        |s: Span| Link {
            line: s.location_line(),
            offset: s.location_offset(),
            column: s.get_utf8_column(),
            content: s.fragment().trim_end()
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::link::{Link, Text};
    use super::Span;
    use super::{parse, parse_link, parse_text};

    #[test]
    fn it_parses_a_link() {
        let test_input: &str = "[[Apple]]";
        let x = parse_link(Span::new(test_input)).unwrap().1;
        assert_eq!(x, Link {
            line: 1,
            offset: 2,
            column: 3,
            content: "Apple"
        });
    }

    #[test]
    fn it_parses_a_link_with_surrounding_whitespace() {
        let test_input: Span = Span::new("[[ Apple Bananas ]]");
        let x = parse_link(test_input).unwrap().1;
        assert_eq!(x, Link {
            line: 1,
            offset: 3,
            column: 4,
            content: "Apple Bananas"
        });
    }

    #[test]
    fn it_parses_text_until_a_link() {
        let test_input: Span = Span::new("Test [[Link]] Input");

        assert_eq!(parse_text(test_input).unwrap().1, Text {
            line: 1,
            offset: 0,
            column: 1,
            content: "Test "
        })
    }

    #[test]
    fn it_parses_multiple_links_in_a_document() {
        let test_input: Span = Span::new("# Test Document

        This document has a couple of [[ References ]] to [[Other Documents]] and [[More Information]]

        Some of these [[References  ]] will be [[Useful References]], others will not.
        ");

        let out: Vec<Link> = parse(test_input).unwrap().1;
        let expected: Vec<Link> = vec!(Link {
            line: 3,
            offset: 58,
            column: 42,
            content: "References"
        }, Link {
            line: 3,
            offset: 77,
            column: 61,
            content: "Other Documents"
        }, Link {
            line: 3,
            offset: 101,
            column: 85,
            content: "More Information"
        }, Link {
            line: 5,
            offset: 145,
            column: 25,
            content: "References"
        }, Link {
            line: 5,
            offset: 170,
            column: 50,
            content: "Useful References"
        });

        assert_eq!(out, expected);
    }
}
