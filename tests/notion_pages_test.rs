mod common;

extern crate notion_into_sqlite;

use common::fixtures;
use notion_into_sqlite::notion_database::parse_database_schema;
use notion_into_sqlite::notion_pages::{parse_notion_page_list, NotionPropertyValue};
use std::error::Error;

#[test]
fn it_parses_notion_page_list() -> Result<(), Box<dyn Error>> {
    let json = serde_json::from_str::<serde_json::Value>(fixtures::NOTION_DATABASE_JSON)?;
    let schema = parse_database_schema(&json)?;
    let pages_json = serde_json::from_str::<serde_json::Value>(fixtures::NOTION_LIST_JSON)?;
    let (pages, next_cursor) = parse_notion_page_list(&schema, &pages_json)?;
    assert_eq!(next_cursor.unwrap(), "e6c9af10-44ec-4a48-a969-156ba5438ff0");
    assert_eq!(pages.len(), 1);

    let entry = pages.first().unwrap();
    assert_eq!(entry.id, "a75b9220-455d-48e1-a36b-c581a345f777");
    assert_eq!(entry.properties.len(), 3);

    let properties = &entry.properties;
    assert_eq!(
        properties.get("Name").unwrap(),
        &NotionPropertyValue::Text("Meu".to_string())
    );
    assert_eq!(
        properties.get("Age").unwrap(),
        &NotionPropertyValue::Number(10.0)
    );
    assert_eq!(
        properties.get("Animal").unwrap(),
        &NotionPropertyValue::Text("cat".to_string())
    );
    Ok(())
}

#[test]
fn it_parses_notion_page_list_with_all_types() -> Result<(), Box<dyn Error>> {
    let json = serde_json::from_str::<serde_json::Value>(fixtures::NOTION_DATABASE_ALL_TYPES_JSON)?;
    let schema = parse_database_schema(&json)?;
    let pages_json =
        serde_json::from_str::<serde_json::Value>(fixtures::NOTION_LIST_ALL_TYPES_JSON)?;
    let (pages, _) = parse_notion_page_list(&schema, &pages_json)?;
    assert_eq!(pages.len(), 1);

    let entry = pages.first().unwrap();
    assert_eq!(entry.id, "ce4593d9-0cfb-4659-8012-12594b723312");
    assert_eq!(entry.properties.len(), 19);

    Ok(())
}
