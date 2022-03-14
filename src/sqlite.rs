use crate::{
    notion_database::{NotionDatabaseSchema, NotionPropertyType},
    notion_list::{NotionEntry, NotionPropertyValue},
};
use rusqlite::{params_from_iter, Connection, Result};

pub struct Sqlite<'a> {
    pub conn: Connection,
    pub table_name: String,
    pub schema: &'a NotionDatabaseSchema,
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
