mod common;

extern crate notion_into_sqlite;

use serial_test::serial;
use std::collections::HashMap;
use std::fs;

use common::fixtures;
use notion_into_sqlite::notion_database::parse_database_schema;
use notion_into_sqlite::notion_pages::{NotionPage, NotionPropertyValue};
use notion_into_sqlite::sqlite::{Sqlite, PAGE_PROPERTIES_TABLE};
use std::error::Error;

static DATABASE_PATH: &str = "tmp/test.db";

fn setup(database_path: &str) {
    fs::remove_file(database_path).ok();
    fs::create_dir("tmp").ok();
}

#[test]
#[serial]
fn it_creates_table() -> Result<(), Box<dyn Error>> {
    setup(DATABASE_PATH);

    let schema = parse_database_schema(fixtures::NOTION_DATABASE_JSON)?;
    let sqlite = Sqlite::new(DATABASE_PATH, &schema)?;
    sqlite.create_tables()?;

    let (table_name, sql): (String, String) =
        sqlite
            .conn
            .query_row("SELECT name, sql FROM sqlite_master", [], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })?;
    assert_eq!(table_name, PAGE_PROPERTIES_TABLE);
    assert!(sql.contains(r#""Name" TEXT"#));
    assert!(sql.contains(r#""Animal" TEXT"#));
    assert!(sql.contains(r#""Age" REAL"#));
    Ok(())
}

#[test]
#[serial]
fn it_creates_table_when_column_name_includes_double_quote() -> Result<(), Box<dyn Error>> {
    setup(DATABASE_PATH);

    let schema = parse_database_schema(fixtures::NOTION_DATABASE_IRREGULAR_JSON)?;
    let sqlite = Sqlite::new(DATABASE_PATH, &schema)?;
    sqlite.create_tables()?;

    let (table_name, sql): (String, String) =
        sqlite
            .conn
            .query_row("SELECT name, sql FROM sqlite_master", [], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })?;
    assert_eq!(table_name, PAGE_PROPERTIES_TABLE);
    assert!(sql.contains(r#""あ&"";#' f　_" REAL"#));
    Ok(())
}

#[test]
#[serial]
fn it_inserts_notion_entry() -> Result<(), Box<dyn Error>> {
    setup(DATABASE_PATH);

    let schema = parse_database_schema(fixtures::NOTION_DATABASE_JSON)?;
    let sqlite = Sqlite::new(DATABASE_PATH, &schema)?;
    sqlite.create_tables()?;

    let page = NotionPage {
        id: "xxxx".to_string(),
        properties: HashMap::from([
            (
                "Name".to_string(),
                NotionPropertyValue::Text("Meu".to_string()),
            ),
            ("Age".to_string(), NotionPropertyValue::Number(5.0)),
        ]),
    };
    sqlite.insert(&page)?;

    let (page_id, name, age): (String, String, f64) = sqlite.conn.query_row(
        format!(
            r#"SELECT page_id,"Name","Age" from {table_name}"#,
            table_name = PAGE_PROPERTIES_TABLE
        )
        .as_str(),
        [],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )?;
    assert_eq!(page_id, "xxxx");
    assert_eq!(name, "Meu");
    assert_eq!(age, 5.0);
    Ok(())
}
