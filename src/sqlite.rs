use crate::{
    notion_database::{NotionDatabaseSchema, NotionPropertyType},
    notion_list::{NotionEntry, NotionPropertyValue},
};
use rusqlite::{params_from_iter, Connection, Result};

pub struct Sqlite<'a> {
    conn: Connection,
    table_name: String,
    schema: &'a NotionDatabaseSchema,
}
impl Sqlite<'_> {
    pub fn new<'a>(path: &str, schema: &'a NotionDatabaseSchema) -> Result<Sqlite<'a>> {
        let conn = Connection::open(path)?;
        // for now, fixed table name
        let table_name = "notion".to_string();
        Ok(Sqlite {
            conn,
            table_name,
            schema,
        })
    }

    pub fn create_table(&self) -> Result<()> {
        let table_definition = self.table_definitin_from();
        let sql = format!(
            "CREATE TABLE {} (id TEXT PRIMARY KEY, {})",
            self.table_name, table_definition
        );
        debug!("{}", sql);
        self.conn.execute(&sql, [])?;
        Ok(())
    }

    pub fn insert(&self, entry: &NotionEntry) -> Result<()> {
        let mut columns = vec!["id"];
        for column in entry.properties.keys() {
            columns.push(column);
        }
        let sql = self.create_insert_sql_for(&columns);
        debug!("{}", sql);
        let entry_id = NotionPropertyValue::Text(entry.id.clone());
        let sql_params = params_from_iter(columns.iter().map(|&column| {
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

    fn table_definitin_from(&self) -> String {
        self.schema
            .properties
            .iter()
            .map(|(_, property)| {
                let data_type = match property.property_type {
                    NotionPropertyType::Title => "TEXT",
                    NotionPropertyType::Number => "REAL",
                    NotionPropertyType::Select => "TEXT",
                    NotionPropertyType::Other => "TEXT",
                };

                format!(r#""{}" {}"#, property.name, data_type)
            })
            .collect::<Vec<_>>()
            .join(", ")
            .to_string()
    }

    /// Create sql like "INSERT INTO {} (id, title) values (?1, ?2)"
    fn create_insert_sql_for(&self, columns: &Vec<&str>) -> String {
        let columns_formatted = columns
            .iter()
            .map(|&column| {
                if column == "id" {
                    column.to_string()
                } else {
                    format!(r#""{}""#, column)
                }
            })
            .collect::<Vec<_>>();
        let placeholders = (1..(columns_formatted.len() + 1))
            .map(|index| format!("?{}", index))
            .collect::<Vec<_>>();

        format!(
            "INSERT INTO {} ({}) VALUES ({})",
            &self.table_name,
            columns_formatted.join(", "),
            placeholders.join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use std::collections::HashMap;
    use std::fs;

    use crate::notion_database::NotionProperty;

    use super::*;

    fn cleanup(database_path: &str) {
        fs::remove_file(database_path).ok();
    }

    #[test]
    #[serial]
    fn test_create_table() {
        let database_path = "tmp/test.db";
        cleanup(database_path);

        let schema = get_schema();
        let sqlite = Sqlite::new(database_path, &schema).unwrap();
        sqlite.create_table().unwrap();

        let (table_name, sql): (String, String) = sqlite
            .conn
            .query_row("SELECT name, sql FROM sqlite_master", [], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })
            .unwrap();
        assert_eq!(table_name, "notion");
        assert!(sql.contains(r#""Name" TEXT"#));
        assert!(sql.contains(r#""Animal" TEXT"#));
        assert!(sql.contains(r#""Age" REAL"#));
    }

    #[test]
    #[serial]
    fn test_insert() {
        let database_path = "tmp/test.db";
        cleanup(database_path);

        let schema = get_schema();
        let sqlite = Sqlite::new(database_path, &schema).unwrap();
        sqlite.create_table().unwrap();

        let entry = NotionEntry {
            id: "xxxx".to_string(),
            properties: HashMap::from([
                (
                    "Name".to_string(),
                    NotionPropertyValue::Text("Meu".to_string()),
                ),
                ("Age".to_string(), NotionPropertyValue::Number(5.0)),
            ]),
        };
        sqlite.insert(&entry).unwrap();

        let (id, name, age): (String, String, f64) = sqlite
            .conn
            .query_row(r#"SELECT id,"Name","Age" from notion"#, [], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })
            .unwrap();
        assert_eq!(id, "xxxx");
        assert_eq!(name, "Meu");
        assert_eq!(age, 5.0);
    }

    fn get_schema() -> NotionDatabaseSchema {
        NotionDatabaseSchema {
            properties: HashMap::from([
                (
                    "Name".to_string(),
                    NotionProperty {
                        name: "Name".to_string(),
                        property_raw_type: "title".to_string(),
                        property_type: NotionPropertyType::Title,
                    },
                ),
                (
                    "Age".to_string(),
                    NotionProperty {
                        name: "Age".to_string(),
                        property_raw_type: "number".to_string(),
                        property_type: NotionPropertyType::Number,
                    },
                ),
                (
                    "Animal".to_string(),
                    NotionProperty {
                        name: "Animal".to_string(),
                        property_raw_type: "select".to_string(),
                        property_type: NotionPropertyType::Select,
                    },
                ),
            ]),
        }
    }
}
