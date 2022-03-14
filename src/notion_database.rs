use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum NotionPropertyType {
    Title,
    Number,
    Select,
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

#[derive(Debug, Clone)]
struct InvalidDatabaseObjectError(String);
impl fmt::Display for InvalidDatabaseObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = self.0.as_str();
        write!(f, "Invalid database object. {}", message)
    }
}
impl Error for InvalidDatabaseObjectError {}

pub fn parse_database_schema(
    database_resp_json: &str,
) -> Result<NotionDatabaseSchema, Box<dyn Error>> {
    let database_resp = serde_json::from_str::<Value>(database_resp_json)?;

    validate_object_type(&database_resp)?;

    let raw_properties = database_resp
        .as_object()
        .and_then(|resp| resp.get("properties"))
        .and_then(|prop| prop.as_object())
        .ok_or(InvalidDatabaseObjectError(
            r#"It must have "properties" object."#.to_string(),
        ))?;

    let properties = raw_properties
        .keys()
        .filter_map(|key| {
            let property = raw_properties.get(key)?.as_object()?;
            let name = property.get("name")?.as_str()?;
            let property_raw_type = property.get("type")?.as_str()?;
            let property_type = match property_raw_type {
                "title" => NotionPropertyType::Title,
                "select" => NotionPropertyType::Select,
                "number" => NotionPropertyType::Number,
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

fn validate_object_type(database_resp: &Value) -> Result<(), InvalidDatabaseObjectError> {
    let object_field = database_resp
        .as_object()
        .and_then(|o| o.get("object"))
        .and_then(|o| o.as_str())
        .ok_or(InvalidDatabaseObjectError(
            r#"It must have `"object": "database"`."#.to_string(),
        ))?;

    if object_field == "database" {
        Ok(())
    } else {
        Err(InvalidDatabaseObjectError(format!(
            r#"It must have `"object": "database"`, but was "{}""#,
            object_field
        )))
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
        assert_eq!(validate_object_type(&json).is_ok(), true);

        let data = r#"
        {
            "object": "xxx"
        }
        "#;
        let json = serde_json::from_str(data).unwrap();
        assert_eq!(validate_object_type(&json).is_err(), true);

        let data = r#"
        {}
        "#;
        let json = serde_json::from_str(data).unwrap();
        assert_eq!(validate_object_type(&json).is_err(), true);
    }
}
