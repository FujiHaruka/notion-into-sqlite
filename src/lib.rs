pub mod json_util;
pub mod notion_client;
pub mod notion_database;
pub mod notion_pages;
pub mod sqlite;

#[macro_use]
extern crate log;

use crate::notion_client::NotionClient;
use crate::sqlite::Sqlite;
use std::env;

pub fn main() {
    env_logger::init();

    let api_key = env::var("NOTION_API_KEY").unwrap();
    let database_id = env::var("NOTION_DATABASE_ID").unwrap();

    let client = NotionClient { api_key };
    let schema = client.get_database(&database_id).unwrap();
    let pages = client.get_all_pages(&database_id, &schema).unwrap();

    let sqlite = Sqlite::new("notion.db", &schema).unwrap();
    sqlite.create_table().unwrap();

    for page in pages {
        sqlite.insert(&page).unwrap();
    }
}
