use crate::database::datenkatalog::{by_data_form_id, DatenkatalogEntity};
use crate::database::Database;
use crate::ui::SelectDisplay;
use mysql::params;
use mysql::prelude::Queryable;
use std::fmt::{Display, Formatter};

pub struct FormEntity {
    pub id: u64,
    pub name: String,
    pub description: String,
}

impl Display for FormEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "ID:           {}\nName:         {}\nBeschreibung: {}",
            self.id, self.name, self.description
        )
    }
}

impl SelectDisplay for FormEntity {
    fn to_string(&self) -> String {
        format!("{}: {}", self.id, self.name)
    }
}

pub fn query(db: &Database, query: &String) -> Vec<FormEntity> {
    let sql = "SELECT id, name, description FROM data_form \
        WHERE name LIKE :name";

    if let Ok(result) = db.connection().exec_map(
        sql,
        params! {"name" => format!("{query}%")},
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
    by_data_form_id(db, id)
}
