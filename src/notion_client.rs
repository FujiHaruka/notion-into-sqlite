use serde_json::Value;
use std::error::Error;
use std::fmt;

use crate::notion_database_schema::{NotionDatabaseSchema, NotionProperty, NotionPropertyType, parse_database_schema};

#[derive(Debug, Clone)]
struct ResponseParseError;

impl fmt::Display for ResponseParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to parse response")
    }
}

impl Error for ResponseParseError {}

pub struct NotionClient {
    pub api_key: String,
}

impl NotionClient {
    pub fn get_database(&self, database_id: &str) -> Result<NotionDatabaseSchema, Box<dyn Error>> {
        let url = format!("https://api.notion.com/v1/databases/{0}", database_id);
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(url)
            .header("Authorization", "Bearer ".to_string() + &self.api_key)
            .header("Notion-Version", "2022-02-22")
            .send()?
            .json::<Value>()?;

        let schema = parse_database_schema(&resp).ok_or(ResponseParseError)?;
        Ok(schema)
    }

    pub fn get_all_columns(&self, database_id: &str) -> Result<Value, Box<dyn Error>> {
        let url = format!("https://api.notion.com/v1/databases/{0}/query", database_id);
        let client = reqwest::blocking::Client::new();

        let mut next_cursor: Option<&str> = None;
        loop {
            let next_cursor_json =
                next_cursor.map_or("null".to_string(), |cursor| format!(r#""{0}""#, cursor));
            let query = format!(
                r#"
            {{
                "page_size": 1,
                "start_cursor": {0}
            }}        
            "#,
                next_cursor_json
            );
            let resp = client
                .post(&url)
                .header("Authorization", "Bearer ".to_string() + &self.api_key)
                .header("Notion-Version", "2022-02-22")
                .body(query)
                .send()?
                .json::<Value>()?;

            if next_cursor.is_none() {
                break;
            }
        }

        Ok(Value::Null)
    }
}

// fn parse_query_response(query_resp: &Value) -> Option<Vec<NotionEntity>> {
//     None
// }

#[cfg(test)]
mod tests {
    #[test]
    fn parse_query_result() {
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
}
