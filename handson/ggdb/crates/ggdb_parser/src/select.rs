use crate::{comma_sep, identifier, Parse, ParseResult, RawSpan};
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::tuple;
use nom_supreme::ParserExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SelectStatement {
    pub table: String,
    pub fields: Vec<String>,
}

// Select [kolon adları] from [tablo adı] where [alan adi]=[alan değeri] için
impl<'a> Parse<'a> for SelectStatement {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        map(
            tuple((
                tag_no_case("select"),
                multispace1,
                comma_sep(identifier).context("Select Columns"),
                multispace1,
                tag_no_case("from"),
                multispace1,
                identifier.context("From Table"),
            ))
            .context("Select Statement"),
            |(_, _, fields, _, _, _, table)| Self { table, fields },
        )(input)
    }
}
