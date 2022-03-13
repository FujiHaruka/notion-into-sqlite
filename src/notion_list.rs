use crate::json_util::{dig_json, JsonKey};
use crate::notion_database_schema::{NotionDatabaseSchema, NotionPropertyType};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum NotionPropertyValue {
    Text(String),
    Number(f64),
    Json(Value),
}

#[derive(Debug)]
pub struct NotionEntry {
    id: String,
    properties: HashMap<String, NotionPropertyValue>,
}

#[derive(Debug)]
struct NotionEntryBuilder<'a> {
    schema: &'a NotionDatabaseSchema,
    title_json_path: Vec<JsonKey<'a>>,
    number_json_path: Vec<JsonKey<'a>>,
    select_json_path: Vec<JsonKey<'a>>,
}
impl NotionEntryBuilder<'_> {
    fn new<'a>(schema: &'a NotionDatabaseSchema) -> NotionEntryBuilder<'a> {
        NotionEntryBuilder {
            schema,
            title_json_path: vec!["title".into(), 0.into(), "plain_text".into()],
            number_json_path: vec!["number".into()],
            select_json_path: vec!["select".into(), "name".into()],
        }
    }

    fn from(&self, json_entry: &Map<String, Value>) -> Option<NotionEntry> {
        let id = json_entry.get("id")?.as_str()?.to_string();
        let properties_object = json_entry.get("properties")?.as_object()?;
        let properties = properties_object
            .iter()
            .filter_map(|(key, property)| {
                let property_schema = self.schema.properties.get(key)?;
                let value: NotionPropertyValue = match &property_schema.property_type {
                    &NotionPropertyType::Title => NotionPropertyValue::Text(
                        dig_json(property, &self.title_json_path)?
                            .as_str()?
                            .to_string(),
                    ),
                    &NotionPropertyType::Number => NotionPropertyValue::Number(
                        dig_json(property, &self.number_json_path)?.as_f64()?,
                    ),
                    &NotionPropertyType::Select => NotionPropertyValue::Text(
                        dig_json(property, &self.select_json_path)?
                            .as_str()?
                            .to_string(),
                    ),
                    &NotionPropertyType::Other => NotionPropertyValue::Json(
                        property.get(&property_schema.property_raw_type)?.clone(),
                    ),
                };
                Some((key.to_string(), value))
            })
            .collect::<HashMap<String, NotionPropertyValue>>();

        Some(NotionEntry { id, properties })
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

pub fn parse_notion_list(
    schema: &NotionDatabaseSchema,
    query_resp_json: &str,
) -> Result<(Vec<NotionEntry>, Option<String>), Box<dyn Error>> {
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
        .ok_or(InvalidListObjectError(
            r#"It must have "results" as arrray of objects."#.to_string(),
        ))?;

    let entry_builder = NotionEntryBuilder::new(schema);
    let entries: Vec<NotionEntry> = results
        .iter()
        .filter_map(|&result| entry_builder.from(result))
        .collect::<Vec<_>>();

    Ok((entries, next_cursor))
}

fn validate_object_type(query_resp: &Value) -> Result<(), InvalidListObjectError> {
    let json_keys = vec![JsonKey::String("object")];
    let object_field = dig_json(query_resp, &json_keys)
        .and_then(|o| o.as_str())
        .ok_or(InvalidListObjectError(
            r#"It must have `"object": "list"`."#.to_string(),
        ))?;

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
    use crate::notion_database_schema::NotionProperty;
    use super::*;

    #[test]
    fn test_parse_query_result() {
        let data = r#"
        {
            "object": "list",
            "results": [
              {
                "object": "page",
                "id": "a75b9220-455d-48e1-a36b-c581a345f777",
                "created_time": "2022-03-12T00:15:00.000Z",
                "last_edited_time": "2022-03-12T00:16:00.000Z",
                "created_by": {
                  "object": "user",
                  "id": "9d069f8b-6223-4853-b7eb-8fe3dfe7d389"
                },
                "last_edited_by": {
                  "object": "user",
                  "id": "9d069f8b-6223-4853-b7eb-8fe3dfe7d389"
                },
                "cover": null,
                "icon": null,
                "parent": {
                  "type": "database_id",
                  "database_id": "f2bf4cd7-b8d1-44fc-856e-8fe60c128b58"
                },
                "archived": false,
                "properties": {
                  "Age": {
                    "id": "GPCK",
                    "type": "number",
                    "number": 10
                  },
                  "Animal": {
                    "id": "wzVU",
                    "type": "select",
                    "select": {
                      "id": "67fe1cf3-29f8-4cb7-9517-803e1d975e86",
                      "name": "cat",
                      "color": "green"
                    }
                  },
                  "Name": {
                    "id": "title",
                    "type": "title",
                    "title": [
                      {
                        "type": "text",
                        "text": {
                          "content": "Meu",
                          "link": null
                        },
                        "annotations": {
                          "bold": false,
                          "italic": false,
                          "strikethrough": false,
                          "underline": false,
                          "code": false,
                          "color": "default"
                        },
                        "plain_text": "Meu",
                        "href": null
                      }
                    ]
                  }
                },
                "url": "https://www.notion.so/Meu-a75b9220455d48e1a36bc581a345f777"
              }
            ],
            "next_cursor": "e6c9af10-44ec-4a48-a969-156ba5438ff0",
            "has_more": true,
            "type": "page",
            "page": {}
        }
        "#;
        let schema = NotionDatabaseSchema{
            properties: HashMap::from([
                ("Name".to_string(), NotionProperty{
                    name: "Name".to_string(),
                    property_raw_type: "title".to_string(),
                    property_type: NotionPropertyType::Title,
                }),
                ("Age".to_string(), NotionProperty{
                    name: "Age".to_string(),
                    property_raw_type: "number".to_string(),
                    property_type: NotionPropertyType::Number,
                }),
                ("Animal".to_string(), NotionProperty{
                    name: "Animal".to_string(),
                    property_raw_type: "select".to_string(),
                    property_type: NotionPropertyType::Select,
                })
            ])
        };
        let (list, next_cursor) = parse_notion_list(&schema, &data).unwrap();
        assert_eq!(next_cursor.unwrap(), "e6c9af10-44ec-4a48-a969-156ba5438ff0");
        assert_eq!(list.len(), 1);
        let entry = list.first().unwrap();
        assert_eq!(entry.id, "a75b9220-455d-48e1-a36b-c581a345f777");
        assert_eq!(entry.properties.len(), 3);
    }

    #[test]
    fn test_validate_object_type() {
        let data = r#"
        {
            "object": "list"
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
