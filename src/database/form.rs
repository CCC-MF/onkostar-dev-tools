use crate::database::datenkatalog::{find_by_data_form_id, DatenkatalogEntity};
use crate::database::Database;
use crate::ui::SelectDisplay;
use mysql::prelude::{BinQuery, FromRow, Queryable, WithParams};
use mysql::{params, FromRowError, PooledConn, Row};
use onkostar_entity_macros::DisplayHelper;
use std::fmt::{Display, Formatter};

#[derive(DisplayHelper)]
pub struct FormEntity {
    #[display(name = "ID")]
    pub id: u64,
    #[display(name = "Name")]
    pub name: String,
    #[display(name = "Beschreibung")]
    pub description: String,
}

impl FromRow for FormEntity {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        if row.is_empty() {
            return Err(FromRowError(row));
        }

        Ok(FormEntity {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            description: row.get(2).unwrap(),
        })
    }
}

impl SelectDisplay for FormEntity {
    fn to_string(&self) -> String {
        format!("{}: {}", self.id, self.name)
    }
}

pub fn query(db: &Database, query: &String) -> Vec<FormEntity> {
    let sql = "SELECT id, name, description FROM data_form \
        WHERE LOWER(name) LIKE :name";

    if let Ok(result) = db.connection().exec_map(
        sql,
        params! {"name" => format!("%{query}%")},
        |(id, name, description)| FormEntity {
            id,
            name,
            description,
        },
    ) {
        return result;
    }

    vec![]
}

pub fn get_by_id(db: &Database, id: u64) -> Option<FormEntity> {
    if let Ok(Some(result)) = "SELECT id, name, description FROM data_form WHERE id = :id"
        .with(params! {"id" => id})
        .first::<FormEntity, PooledConn>(db.connection())
    {
        Some(result)
    } else {
        None
    }
}

pub fn by_data_catalogue_id(db: &Database, id: u64) -> Vec<FormEntity> {
    let sql = "SELECT df.id, df.name, df.description FROM data_form_data_catalogue dc \
        JOIN data_form df ON dc.data_form_id = df.id \
        WHERE dc.data_catalogue_id = :id \
        ORDER BY df.id";

    if let Ok(result) =
        db.connection()
            .exec_map(sql, params! {"id" => id}, |(id, name, description)| {
                FormEntity {
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

pub fn subforms(db: &Database, id: u64) -> Vec<FormEntity> {
    let sql = "SELECT sub.id, sub.name, sub.description FROM data_form_entry dfe \
        JOIN data_form sub on dfe.referenced_data_form = sub.id \
        WHERE dfe.data_form_id = :id and dfe.type = 'subform' ORDER BY sub.id";

    if let Ok(result) =
        db.connection()
            .exec_map(sql, params! {"id" => id}, |(id, name, description)| {
                FormEntity {
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

pub fn data_catalogues(db: &Database, id: u64) -> Vec<DatenkatalogEntity> {
    find_by_data_form_id(db, id)
}
