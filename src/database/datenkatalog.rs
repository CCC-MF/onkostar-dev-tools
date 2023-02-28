use crate::database::form::{by_data_catalogue_id, FormEntity};
use crate::database::Database;
use crate::ui::SelectDisplay;
use mysql::prelude::{BinQuery, FromRow, Queryable, TextQuery, WithParams};
use mysql::{params, FromRowError, PooledConn, Row};
use regex::Regex;
use std::fmt::{Display, Formatter};

pub struct DatenkatalogEntity {
    pub id: u64,
    pub name: String,
    pub description: String,
}

impl FromRow for DatenkatalogEntity {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        if row.is_empty() {
            return Err(FromRowError(row));
        }

        Ok(DatenkatalogEntity {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            description: row.get(2).unwrap(),
        })
    }
}

impl Display for DatenkatalogEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "ID:           {}\nName:         {}\nBeschreibung: {}",
            self.id, self.name, self.description
        )
    }
}

impl SelectDisplay for DatenkatalogEntity {
    fn to_string(&self) -> String {
        format!("{}: {}", self.id, self.name)
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

pub fn get_by_id(db: &Database, id: u64) -> Option<DatenkatalogEntity> {
    if let Ok(Some(result)) = "SELECT id, name, description FROM data_catalogue WHERE id = :id"
        .with(params! {"id" => id})
        .first::<DatenkatalogEntity, PooledConn>(db.connection())
    {
        Some(result)
    } else {
        None
    }
}

pub fn forms(db: &Database, id: u64) -> Vec<FormEntity> {
    by_data_catalogue_id(db, id)
}

pub fn by_data_form_id(db: &Database, id: u64) -> Vec<DatenkatalogEntity> {
    let sql = "SELECT dc.id, dc.name, dc.description FROM data_catalogue dc \
        JOIN data_form_data_catalogue dfdc ON dc.id = dfdc.data_catalogue_id \
        WHERE dfdc.data_form_id= :id \
        ORDER BY dc.id";

    if let Ok(result) =
        db.connection()
            .exec_map(sql, params! {"id" => id}, |(id, name, description)| {
                DatenkatalogEntity {
                    id,
                    name,
                    description,
                }
            })
    {
        return result;
    }

    vec![]
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
