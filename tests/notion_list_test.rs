mod common;

extern crate notion_into_sqlite;

use common::fixtures;
use notion_into_sqlite::notion_database::parse_database_schema;
use notion_into_sqlite::notion_list::{parse_notion_list, NotionPropertyValue};
use std::error::Error;

#[test]
fn it_parses_notion_list() -> Result<(), Box<dyn Error>> {
    let schema = parse_database_schema(fixtures::NOTION_DATABASE_JSON)?;
    let (list, next_cursor) = parse_notion_list(&schema, fixtures::NOTION_LIST_JSON)?;
    assert_eq!(next_cursor.unwrap(), "e6c9af10-44ec-4a48-a969-156ba5438ff0");
    assert_eq!(list.len(), 1);

    let entry = list.first().unwrap();
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
