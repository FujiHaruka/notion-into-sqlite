use std::collections::HashMap;

use crate::{
    notion_database::{NotionDatabaseSchema, NotionPropertyType},
    notion_list::{NotionEntry, NotionPropertyValue},
};
use rusqlite::{params_from_iter, Connection, Result};

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
    pub table_name: String,
    pub schema: &'a NotionDatabaseSchema,
    column_names: ColumnNames,
}
impl Sqlite<'_> {
    pub fn new<'a>(path: &str, schema: &'a NotionDatabaseSchema) -> Result<Sqlite<'a>> {
        let conn = Connection::open(path)?;
        // for now, fixed table name
        let table_name = "notion".to_string();
        let column_names = ColumnNames::new(schema);
        Ok(Sqlite {
            conn,
            table_name,
            schema,
            column_names,
        })
    }

    pub fn create_table(&self) -> Result<()> {
        let table_definition = self.table_definitin_from();
        let sql = format!(
            "CREATE TABLE {table_name} (id TEXT PRIMARY KEY, {definition})",
            table_name = self.table_name,
            definition = table_definition,
        );
        debug!("{}", sql);
        self.conn.execute(&sql, [])?;
        Ok(())
    }

    pub fn insert(&self, entry: &NotionEntry) -> Result<()> {
        let mut property_names = vec!["id"];
        for name in entry.properties.keys() {
            property_names.push(name);
        }
        let sql = self.create_insert_sql_for(&property_names);
        debug!("{}", sql);
        let entry_id = NotionPropertyValue::Text(entry.id.clone());
        let sql_params = params_from_iter(property_names.iter().map(|&column| {
            if column == "id" {
                &entry_id
            } else {
                entry.properties.get(column).unwrap()
            }
        }));
        debug!("Parameters: {:?}", sql_params);
        self.conn.execute(&sql, sql_params)?;

        Ok(())
    }

    /// Get table definistion string from the schema object.
    /// It's a part of SQL query specified in {{}}:
    /// CREATE TABLE notion (id TEXT PRIMARY KEY, {{"Animal" TEXT, "Age" REAL, "Name" TEXT}})
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
                if property_name == "id" {
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
            table_name = &self.table_name,
            columns = columns_formatted.join(", "),
            values = placeholders.join(", ")
        )
    }
}
