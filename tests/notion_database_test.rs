mod common;

extern crate notion_into_sqlite;

use common::fixtures;
use notion_into_sqlite::notion_database::{parse_database_schema, NotionPropertyType};
use std::error::Error;

#[test]
fn it_parses_database_json() -> Result<(), Box<dyn Error>> {
    let json = serde_json::from_str::<serde_json::Value>(fixtures::NOTION_DATABASE_JSON)?;
    let schema = parse_database_schema(&json)?;
    let properties = schema.properties;
    assert_eq!(properties.len(), 3);

    let name_property = properties.get("Name").unwrap();
    assert_eq!(name_property.name, "Name");
    assert_eq!(name_property.property_raw_type, "title");
    assert_eq!(name_property.property_type, NotionPropertyType::Title);

    let age_property = properties.get("Age").unwrap();
    assert_eq!(age_property.name, "Age");
    assert_eq!(age_property.property_raw_type, "number");
    assert_eq!(age_property.property_type, NotionPropertyType::Number);

    let age_property = properties.get("Animal").unwrap();
    assert_eq!(age_property.name, "Animal");
    assert_eq!(age_property.property_raw_type, "select");
    assert_eq!(age_property.property_type, NotionPropertyType::Select);
    Ok(())
}

#[test]
fn it_parses_database_json_with_all_property_types() -> Result<(), Box<dyn Error>> {
    let json = serde_json::from_str::<serde_json::Value>(fixtures::NOTION_DATABASE_ALL_TYPES_JSON)?;
    let schema = parse_database_schema(&json)?;
    let properties = schema.properties;
    assert_eq!(properties.len(), 19);

    Ok(())
}
