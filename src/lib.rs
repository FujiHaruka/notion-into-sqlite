pub mod json_util;
pub mod notion_client;
pub mod notion_database;
pub mod notion_pages;
pub mod sqlite;

#[macro_use]
extern crate log;

use crate::notion_client::NotionClient;
use crate::sqlite::Sqlite;
use anyhow::{Context, Result};

pub fn main(api_key: &str, database_id: &str, output: &str) -> Result<()> {
    env_logger::init();

    Sqlite::validate_database_path(output)
        .with_context(|| format!("Failed to create a database file {}", output))?;

    let client = NotionClient {
        api_key: api_key.into(),
    };

    let schema = client
        .get_database(database_id)
        .with_context(|| "Failed to fetch database schema")?;
    let pages = client
        .get_all_pages(database_id, &schema)
        .with_context(|| "Failed to fetch pages")?;

    let sqlite = Sqlite::new(output, &schema).with_context(|| "Failed to connect to sqlite")?;
    sqlite
        .create_tables()
        .with_context(|| "Failed to create tables")?;

    for page in pages {
        sqlite
            .insert(&page)
            .with_context(|| "Failed to insert pages to sqlite")?;
    }

    Ok(())
}
