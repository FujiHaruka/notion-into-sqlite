use crate::notion_database_schema::NotionDatabaseSchema;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

pub enum NotionEntryValue {
    Text(String),
    Number(f64),
    Json(Value),
}

pub struct NotionEntry {
    id: String,
    properties: HashMap<String, NotionEntryValue>,
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

pub fn parse_notion_entries(
    schema: &NotionDatabaseSchema,
    query_resp_json: &str,
) -> Result<Vec<NotionEntry>, Box<dyn Error>> {
    let query_resp = serde_json::from_str::<Value>(query_resp_json)?;

    validate_object_type(&query_resp)?;

    let results = query_resp
        .as_object()
        .and_then(|resp| resp.get("results"))
        .and_then(|results| results.as_array())
        .map(|results| {
            results
                .iter()
                .filter_map(|r| r.as_object())
                .collect::<Vec<_>>()
        })
        .ok_or(InvalidListObjectError(
            r#"It must have "results" as array of objects."#.to_string(),
        ))?;
    let entries = results;

    Ok(vec![])
}

fn validate_object_type(query_resp: &Value) -> Result<(), InvalidListObjectError> {
    let object_field = query_resp
        .as_object()
        .and_then(|o| o.get("object"))
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

#[cfg(test)]
mod tests {
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
