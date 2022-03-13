mod json_util;
mod notion_client;
mod notion_database_schema;
mod notion_list;
mod sqlite;

#[macro_use]
extern crate log;

use crate::notion_client::NotionClient;
use std::env;

fn main() {
    env_logger::init();

    let api_key = env::var("NOTION_API_KEY").unwrap();
    let database_id = env::var("NOTION_DATABASE_ID").unwrap();

    let client = NotionClient { api_key };
    let schema = client.get_database(&database_id).unwrap();
    let list = client.get_all_entries(&database_id, &schema).unwrap();

    println!("spam!spam!spam!spam!{:?}", list);
}
