use crate::{column_value_definitions, identifier, ColumnValue, Parse, ParseResult, RawSpan};
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom_supreme::ParserExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InsertStatement {
    pub table: String,
    pub columns: Vec<ColumnValue>,
}

// insert into [tablo adı] ([kolon adı]:[değeri],[kolon adı]:[değeri]) parse işlemi için
impl<'a> Parse<'a> for InsertStatement {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        map(
            tuple((
                tag_no_case("insert"),
                preceded(multispace1, tag_no_case("into")),
                preceded(multispace1, identifier.context("Table Name")),
                preceded(multispace1, tag_no_case("values")),
                preceded(multispace1, column_value_definitions),
            ))
            .context("Insert Into Statement"),
            |(_, _, table, _, columns)| Self { table, columns },
        )(input)
    }
}
