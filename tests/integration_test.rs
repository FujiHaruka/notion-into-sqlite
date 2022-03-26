mod common;

extern crate notion_into_sqlite;

use regex::Regex;
use std::env;
use std::error::Error;
use std::fs;
use std::process::Command;

use common::helpers::before_db;

#[test]
fn snapshot_test() -> Result<(), Box<dyn Error>> {
    let api_key = env::var("NOTION_API_KEY")?;
    let database_id = env::var("NOTION_DATABASE_ID")?;
    let output = "tmp/snapshot1.db";
    before_db(output);

    notion_into_sqlite::main(&api_key, &database_id, output)?;

    // depending on sqlite3 command is not so good
    let dump = Command::new("sqlite3")
        .args([output, "-header", "select page_id, LastEditedTime, RichText, Relation, Phone, CreatedTime, Email, Formula, LastEditedBy, Files, Name, MultiSelect, Date, People, \"Select\", CreatedBy, Number, Checkbox, WebSite, Rollup, id, url, created_time, created_by, last_edited_time, last_edited_by, archived from pages inner join page_metadata on pages.page_id = page_metadata.id"])
        .output()?;
    if !dump.status.success() {
        panic!("{}", String::from_utf8(dump.stderr).unwrap());
    }
    let snapshot = sanitize_file_object(&String::from_utf8(dump.stdout)?);

    // compare result line by line
    let snapshot_lines = snapshot.split("\n").collect::<Vec<&str>>();

    let expected_snapshot =
        sanitize_file_object(&fs::read_to_string("tests/fixtures/snapshot1.txt")?);
    let expected_snapshot_lines = expected_snapshot.split("\n").collect::<Vec<&str>>();
    for (i, line) in expected_snapshot_lines.iter().enumerate() {
        assert_eq!(line, snapshot_lines.get(i).unwrap());
    }

    Ok(())
}

fn sanitize_file_object(text: &str) -> String {
    let file_url_pattern = Regex::new("https://s3[^\"]+").unwrap();
    let mut result = file_url_pattern.replace(text, "<file_url>").to_string();

    // "expiry_time\":\"2022-03-26T13:55:14.661Z\"
    let file_expiration_pattern = Regex::new(r#""expiry_time":"[^"]+""#).unwrap();
    result = file_expiration_pattern
        .replace(&result, r#""expiry_time":"<date_time>""#)
        .to_string();
    result
}
