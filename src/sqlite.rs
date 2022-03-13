use crate::notion_database_schema::{NotionDatabaseSchema, NotionPropertyType};
use rusqlite::{named_params, Connection, Result};

pub struct Sqlite<'a> {
    conn: Connection,
    schema: &'a NotionDatabaseSchema,
}
impl Sqlite<'_> {
    pub fn new<'a>(path: &str, schema: &'a NotionDatabaseSchema) -> Result<Sqlite<'a>> {
        let conn = Connection::open(path)?;

        Ok(Sqlite { conn, schema })
    }

    pub fn create_table(&self, table_name: &str) -> Result<()> {
        let table_definition = self.table_definitin_from();
        self.conn.execute(
            &format!(
                "CREATE TABLE {} (id TEXT PRIMARY KEY, {})",
                table_name, table_definition
            ),
            [],
        )?;
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
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;

    use crate::notion_database_schema::NotionProperty;

    use super::*;

    fn cleanup(database_path: &str) {
        fs::remove_file(database_path).ok();
    }

    #[test]
    fn test_create_table() {
        let database_path = "tmp/test.db";
        cleanup(database_path);

        let schema = NotionDatabaseSchema {
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
        };

        let sqlite = Sqlite::new(database_path, &schema).unwrap();
        sqlite.create_table("notion_table").unwrap();

        let (table_name, sql): (String, String) = sqlite
            .conn
            .query_row("SELECT name, sql FROM sqlite_master", [], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })
            .unwrap();
        assert_eq!(table_name, "notion_table");
        assert!(sql.contains(r#""Name" TEXT"#));
        assert!(sql.contains(r#""Animal" TEXT"#));
        assert!(sql.contains(r#""Age" REAL"#));
    }
}
