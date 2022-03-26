use anyhow::{anyhow, Result};
use serde_json::{json, Value};

use crate::json_util::{dig_json, JsonKey};
use crate::notion_database::{parse_database_schema, NotionDatabaseSchema};
use crate::notion_pages::{parse_notion_page_list, NotionPage};

pub struct NotionClient {
    pub api_key: String,
}

impl NotionClient {
    pub fn get_database(&self, database_id: &str) -> Result<NotionDatabaseSchema> {
        let url = format!("https://api.notion.com/v1/databases/{0}", database_id);
        let client = reqwest::blocking::Client::new();
        info!("Requesting database schema. URL: {}", &url);
        let resp = client
            .get(url)
            .header("Authorization", "Bearer ".to_string() + &self.api_key)
            .header("Notion-Version", "2022-02-22")
            .send()?
            .json::<Value>()?;
        info!("Request done.");

        self.validate_response(&resp)?;

        let schema = parse_database_schema(&resp)?;
        info!("Database schema: {:?}", schema);
        Ok(schema)
    }

    pub fn get_all_pages(
        &self,
        database_id: &str,
        schema: &NotionDatabaseSchema,
    ) -> Result<Vec<NotionPage>> {
        let url = format!("https://api.notion.com/v1/databases/{0}/query", database_id);
        let client = reqwest::blocking::Client::new();

        let mut next_cursor: Option<String> = None;
        let mut all_pages: Vec<NotionPage> = vec![];
        loop {
            let mut query = json!({
                "page_size": 10i32,
                "sorts": [{
                    "timestamp": "created_time",
                    "direction": "ascending",
                }]
            });
            if let Some(cursor) = (&next_cursor).as_ref() {
                query
                    .as_object_mut()
                    .unwrap()
                    .insert("start_cursor".into(), cursor.clone().into());
            }
            let query_str = query.to_string();

            info!("Requesting query: URL: {}, query: {}", &url, &query_str);
            let resp = client
                .post(&url)
                .header("Authorization", "Bearer ".to_string() + &self.api_key)
                .header("Notion-Version", "2022-02-22")
                .header("Content-Type", "application/json")
                .body(query_str)
                .send()?
                .json::<Value>()?;
            info!("Request done.");

            self.validate_response(&resp)?;

            let (mut pages, _next_cursor) = parse_notion_page_list(schema, &resp)?;
            info!("Pages: {:?}", pages.len());
            all_pages.append(&mut pages);
            next_cursor = _next_cursor;

            if next_cursor.is_none() {
                info!("Fetched all items.");
                break;
            } else {
                info!("Has more items.");
            }
        }

        Ok(all_pages)
    }

    fn validate_response(&self, resp: &Value) -> Result<()> {
        let json_keys = vec![JsonKey::String("object")];
        let object_field = dig_json(resp, &json_keys)
            .and_then(|o| o.as_str())
            .ok_or_else(|| anyhow!("Unexpected response from Notion API: {}", resp))?;

        if object_field == "error" {
            Err(anyhow!("Error response from Notion API: {}", resp,))
        } else {
            Ok(())
        }
    }
}
