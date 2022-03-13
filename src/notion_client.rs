use std::error::Error;

use crate::notion_database_schema::{parse_database_schema, NotionDatabaseSchema};
use crate::notion_list::{parse_notion_list, NotionEntry};

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
            .text()?;
        info!("Request done.");

        let schema = parse_database_schema(&resp)?;
        info!("Database schema: {:?}", schema);
        Ok(schema)
    }

    pub fn get_all_entries(
        &self,
        database_id: &str,
        schema: &NotionDatabaseSchema,
    ) -> Result<Vec<NotionEntry>, Box<dyn Error>> {
        let url = format!("https://api.notion.com/v1/databases/{0}/query", database_id);
        let client = reqwest::blocking::Client::new();

        let mut next_cursor: Option<String> = None;
        let mut notion_entries: Vec<NotionEntry> = vec![];
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
                .text()?;
            info!("Request done.");

            let (mut list, _next_cursor) = parse_notion_list(schema, &resp)?;
            info!("Items: {:?}", list);
            notion_entries.append(&mut list);
            next_cursor = _next_cursor;

            if next_cursor.is_none() {
                info!("Fetched all items.");
                break;
            } else {
                info!("Has more items.");
            }
        }

        Ok(notion_entries)
    }
}
