use crate::database::datenkatalog::{delete_entry, find_by_procedure_id};
use crate::database::Database;
use mysql::prelude::{BinQuery, FromRow, Queryable, WithParams};
use mysql::{params, FromRowError, Row};
use std::fmt::{Display, Formatter};
use time::PrimitiveDateTime;

#[derive(Debug)]
pub struct ProcedureForm {
    pub procedure_id: u64,
    pub procedure_start: Option<PrimitiveDateTime>,
    pub disease_id: Option<u64>,
    pub data_form_id: u64,
    pub data_form_name: String,
}

impl FromRow for ProcedureForm {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        if row.is_empty() {
            return Err(FromRowError(row));
        }

        Ok(ProcedureForm {
            procedure_id: row.get(0).unwrap(),
            procedure_start: row.get(1).unwrap(),
            disease_id: row.get(2).unwrap(),
            data_form_id: row.get(3).unwrap(),
            data_form_name: row.get(4).unwrap(),
        })
    }
}

impl Display for ProcedureForm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Prozedur-ID:  {}\nErkrankung:   {}\nBeginn:       {}\nFormularname: {}",
            self.procedure_id,
            match self.disease_id {
                Some(id) => id.to_string(),
                _ => String::new(),
            },
            match self.procedure_start {
                Some(start) => start.date().to_string(),
                _ => String::new(),
            },
            self.data_form_name
        )
    }
}

pub fn procedures_by_patient_id(db: &Database, patient_id: u64) -> Vec<ProcedureForm> {
    let sql = "SELECT prozedur.id, prozedur.beginndatum, erkrankung_id, data_form.id, data_form.name FROM prozedur \
        JOIN data_form ON prozedur.data_form_id = data_form.id \
        LEFT JOIN erkrankung_prozedur ON prozedur.id = erkrankung_prozedur.prozedur_id \
        WHERE prozedur.hauptprozedur_id IS NULL AND patient_id = :patient_id ORDER BY prozedur.beginndatum";

    if let Ok(result) = db.connection().exec_map(
        sql,
        params! {"patient_id" => patient_id},
        |(procedure_id, procedure_start, disease_id, data_form_id, data_form_name)| ProcedureForm {
            procedure_id,
            procedure_start,
            disease_id,
            data_form_id,
            data_form_name,
        },
    ) {
        return result;
    };

    vec![]
}

pub fn delete(db: &Database, id: u64) -> u64 {
    let mut count = 0;
    find_by_procedure_id(db, id).iter().for_each(|dk| {
        if delete_entry(db, dk.id, id) {
            count += 1;
        }
    });

    if "DELETE FROM prozedur WHERE id = :id"
        .with(params! {"id" => id})
        .run(db.connection())
        .is_ok()
    {
        count += 1
    }

    count
}
