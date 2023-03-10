use crate::database::Database;
use crate::ui::SelectDisplay;
use mysql::prelude::{BinQuery, FromRow, Queryable, WithParams};
use mysql::{params, FromRowError, PooledConn, Row};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct MerkmalskatalogEntity {
    pub id: u64,
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

impl Display for MerkmalskatalogEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ID:           {}\nName:         {}", self.id, self.name)
    }
}

impl SelectDisplay for MerkmalskatalogEntity {
    fn to_string(&self) -> String {
        format!("{}: {}", self.id, self.name)
    }
}

#[derive(Debug)]
pub struct MerkmalskatalogVersionEntity {
    pub id: u64,
    pub version: u64,
    pub description: String,
}

impl Display for MerkmalskatalogVersionEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "ID:           {}\nBeschreibung: {}",
            self.id, self.version
        )
    }
}

impl SelectDisplay for MerkmalskatalogVersionEntity {
    fn to_string(&self) -> String {
        format!("{}: {}", self.id, self.description)
    }
}

#[derive(Debug)]
pub struct MerkmalskatalogCategoryEntity {
    pub id: u64,
    pub name: String,
    pub beschreibung: String,
}

impl Display for MerkmalskatalogCategoryEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "ID:           {}\nName:         {}\nBeschreibung: {}",
            self.id, self.name, self.beschreibung
        )
    }
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
