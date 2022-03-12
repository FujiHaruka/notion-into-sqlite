use serde_json::Value;
use std::error::Error;
use std::fmt;

use crate::notion_database_schema::{parse_database_schema, NotionDatabaseSchema};

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
