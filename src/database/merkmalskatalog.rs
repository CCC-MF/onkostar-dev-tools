use crate::database::Database;
use crate::ui::SelectDisplay;
use mysql::prelude::{BinQuery, FromRow, Queryable, WithParams};
use mysql::{params, FromRowError, PooledConn, Row};
use onkostar_entity_macros::DisplayHelper;
use std::fmt::{Display, Formatter};

#[derive(Debug, DisplayHelper)]
pub struct MerkmalskatalogEntity {
    #[display("ID")]
    pub id: u64,
    #[display("Name")]
    pub name: String,
}

impl FromRow for MerkmalskatalogEntity {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        if row.is_empty() {
            return Err(FromRowError(row));
        }

        Ok(MerkmalskatalogEntity {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
        })
    }
}

impl SelectDisplay for MerkmalskatalogEntity {
    fn to_string(&self) -> String {
        format!("{}: {}", self.id, self.name)
    }
}

#[derive(Debug, DisplayHelper)]
pub struct MerkmalskatalogVersionEntity {
    #[display("ID")]
    pub id: u64,
    #[display("Beschreibung")]
    pub version: u64,
    pub description: String,
}

impl SelectDisplay for MerkmalskatalogVersionEntity {
    fn to_string(&self) -> String {
        format!("{}: {}", self.id, self.description)
    }
}

#[derive(Debug, DisplayHelper)]
pub struct MerkmalskatalogCategoryEntity {
    #[display("ID")]
    pub id: u64,
    #[display("Name")]
    pub name: String,
    #[display("Beschreibung")]
    pub beschreibung: String,
}

pub fn query(db: &Database, query: &String) -> Vec<MerkmalskatalogEntity> {
    let sql = "SELECT id, name FROM property_catalogue \
            WHERE LOWER(name) LIKE :name ORDER BY id";

    if let Ok(result) = db.connection().exec_map(
        sql,
        params! {"name" => format!("%{query}%")},
        |(id, name)| MerkmalskatalogEntity { id, name },
    ) {
        return result;
    };

    vec![]
}

pub fn get_by_id(db: &Database, id: u64) -> Option<MerkmalskatalogEntity> {
    if let Ok(Some(result)) = "SELECT id, name FROM property_catalogue WHERE id = :id"
        .with(params! {"id" => id})
        .first::<MerkmalskatalogEntity, PooledConn>(db.connection())
    {
        Some(result)
    } else {
        None
    }
}

pub fn versions(db: &Database, id: u64) -> Vec<MerkmalskatalogVersionEntity> {
    let sql = "SELECT id, version_number, description FROM property_catalogue_version \
            WHERE datacatalog_id = :id ORDER BY id";

    if let Ok(result) =
        db.connection()
            .exec_map(sql, params! {"id" => id}, |(id, version, description)| {
                MerkmalskatalogVersionEntity {
                    id,
                    version,
                    description,
                }
            })
    {
        return result;
    };

    vec![]
}

pub fn values(db: &Database, version_id: u64) -> Vec<MerkmalskatalogCategoryEntity> {
    let sql = "SELECT id, name, beschreibung FROM property_catalogue_category \
            WHERE version_id = :id ORDER BY id";

    if let Ok(result) = db.connection().exec_map(
        sql,
        params! {"id" => version_id},
        |(id, name, beschreibung)| MerkmalskatalogCategoryEntity {
            id,
            name,
            beschreibung,
        },
    ) {
        return result;
    };

    vec![]
}
