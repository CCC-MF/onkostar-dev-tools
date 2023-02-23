use crate::database::Database;
use mysql::prelude::{BinQuery, Queryable, TextQuery, WithParams};
use mysql::{params, PooledConn};
use regex::Regex;
use std::fmt::{Display, Formatter};

pub struct DatenkatalogEntity {
    pub id: u64,
    pub name: String,
    pub description: String,
}

impl Display for DatenkatalogEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.name)
    }
}

pub struct Datenkatalog;

impl Datenkatalog {
    pub fn query(db: &Database, query: &String) -> Vec<DatenkatalogEntity> {
        let sql = "SELECT id, name, description FROM data_catalogue WHERE name LIKE :name";

        if let Ok(result) = db.connection().exec_map(
            sql,
            params! {"name" => format!("{query}%")},
            |(id, name, description)| DatenkatalogEntity {
                id,
                name,
                description,
            },
        ) {
            return result;
        }

        vec![]
    }

    pub fn clean(db: &Database, id: u64) -> bool {
        if let Ok(Some(name)) = "SELECT name FROM data_catalogue WHERE id LIKE :id"
            .with(params! {"id" => id})
            .first::<String, PooledConn>(db.connection())
        {
            let name_re = Regex::new(r"[[:^alpha:]]").unwrap();
            let table_name = &format!("dk_{}", name_re.replace(name.as_str(), "_").to_lowercase());

            return format!(
                "DELETE FROM prozedur WHERE id IN (SELECT id FROM {}); DELETE FROM {}",
                table_name, table_name
            )
            .run(db.connection())
            .is_ok();
        };

        false
    }
}
