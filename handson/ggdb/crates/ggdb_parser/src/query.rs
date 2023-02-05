use crate::create::CreateStatement;
use crate::delete::DeleteWithWhereStatement;
use crate::error::FormatError;
use crate::insert::InsertStatement;
use crate::select::SelectStatement;
use crate::select_where::SelectWithWhereStatement;
use crate::{Parse, ParseResult, RawSpan};
use nom::branch::alt;
use nom::character::complete::{char, multispace0};
use nom::combinator::map;
use nom::error::context;
use nom::sequence::{preceded, tuple};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Query {
    Create(CreateStatement),
    Insert(InsertStatement),
    Delete(DeleteWithWhereStatement),
    SelectWhere(SelectWithWhereStatement),
    Select(SelectStatement),
}

impl<'a> Parse<'a> for Query {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        let (rest, (query, _, _, _)) = context(
            "Query",
            preceded(
                multispace0,
                tuple((
                    alt((
                        map(CreateStatement::parse, Query::Create),
                        map(InsertStatement::parse, Query::Insert),
                        map(DeleteWithWhereStatement::parse, Query::Delete),
                        map(SelectWithWhereStatement::parse, Query::SelectWhere),
                        map(SelectStatement::parse, Query::Select),
                    )),
                    multispace0,
                    char(';'),
                    multispace0,
                )),
            ),
        )(input)?;
        Ok((rest, query))
    }
}

impl<'a> TryFrom<&'a str> for Query {
    type Error = FormatError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match Query::parse_format_error(value) {
            Ok(query) => Ok(query),
            Err(e) => Err(e),
        }
    }
}

pub fn parse_query(input: &str) -> Result<Query, FormatError<'_>> {
    input.try_into()
}
