use crate::database::Database;
use mysql::params;
use mysql::prelude::Queryable;
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
}
