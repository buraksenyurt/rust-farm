pub mod create_command;
pub mod delete_command;
pub mod insert_command;
pub mod select_command;
pub mod query;

use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take_while1};
use nom::character::complete::{char, multispace0, multispace1};
use nom::combinator::map;
use nom::error::context;
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, tuple};
use nom::IResult;
use nom_locate::LocatedSpan;
use nom_supreme::ParserExt;
use serde::{Deserialize, Serialize};

pub type RawSpan<'a> = LocatedSpan<&'a str>;
pub type ParseResult<'a, T> = IResult<RawSpan<'a>, T>;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_command::CreateStatement;
    use crate::delete_command::DeleteWithWhereStatement;
    use crate::insert_command::InsertStatement;
    use crate::select_command::SelectWithWhereStatement;

    #[test]
    fn create_table_test() {
        let actual = CreateStatement::parse_from_raw(
            "CREATE TABLE Product (id int,title string,price decimal)",
        )
        .unwrap()
        .1;
        let expected = CreateStatement {
            table: "Product".into(),
            columns: vec![
                Column {
                    name: "id".into(),
                    db_type: DbType::Int,
                },
                Column {
                    name: "title".into(),
                    db_type: DbType::String,
                },
                Column {
                    name: "price".into(),
                    db_type: DbType::Decimal,
                },
            ],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn column_value_separator_test() {
        let actual = ColumnValue::parse_from_raw("price:25").unwrap().1;
        let expected = ColumnValue {
            name: "price".into(),
            value: "25".into(),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn insert_into_test() {
        let actual = InsertStatement::parse_from_raw(
            "insert into Product values (id:1,title:CdBox,price:25)",
        )
        .unwrap()
        .1;
        let expected = InsertStatement {
            table: "Product".into(),
            columns: vec![
                ColumnValue {
                    name: "id".into(),
                    value: "1".into(),
                },
                ColumnValue {
                    name: "title".into(),
                    value: "CdBox".into(),
                },
                ColumnValue {
                    name: "price".into(),
                    value: "25".into(),
                },
            ],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn single_where_column_test() {
        let actual = WhereColumn::parse_from_raw("id=1234").unwrap().1;
        let expected = WhereColumn {
            name: "id".into(),
            value: "1234".into(),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn select_with_where_test() {
        let actual = SelectWithWhereStatement::parse_from_raw(
            "SELECT id,title,price FROM Product WHERE id=1001,category=3",
        )
        .unwrap()
        .1;
        let expected = SelectWithWhereStatement {
            table: "Product".into(),
            fields: vec!["id".into(), "title".into(), "price".into()],
            where_columns: vec![
                WhereColumn {
                    name: "id".into(),
                    value: "1001".into(),
                },
                WhereColumn {
                    name: "category".into(),
                    value: "3".into(),
                },
            ],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn delete_with_where_test() {
        let actual = DeleteWithWhereStatement::parse_from_raw(
            "DELETE FROM Product WHERE id=1001,category=3",
        )
        .unwrap()
        .1;
        let expected = DeleteWithWhereStatement {
            table: "Product".into(),
            where_columns: vec![
                WhereColumn {
                    name: "id".into(),
                    value: "1001".into(),
                },
                WhereColumn {
                    name: "category".into(),
                    value: "3".into(),
                },
            ],
        };
        assert_eq!(actual, expected);
    }
}
