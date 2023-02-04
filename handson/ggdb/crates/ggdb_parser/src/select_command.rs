use crate::{comma_sep, identifier, where_definitions, Parse, ParseResult, RawSpan, WhereColumn};
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::tuple;
use nom_supreme::ParserExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SelectWithWhereStatement {
    pub table: String,
    pub fields: Vec<String>,
    pub where_columns: Vec<WhereColumn>,
}

// Select [kolon adları] from [tablo adı] where [alan adi]=[alan değeri] için
impl<'a> Parse<'a> for SelectWithWhereStatement {
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
                multispace1,
                tag_no_case("where"),
                multispace1,
                where_definitions,
            ))
            .context("Select With Where Statement"),
            |(_, _, fields, _, _, _, table, _, _, _, where_columns)| Self {
                table,
                fields,
                where_columns,
            },
        )(input)
    }
}
