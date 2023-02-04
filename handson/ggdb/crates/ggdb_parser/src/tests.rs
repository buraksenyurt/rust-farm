#[cfg(test)]
mod tests {
    use crate::create_command::CreateStatement;
    use crate::delete_command::DeleteWithWhereStatement;
    use crate::insert_command::InsertStatement;
    use crate::query::Query;
    use crate::select_command::SelectWithWhereStatement;
    use crate::{Column, ColumnValue, DbType, Parse, WhereColumn};

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

    #[test]
    fn select_query_test() {
        let expected = SelectWithWhereStatement {
            table: "Customer".to_string(),
            fields: vec!["title".to_string(), "email".to_string()],
            where_columns: vec![WhereColumn {
                name: "id".to_string(),
                value: "1234".to_string(),
            }],
        };
        let actual = Query::parse_from_raw("SELECT title,email from Customer Where id=1234;")
            .unwrap()
            .1;
        assert_eq!(actual, Query::Select(expected));
    }

    #[test]
    fn insert_query_test() {
        let expected = InsertStatement {
            table: "Product".to_string(),
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
        let actual = Query::parse_from_raw("INSERT INTO Product VALUES (id:1,title:CdBox,price:25);")
            .unwrap()
            .1;
        assert_eq!(actual, Query::Insert(expected));
    }
}