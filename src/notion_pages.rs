use crate::json_util::{dig_json, JsonKey};
use crate::notion_database::{NotionDatabaseSchema, NotionPropertyType};
use rusqlite::ToSql;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum NotionPropertyValue {
    Text(String),
    Number(f64),
    Json(Value),
}
impl ToSql for NotionPropertyValue {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        match self {
            NotionPropertyValue::Text(value) => value.to_sql(),
            NotionPropertyValue::Number(value) => value.to_sql(),
            NotionPropertyValue::Json(value) => Ok(rusqlite::types::ToSqlOutput::from(
                serde_json::to_string(value).unwrap(),
            )),
        }
    }
}

#[derive(Debug)]
pub struct NotionPage {
    pub id: String,
    pub properties: HashMap<String, NotionPropertyValue>,
}

#[derive(Debug)]
struct NotionPageBuilder<'a> {
    schema: &'a NotionDatabaseSchema,
    title_json_path: Vec<JsonKey<'a>>,
    number_json_path: Vec<JsonKey<'a>>,
    select_json_path: Vec<JsonKey<'a>>,
}
impl NotionPageBuilder<'_> {
    fn new(schema: &NotionDatabaseSchema) -> NotionPageBuilder<'_> {
        NotionPageBuilder {
            schema,
            title_json_path: vec!["title".into(), 0.into(), "plain_text".into()],
            number_json_path: vec!["number".into()],
            select_json_path: vec!["select".into(), "name".into()],
        }
    }

    fn from(&self, json_entry: &Map<String, Value>) -> Option<NotionPage> {
        let id = json_entry.get("id")?.as_str()?.to_string();
        let properties_object = json_entry.get("properties")?.as_object()?;
        let properties = properties_object
            .iter()
            .filter_map(|(key, property)| {
                let property_schema = self.schema.properties.get(key)?;
                let value: NotionPropertyValue = match property_schema.property_type {
                    NotionPropertyType::Title => NotionPropertyValue::Text(
                        dig_json(property, &self.title_json_path)?
                            .as_str()?
                            .to_string(),
                    ),
                    NotionPropertyType::Number => NotionPropertyValue::Number(
                        dig_json(property, &self.number_json_path)?.as_f64()?,
                    ),
                    NotionPropertyType::Select => NotionPropertyValue::Text(
                        dig_json(property, &self.select_json_path)?
                            .as_str()?
                            .to_string(),
                    ),
                    NotionPropertyType::Other => NotionPropertyValue::Json(
                        property.get(&property_schema.property_raw_type)?.clone(),
                    ),
                };
                Some((key.to_string(), value))
            })
            .collect::<HashMap<String, NotionPropertyValue>>();

        Some(NotionPage { id, properties })
    }
}

#[derive(Debug, Clone)]
struct InvalidListObjectError(String);
impl fmt::Display for InvalidListObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = self.0.as_str();
        write!(f, "Invalid list object. {}", message)
    }
}
impl Error for InvalidListObjectError {}

pub fn parse_notion_page_list(
    schema: &NotionDatabaseSchema,
    query_resp_json: &str,
) -> Result<(Vec<NotionPage>, Option<String>), Box<dyn Error>> {
    let query_resp = serde_json::from_str::<Value>(query_resp_json)?;

    validate_object_type(&query_resp)?;

    let next_cursor = get_next_cursor(&query_resp);

    let results_json_keys = vec![JsonKey::String("results")];
    let results = dig_json(&query_resp, &results_json_keys)
        .and_then(|results| results.as_array())
        .map(|results| {
            results
                .iter()
                .filter_map(|r| r.as_object())
                .collect::<Vec<_>>()
        })
        .ok_or_else(|| {
            InvalidListObjectError(r#"It must have "results" as arrray of objects."#.to_string())
        })?;

    let page_builder = NotionPageBuilder::new(schema);
    let pages: Vec<NotionPage> = results
        .iter()
        .filter_map(|&result| page_builder.from(result))
        .collect::<Vec<_>>();

    Ok((pages, next_cursor))
}

fn validate_object_type(query_resp: &Value) -> Result<(), InvalidListObjectError> {
    let json_keys = vec![JsonKey::String("object")];
    let object_field = dig_json(query_resp, &json_keys)
        .and_then(|o| o.as_str())
        .ok_or_else(|| InvalidListObjectError(r#"It must have `"object": "list"`."#.to_string()))?;

    if object_field == "list" {
        Ok(())
    } else {
        Err(InvalidListObjectError(format!(
            r#"It must have `"object": "list"`, but was "{}""#,
            object_field
        )))
    }
}

fn get_next_cursor(query_resp: &Value) -> Option<String> {
    let json_keys: Vec<JsonKey> = vec!["next_cursor".into()];
    Some(dig_json(query_resp, &json_keys)?.as_str()?.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_object_type() {
        let data = r#"
        {
            "object": "list"
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
