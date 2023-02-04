use crate::{
   identifier, where_definitions, Parse,
    ParseResult, RawSpan, WhereColumn,
};
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::tuple;
use nom_supreme::ParserExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct DeleteWithWhereStatement {
    pub table: String,
    pub where_columns: Vec<WhereColumn>,
}

// delete from [table_name] where id=1 örneği
impl<'a> Parse<'a> for DeleteWithWhereStatement {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        map(
            tuple((
                tag_no_case("delete"),
                multispace1,
                tag_no_case("from"),
                multispace1,
                identifier.context("From Table"),
                multispace1,
                tag_no_case("where"),
                multispace1,
                where_definitions,
            ))
            .context("Delete With Where Statement"),
            |(_, _, _, _, table, _, _, _, where_columns)| Self {
                table,
                where_columns,
            },
        )(input)
    }
}
