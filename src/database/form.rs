use crate::database::Database;
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
        write!(f, "{}: {}", self.id, self.name)
    }
}

pub fn by_data_catalogue_id(db: &Database, id: u64) -> Vec<FormEntity> {
    let sql = "SELECT df.id, df.name, df.description from data_form_data_catalogue dc \
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
