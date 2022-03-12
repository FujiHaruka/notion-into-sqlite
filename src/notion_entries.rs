use crate::notion_database_schema::{NotionDatabaseSchema};
use serde_json::Value;
use std::collection::HashMap;

pub enum NotionEntryValue {
    Text(String),
    Number(f64),
    Json(Value),
}

type NotionEntry = HashMap<String, NotionEntryValue>;

pub fn parse_notion_entries(schema: &NotionDatabaseSchema, query_resp: &Value) -> Option<Vec<NotionEntry>> {
    None
}

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