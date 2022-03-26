mod common;

extern crate notion_into_sqlite;

use rusqlite::params;
use std::collections::HashMap;

use common::{fixtures, helpers};
use notion_into_sqlite::notion_database::parse_database_schema;
use notion_into_sqlite::notion_pages::{NotionPage, NotionPropertyValue};
use notion_into_sqlite::sqlite::{
    Sqlite, PAGE_ID_COLUMN, PAGE_METADATA_TABLE, PAGE_PROPERTIES_TABLE,
};
use std::error::Error;

#[test]
fn it_creates_tables() -> Result<(), Box<dyn Error>> {
    let database_path = "tmp/test1.db";
    helpers::before_db(database_path);

    let json = serde_json::from_str::<serde_json::Value>(fixtures::NOTION_DATABASE_JSON)?;
    let schema = parse_database_schema(&json)?;
    let sqlite = Sqlite::new(database_path, &schema)?;
    sqlite.create_tables()?;

    let table_def_sql: String = sqlite.conn.query_row(
        "SELECT sql FROM sqlite_master where name=?1",
        params![PAGE_PROPERTIES_TABLE],
        |row| Ok(row.get(0)?),
    )?;
    assert!(table_def_sql.contains(&format!(
        "{id_column} TEXT PRIMARY KEY",
        id_column = PAGE_ID_COLUMN
    )));
    assert!(table_def_sql.contains(r#""Name" TEXT"#));
    assert!(table_def_sql.contains(r#""Animal" TEXT"#));
    assert!(table_def_sql.contains(r#""Age" REAL"#));

    let table_def_sql: String = sqlite.conn.query_row(
        "SELECT sql FROM sqlite_master where name=?1",
        params![PAGE_METADATA_TABLE],
        |row| Ok(row.get(0)?),
    )?;
    assert!(table_def_sql.contains(r#"url TEXT"#));

    Ok(())
}

#[test]
fn it_creates_table_when_column_name_includes_double_quote() -> Result<(), Box<dyn Error>> {
    let database_path = "tmp/test2.db";
    helpers::before_db(database_path);

    let json = serde_json::from_str::<serde_json::Value>(fixtures::NOTION_DATABASE_IRREGULAR_JSON)?;
    let schema = parse_database_schema(&json)?;
    let sqlite = Sqlite::new(database_path, &schema)?;
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
fn it_inserts_notion_entry() -> Result<(), Box<dyn Error>> {
    let database_path = "tmp/test3.db";
    helpers::before_db(database_path);

    let json = serde_json::from_str::<serde_json::Value>(fixtures::NOTION_DATABASE_JSON)?;
    let schema = parse_database_schema(&json)?;
    let sqlite = Sqlite::new(database_path, &schema)?;
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
        url: "https://www.notion.so/xxxx".to_string(),
        created_time: "2022-03-12T00:15:00.000Z".to_string(),
        created_by: serde_json::from_str(
            r#"{
            "object": "user",
            "id": "9d069f8b-6223-4853-b7eb-8fe3dfe7d389"
        }"#,
        )
        .unwrap(),
        last_edited_time: "2022-03-12T00:16:00.000Z".to_string(),
        last_edited_by: serde_json::from_str(
            r#"{
            "object": "user",
            "id": "9d069f8b-6223-4853-b7eb-8fe3dfe7d389"
        }"#,
        )
        .unwrap(),
        archived: false,
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

    let (page_id, url, created_time, created_by): (String, String, String, String) =
        sqlite.conn.query_row(
            format!(
                r#"SELECT id, url, created_time, created_by from {table_name}"#,
                table_name = PAGE_METADATA_TABLE
            )
            .as_str(),
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )?;
    assert_eq!(page_id, "xxxx");
    assert_eq!(url, "https://www.notion.so/xxxx");
    assert_eq!(created_time, "2022-03-12T00:15:00.000Z");
    assert!(serde_json::from_str::<serde_json::Value>(&created_by).is_ok());

    Ok(())
}
