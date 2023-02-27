use crate::database::Database;
use mysql::prelude::{BinQuery, Queryable, TextQuery, WithParams};
use mysql::{params, PooledConn};
use regex::Regex;
use std::fmt::{Display, Formatter};
use crate::database::form::{by_data_catalogue_id, FormEntity};

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

pub fn forms(db: &Database, id: u64) -> Vec<FormEntity> {
    by_data_catalogue_id(db, id)
}

pub fn get_name(db: &Database, id: u64) -> Result<String, ()> {
    if let Ok(Some(name)) = "SELECT name FROM data_catalogue WHERE id LIKE :id"
        .with(params! {"id" => id})
        .first::<String, PooledConn>(db.connection())
    {
        return Ok(name);
    }
    Err(())
}

pub fn clean(db: &Database, id: u64) -> u64 {
    if let Ok(name) = get_name(db, id) {
        let name_re = Regex::new(r"[[:^alpha:]]").unwrap();
        let table_name = &format!("dk_{}", name_re.replace(name.as_str(), "_").to_lowercase());

        if let Ok(Some(count)) =
            format!("SELECT COUNT(*) FROM {}", table_name).first(db.connection())
        {
            if format!(
                "DELETE FROM prozedur WHERE id IN (SELECT id FROM {}); DELETE FROM {}",
                table_name, table_name
            )
            .run(db.connection())
            .is_ok()
            {
                return count;
            }
        }
    };

    0
}
