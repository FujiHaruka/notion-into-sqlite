use std::collections::HashMap;

pub struct NotionClient {
    pub api_key: String
}

impl NotionClient {
    pub fn get_database(&self, id: &str) {
        let url = "https://api.notion.com/v1/databases/".to_string() + id;
        let client = reqwest::blocking::Client::new();
        let resp = client.get(url)
            .header("Authorization", "Bearer ".to_string() + &self.api_key)
            .header("Notion-Version", "2022-02-22")
            .send()
            .unwrap()
            .text()
            // .json::<HashMap<String, String>>()
            .unwrap();
        println!("{:#?}", resp);
    }
}
