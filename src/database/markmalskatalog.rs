use crate::database::Database;
use mysql::params;
use mysql::prelude::Queryable;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct MerkmalskatalogEntity {
    pub id: u128,
    pub name: String,
}

impl Display for MerkmalskatalogEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.name)
    }
}

#[derive(Debug)]
pub struct MerkmalskatalogVersionEntity {
    pub id: u128,
    pub version: u64,
    pub description: String,
}

impl Display for MerkmalskatalogVersionEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.description)
    }
}

#[derive(Debug)]
pub struct MerkmalskatalogCategoryEntity {
    pub id: u128,
    pub name: String,
    pub beschreibung: String,
}

pub struct Merkmalskatalog;

impl Merkmalskatalog {
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

    pub fn versions(db: &Database, id: u128) -> Vec<MerkmalskatalogVersionEntity> {
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

    pub fn values(db: &Database, version_id: u128) -> Vec<MerkmalskatalogCategoryEntity> {
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
}
