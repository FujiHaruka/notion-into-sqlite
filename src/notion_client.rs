use serde_json::Value;
use std::error::Error;

use crate::json_util::{dig_json, JsonKey};
use crate::notion_database::{parse_database_schema, NotionDatabaseSchema};
use crate::notion_pages::{parse_notion_page_list, NotionPage};

pub struct NotionClient {
    pub api_key: String,
}

impl NotionClient {
    pub fn get_database(&self, database_id: &str) -> Result<NotionDatabaseSchema, Box<dyn Error>> {
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
    ) -> Result<Vec<NotionPage>, Box<dyn Error>> {
        let url = format!("https://api.notion.com/v1/databases/{0}/query", database_id);
        let client = reqwest::blocking::Client::new();

        let mut next_cursor: Option<String> = None;
        let mut all_pages: Vec<NotionPage> = vec![];
        loop {
            let next_cursor_json =
                next_cursor.map_or("null".to_string(), |cursor| format!(r#""{0}""#, cursor));
            let query = format!(
                r#"{{ "page_size": 50, "start_cursor": {0} }}"#,
                next_cursor_json
            );

            info!("Requesting query: URL: {}, query: {}", &url, &query);
            let resp = client
                .post(&url)
                .header("Authorization", "Bearer ".to_string() + &self.api_key)
                .header("Notion-Version", "2022-02-22")
                .body(query)
                .send()?
                .json::<Value>()?;
            info!("Request done.");

            self.validate_response(&resp)?;

            let (mut pages, _next_cursor) = parse_notion_page_list(schema, &resp)?;
            info!("Pages: {:?}", pages);
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

    fn validate_response(&self, resp: &Value) -> Result<(), String> {
        let json_keys = vec![JsonKey::String("object")];
        let object_field = dig_json(resp, &json_keys)
            .and_then(|o| o.as_str())
            .ok_or_else(|| format!("Unexpected response from Notion API: {}", resp.to_string()))?;

        if object_field == "error" {
            Err(format!(
                "Error response from Notion API: {}",
                resp.to_string(),
            ))
        } else {
            Ok(())
        }
    }
}
