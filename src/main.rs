extern crate notion_into_sqlite;
use anyhow::Result;
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

fn main() -> Result<()> {
    let args = Args::parse();
    let api_key = args.api_key;
    let database_id = args.database_id;
    let output = args.output;

    notion_into_sqlite::main(&api_key, &database_id, &output)
}
