use anyhow::{anyhow, Result};
use serde_json::Value;
use std::collections::HashMap;

/// Types of property values
/// See https://developers.notion.com/reference/property-value-object
/// > Possible values are "rich_text", "number", "select", "multi_select", "date",
/// > "formula", "relation", "rollup", "title", "people", "files", "checkbox","url",
/// > "email", "phone_number", "created_time", "created_by", "last_edited_time", and "last_edited_by".
#[derive(Debug, PartialEq)]
pub enum NotionPropertyType {
    RichText,
    Number,
    Select,
    MultiSelect,
    Date,
    Formula,
    Relation,
    Rollup,
    Title,
    People,
    Files,
    Checkbox,
    Url,
    Email,
    PhoneNumber,
    CreatedTime,
    CreatedBy,
    LastEditedTime,
    LastEditedBy,
    Other,
}

#[derive(Debug)]
pub struct NotionProperty {
    pub name: String,
    pub property_type: NotionPropertyType,
    pub property_raw_type: String,
}

#[derive(Debug)]
pub struct NotionDatabaseSchema {
    pub properties: HashMap<String, NotionProperty>,
}

pub fn parse_database_schema(database_resp: &Value) -> Result<NotionDatabaseSchema> {
    validate_object_type(database_resp)?;

    let raw_properties = database_resp
        .as_object()
        .and_then(|resp| resp.get("properties"))
        .and_then(|prop| prop.as_object())
        .ok_or_else(|| anyhow!(r#"It must have "properties" object."#))?;

    let properties = raw_properties
        .keys()
        .filter_map(|key| {
            let property = raw_properties.get(key)?.as_object()?;
            let name = property.get("name")?.as_str()?;
            let property_raw_type = property.get("type")?.as_str()?;
            let property_type = match property_raw_type {
                "rich_text" => NotionPropertyType::RichText,
                "number" => NotionPropertyType::Number,
                "select" => NotionPropertyType::Select,
                "multi_select" => NotionPropertyType::MultiSelect,
                "date" => NotionPropertyType::Date,
                "formula" => NotionPropertyType::Formula,
                "relation" => NotionPropertyType::Relation,
                "rollup" => NotionPropertyType::Rollup,
                "title" => NotionPropertyType::Title,
                "people" => NotionPropertyType::People,
                "files" => NotionPropertyType::Files,
                "checkbox" => NotionPropertyType::Checkbox,
                "url" => NotionPropertyType::Url,
                "email" => NotionPropertyType::Email,
                "phone_number" => NotionPropertyType::PhoneNumber,
                "created_time" => NotionPropertyType::CreatedTime,
                "created_by" => NotionPropertyType::CreatedBy,
                "last_edited_time" => NotionPropertyType::LastEditedTime,
                "last_edited_by" => NotionPropertyType::LastEditedBy,
                _ => NotionPropertyType::Other,
            };
            Some((
                name.to_string(),
                NotionProperty {
                    name: name.to_string(),
                    property_raw_type: property_raw_type.to_string(),
                    property_type,
                },
            ))
        })
        .collect::<HashMap<String, NotionProperty>>();

    Ok(NotionDatabaseSchema { properties })
}

fn validate_object_type(database_resp: &Value) -> Result<()> {
    let object_field = database_resp
        .as_object()
        .and_then(|o| o.get("object"))
        .and_then(|o| o.as_str())
        .ok_or_else(|| anyhow!(r#"It must have `"object": "database"`."#.to_string()))?;

    if object_field == "database" {
        Ok(())
    } else {
        Err(anyhow!(
            r#"It must have `"object": "database"`, but was "{}""#,
            object_field
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_object_type() {
        let data = r#"
        {
            "object": "database"
        }
        "#;
        let json = serde_json::from_str(data).unwrap();
        assert!(validate_object_type(&json).is_ok());

        let data = r#"
        {
            "object": "xxx"
        }
        "#;
        let json = serde_json::from_str(data).unwrap();
        assert!(validate_object_type(&json).is_err());

        let data = r#"
        {}
        "#;
        let json = serde_json::from_str(data).unwrap();
        assert!(validate_object_type(&json).is_err());
    }
}
