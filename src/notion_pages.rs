use crate::json_util::{dig_json, JsonKey};
use crate::notion_database::{NotionDatabaseSchema, NotionPropertyType};
use anyhow::{anyhow, Result};
use rusqlite::ToSql;
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum NotionPropertyValue {
    Text(String),
    Number(f64),
    Json(Value),
    Boolean(bool),
}
impl ToSql for NotionPropertyValue {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        match self {
            NotionPropertyValue::Text(value) => value.to_sql(),
            NotionPropertyValue::Number(value) => value.to_sql(),
            NotionPropertyValue::Json(value) => Ok(rusqlite::types::ToSqlOutput::from(
                serde_json::to_string(value).unwrap(),
            )),
            NotionPropertyValue::Boolean(value) => value.to_sql(),
        }
    }
}

#[derive(Debug)]
pub struct NotionPage {
    pub id: String,
    pub properties: HashMap<String, NotionPropertyValue>,
    pub url: String,
    pub created_time: String,
    pub created_by: Value,
    pub last_edited_time: String,
    pub last_edited_by: Value,
    pub archived: bool,
}

#[allow(non_snake_case)]
#[derive(Debug)]
struct NotionPageBuilder<'a> {
    schema: &'a NotionDatabaseSchema,
    TITLE_JSON_PATH: Vec<JsonKey<'a>>,
    SELECT_JSON_PATH: Vec<JsonKey<'a>>,
}
impl NotionPageBuilder<'_> {
    fn new(schema: &NotionDatabaseSchema) -> NotionPageBuilder<'_> {
        NotionPageBuilder {
            schema,
            TITLE_JSON_PATH: vec!["title".into(), 0.into(), "plain_text".into()],
            SELECT_JSON_PATH: vec!["select".into(), "name".into()],
        }
    }

    fn from(&self, json_entry: &Map<String, Value>) -> Option<NotionPage> {
        let id = json_entry.get("id")?.as_str()?.to_string();

        let url = json_entry.get("url")?.as_str()?.to_string();
        let created_time = json_entry.get("created_time")?.as_str()?.to_owned();
        let created_by = json_entry.get("created_by")?.clone();
        let last_edited_time = json_entry.get("last_edited_time")?.as_str()?.to_owned();
        let last_edited_by = json_entry.get("last_edited_by")?.clone();
        let archived = json_entry.get("archived")?.as_bool()?;

        let properties_object = json_entry.get("properties")?.as_object()?;
        let properties = properties_object
            .iter()
            .filter_map(|(key, property)| {
                let property_schema = self.schema.properties.get(key)?;
                let value: NotionPropertyValue = match property_schema.property_type {
                    // TODO: convert to plain text
                    NotionPropertyType::RichText => {
                        NotionPropertyValue::Json(property.get("rich_text")?.clone())
                    }
                    NotionPropertyType::Number => {
                        NotionPropertyValue::Number(property.get("number")?.as_f64()?)
                    }
                    NotionPropertyType::Select => NotionPropertyValue::Text(
                        dig_json(property, &self.SELECT_JSON_PATH)?
                            .as_str()?
                            .to_string(),
                    ),
                    NotionPropertyType::Title => NotionPropertyValue::Text(
                        dig_json(property, &self.TITLE_JSON_PATH)?
                            .as_str()?
                            .to_string(),
                    ),
                    NotionPropertyType::Checkbox => {
                        NotionPropertyValue::Boolean(property.get("checkbox")?.as_bool()?)
                    }
                    NotionPropertyType::Url => {
                        NotionPropertyValue::Text(property.get("url")?.as_str()?.to_string())
                    }
                    NotionPropertyType::Email => {
                        NotionPropertyValue::Text(property.get("email")?.as_str()?.to_string())
                    }
                    NotionPropertyType::PhoneNumber => NotionPropertyValue::Text(
                        property.get("phone_number")?.as_str()?.to_string(),
                    ),
                    NotionPropertyType::CreatedTime => NotionPropertyValue::Text(
                        property.get("created_time")?.as_str()?.to_string(),
                    ),
                    NotionPropertyType::LastEditedTime => NotionPropertyValue::Text(
                        property.get("last_edited_time")?.as_str()?.to_string(),
                    ),
                    NotionPropertyType::Other => NotionPropertyValue::Json(property.clone()),
                    _ => NotionPropertyValue::Json(
                        property.get(&property_schema.property_raw_type)?.clone(),
                    ),
                };
                Some((key.to_string(), value))
            })
            .collect::<HashMap<String, NotionPropertyValue>>();

        Some(NotionPage {
            id,
            properties,
            url,
            created_time,
            created_by,
            last_edited_by,
            last_edited_time,
            archived,
        })
    }
}

pub fn parse_notion_page_list(
    schema: &NotionDatabaseSchema,
    query_resp: &Value,
) -> Result<(Vec<NotionPage>, Option<String>)> {
    validate_object_type(query_resp)?;

    let next_cursor = get_next_cursor(query_resp);

    let results_json_keys = vec![JsonKey::String("results")];
    let results = dig_json(query_resp, &results_json_keys)
        .and_then(|results| results.as_array())
        .map(|results| {
            results
                .iter()
                .filter_map(|r| r.as_object())
                .collect::<Vec<_>>()
        })
        .ok_or_else(|| anyhow!(r#"It must have "results" as arrray of objects."#))?;

    let page_builder = NotionPageBuilder::new(schema);
    let pages: Vec<NotionPage> = results
        .iter()
        .filter_map(|&result| page_builder.from(result))
        .collect::<Vec<_>>();

    Ok((pages, next_cursor))
}

fn validate_object_type(query_resp: &Value) -> Result<()> {
    let json_keys = vec![JsonKey::String("object")];
    let object_field = dig_json(query_resp, &json_keys)
        .and_then(|o| o.as_str())
        .ok_or_else(|| anyhow!(r#"It must have `"object": "list"`."#.to_string()))?;

    if object_field == "list" {
        Ok(())
    } else {
        Err(anyhow!(
            r#"It must have `"object": "list"`, but was "{}""#,
            object_field
        ))
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
