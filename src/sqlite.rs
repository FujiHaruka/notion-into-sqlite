use std::{collections::HashMap, fs, path::Path};

use crate::{
    notion_database::{NotionDatabaseSchema, NotionPropertyType},
    notion_pages::{NotionPage, NotionPropertyValue},
};
use anyhow::{anyhow, Result};
use rusqlite::{params, params_from_iter, Connection};

pub static PAGE_METADATA_TABLE: &str = "page_metadata";
pub static PAGE_PROPERTIES_TABLE: &str = "pages";
pub static PAGE_ID_COLUMN: &str = "page_id";

/// Resolve SQLite's column name from Notion's property name
struct ColumnNames {
    hash: HashMap<String, String>,
}
impl ColumnNames {
    fn new(schema: &NotionDatabaseSchema) -> ColumnNames {
        let mut hash = HashMap::new();
        for property in schema.properties.values() {
            let column = property.name.replace('"', "\"\"");
            hash.insert(property.name.to_string(), column);
        }
        ColumnNames { hash }
    }

    /// Resolve SQLite's column name
    fn resolve(&self, notion_property_name: &str) -> &str {
        self.hash.get(notion_property_name).unwrap()
    }
}

pub struct Sqlite<'a> {
    pub conn: Connection,
    pub schema: &'a NotionDatabaseSchema,
    column_names: ColumnNames,
}
impl Sqlite<'_> {
    pub fn new<'a>(path: &str, schema: &'a NotionDatabaseSchema) -> Result<Sqlite<'a>> {
        let conn = Connection::open(path)?;
        let column_names = ColumnNames::new(schema);
        Ok(Sqlite {
            conn,
            schema,
            column_names,
        })
    }

    /// Check if database file can be created
    pub fn validate_database_path(path: &str) -> Result<()> {
        if Path::new(path).exists() {
            return Err(anyhow!("{} already exists", path));
        }

        let conn = Connection::open(path)?;
        match conn.close() {
            Ok(_) => {
                // Delete file created by the connection because Connection::open() is just used for validation
                fs::remove_file(path).ok();
                Ok(())
            }
            Err((_, err)) => Err(anyhow!(err.to_string())),
        }
    }

    pub fn create_tables(&self) -> Result<()> {
        // Create page properties table
        let table_definition = self.table_definitin_from();
        let sql = format!(
            "CREATE TABLE {table_name} (
                {id_column} TEXT PRIMARY KEY,
                {definition}
            )",
            table_name = PAGE_PROPERTIES_TABLE,
            id_column = PAGE_ID_COLUMN,
            definition = table_definition,
        );
        debug!("{}", sql);
        self.conn.execute(&sql, [])?;

        // Create page metadata table
        let sql = format!(
            "CREATE TABLE {table_name} (
                id TEXT PRIMARY KEY,
                url TEXT,
                created_time TEXT,
                created_by JSON,
                last_edited_time TEXT,
                last_edited_by JSON,
                archived BOOLEAN
            )",
            table_name = PAGE_METADATA_TABLE,
        );
        debug!("{}", sql);
        self.conn.execute(&sql, [])?;
        Ok(())
    }

    pub fn insert(&self, page: &NotionPage) -> Result<()> {
        // Insert properties of page
        let mut property_names = vec![PAGE_ID_COLUMN];
        for name in page.properties.keys() {
            property_names.push(name);
        }
        let sql = self.create_insert_sql_for(&property_names);
        debug!("{}", sql);
        let page_id = NotionPropertyValue::Text(page.id.clone());
        let sql_params = params_from_iter(property_names.iter().map(|&column| {
            if column == PAGE_ID_COLUMN {
                &page_id
            } else {
                page.properties.get(column).unwrap()
            }
        }));
        debug!("Parameters: {:?}", sql_params);
        self.conn.execute(&sql, sql_params)?;

        // Insert page metadata
        let sql = format!(
            "INSERT INTO {table_name} (
                id,
                url,
                created_time,
                created_by,
                last_edited_time,
                last_edited_by,
                archived
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            table_name = PAGE_METADATA_TABLE,
        );
        let sql_params = params![
            page.id,
            page.url,
            page.created_time,
            page.created_by.to_string(),
            page.last_edited_time,
            page.last_edited_by.to_string(),
            page.archived
        ];
        self.conn.execute(&sql, sql_params)?;

        Ok(())
    }

    /// Get table definistion string from the schema object.
    /// It's a part of SQL query specified in {{}}:
    /// CREATE TABLE notion (page_id TEXT PRIMARY KEY, {{"Animal" TEXT, "Age" REAL, "Name" TEXT}})
    fn table_definitin_from(&self) -> String {
        self.schema
            .properties
            .iter()
            .map(|(_, property)| {
                let column = self.column_names.resolve(&property.name);
                let data_type = match property.property_type {
                    NotionPropertyType::Title => "TEXT",
                    NotionPropertyType::Number => "REAL",
                    NotionPropertyType::Select => "TEXT",
                    NotionPropertyType::Other => "TEXT",
                };
                format!(r#""{column}" {data_type}"#)
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Create sql like "INSERT INTO {} (id, title) values (?1, ?2)"
    fn create_insert_sql_for(&self, properties: &[&str]) -> String {
        let columns_formatted = properties
            .iter()
            .map(|&property_name| {
                if property_name == PAGE_ID_COLUMN {
                    property_name.to_string()
                } else {
                    let column = self.column_names.resolve(property_name);
                    format!(r#""{column}""#)
                }
            })
            .collect::<Vec<_>>();
        let placeholders = (1..(columns_formatted.len() + 1))
            .map(|index| format!("?{}", index))
            .collect::<Vec<_>>();

        format!(
            "INSERT INTO {table_name} ({columns}) VALUES ({values})",
            table_name = &PAGE_PROPERTIES_TABLE,
            columns = columns_formatted.join(", "),
            values = placeholders.join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn validate_database_path() {
        fs::create_dir("tmp").ok();

        let valid_path = "./tmp/a.db";
        let result = Sqlite::validate_database_path(valid_path);
        assert!(result.is_ok());
        let invalid_path = "tmp/foo/bar/a.db";
        let result = Sqlite::validate_database_path(invalid_path);
        assert!(result.is_err());
    }
}
