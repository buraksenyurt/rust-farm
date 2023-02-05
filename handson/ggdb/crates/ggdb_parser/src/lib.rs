pub mod create;
pub mod delete;
pub mod error;
pub mod insert;
pub mod query;
pub mod select;
pub mod select_where;
pub mod tests;

use crate::error::{format_parse_error, FormatError, QueryParseError};
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take_while1};
use nom::character::complete::{char, multispace0, multispace1};
use nom::combinator::{all_consuming, map};
use nom::error::context;
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, tuple};
use nom::{Finish, IResult};
use nom_locate::LocatedSpan;
use nom_supreme::ParserExt;
use serde::{Deserialize, Serialize};

pub type RawSpan<'a> = LocatedSpan<&'a str>;
pub type ParseResult<'a, T> = IResult<RawSpan<'a>, T, QueryParseError<'a>>;

pub(crate) fn identifier(i: RawSpan) -> ParseResult<String> {
    map(take_while1(|c: char| c.is_alphanumeric()), |s: RawSpan| {
        s.fragment().to_string()
    })(i)
}

pub(crate) fn comma_sep<'a, O, E, F>(
    f: F,
) -> impl FnMut(RawSpan<'a>) -> IResult<RawSpan<'a>, Vec<O>, E>
where
    F: nom::Parser<RawSpan<'a>, O, E>,
    E: nom::error::ParseError<RawSpan<'a>>,
{
    separated_list1(tuple((multispace0, char(','), multispace0)), f)
}

pub trait Parse<'a>: Sized {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self>;

    fn parse_from_raw(input: &'a str) -> ParseResult<'a, Self> {
        let i = LocatedSpan::new(input);
        Self::parse(i)
    }

    fn parse_format_error(i: &'a str) -> Result<Self, FormatError<'a>> {
        let input = LocatedSpan::new(i);
        match all_consuming(Self::parse)(input).finish() {
            Ok((_, query)) => Ok(query),
            Err(e) => Err(format_parse_error(i, e)),
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum DbType {
    String,
    Int,
    Decimal,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub db_type: DbType,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ColumnValue {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct WhereColumn {
    pub name: String,
    pub value: String,
}

// string ve int veri türlerinin parse işlemi için
impl<'a> Parse<'a> for DbType {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        context(
            "Column Type",
            alt((
                map(tag_no_case("int"), |_| Self::Int),
                map(tag_no_case("string"), |_| Self::String),
                map(tag_no_case("decimal"), |_| Self::Decimal),
            )),
        )(input)
    }
}

// tablonun [kolon adı] [veri tipi] parse işlemi için
impl<'a> Parse<'a> for Column {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        context(
            "Create Column",
            map(
                separated_pair(
                    identifier.context("Column Name"),
                    multispace1,
                    DbType::parse,
                ),
                |(name, db_type)| Self { name, db_type },
            ),
        )(input)
    }
}

// insert ifadesindeki [kolon adı]:[value], kısımlarını parse etmek için
impl<'a> Parse<'a> for ColumnValue {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        context(
            "Column Value",
            map(
                separated_pair(
                    identifier.context("Column Name"),
                    tag(":"),
                    identifier.context("Value"),
                ),
                |(name, value)| Self { name, value },
            ),
        )(input)
    }
}

impl<'a> Parse<'a> for WhereColumn {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        context(
            "Where Column",
            map(
                separated_pair(
                    identifier.context("Column Name"),
                    tag("="),
                    identifier.context("Value"),
                ),
                |(name, value)| Self { name, value },
            ),
        )(input)
    }
}

fn column_definitions(input: RawSpan<'_>) -> ParseResult<'_, Vec<Column>> {
    context(
        "Column Definitions",
        map(
            tuple((char('('), comma_sep(Column::parse), char(')'))),
            |(_, cols, _)| cols,
        ),
    )(input)
}

fn column_value_definitions(input: RawSpan<'_>) -> ParseResult<'_, Vec<ColumnValue>> {
    context(
        "Column Value Definitions",
        map(
            tuple((char('('), comma_sep(ColumnValue::parse), char(')'))),
            |(_, cols, _)| cols,
        ),
    )(input)
}

fn where_definitions(input: RawSpan<'_>) -> ParseResult<'_, Vec<WhereColumn>> {
    context(
        "Where Definitions",
        map(comma_sep(WhereColumn::parse), |cols| cols),
    )(input)
}
