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
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Notion API key
    #[clap(long)]
    api_key: String,

    /// Notion database ID
    #[clap(long)]
    database_id: String,

    /// Output path of sqlite database
    #[clap(long, default_value = "notion.db")]
    output: String,
}

pub fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();
    let api_key = args.api_key;
    let database_id = args.database_id;
    let output = args.output;

    Sqlite::validate_database_path(&output)
        .with_context(|| format!("Failed to create a database file {}", output))?;

    let client = NotionClient { api_key };

    let schema = client
        .get_database(&database_id)
        .with_context(|| "Failed to fetch database schema")?;
    let pages = client
        .get_all_pages(&database_id, &schema)
        .with_context(|| "Failed to fetch pages")?;

    let sqlite = Sqlite::new(&output, &schema).with_context(|| "Failed to connect to sqlite")?;
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
