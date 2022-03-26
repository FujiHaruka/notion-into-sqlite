use std::fs;

#[allow(dead_code)]
pub fn before_db(database_path: &str) {
    fs::remove_file(database_path).ok();
    fs::create_dir("tmp").ok();
}
