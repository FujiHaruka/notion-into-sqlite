pub mod json_util;
pub mod notion_client;
pub mod notion_database;
pub mod notion_pages;
pub mod sqlite;

#[macro_use]
extern crate log;

use crate::notion_client::NotionClient;
use crate::sqlite::Sqlite;
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

pub fn main() {
    env_logger::init();

    let args = Args::parse();
    let api_key = args.api_key;
    let database_id = args.database_id;
    let output = args.output;

    // TODO: check output path

    let client = NotionClient { api_key };

    let schema = client
        .get_database(&database_id)
        .expect("Failed to fetch database schema");
    let pages = client
        .get_all_pages(&database_id, &schema)
        .expect("Failed to fetch pages");

    let sqlite = Sqlite::new(&output, &schema).expect("Failed to connect to sqlite");
    sqlite.create_tables().expect("Failed to craete tables");

    for page in pages {
        sqlite
            .insert(&page)
            .expect("Failed to insert pages to sqlite");
    }
}
