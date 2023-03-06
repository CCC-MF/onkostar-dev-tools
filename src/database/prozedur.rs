use crate::database::Database;
use mysql::prelude::{FromRow, Queryable};
use mysql::{params, FromRowError, Row};
use std::fmt::{Display, Formatter};
use time::PrimitiveDateTime;

#[derive(Debug)]
pub struct ProcedureForm {
    pub procedure_id: u64,
    pub procedure_start: PrimitiveDateTime,
    pub disease_id: u64,
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

        println!(">> {:?}", row);

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
            self.disease_id,
            self.procedure_start.date(),
            self.data_form_name
        )
    }
}

pub fn procedures_by_patient_id(db: &Database, patient_id: u64) -> Vec<ProcedureForm> {
    let sql = "SELECT prozedur.id, prozedur.beginndatum, erkrankung_id, data_form.id, data_form.name FROM prozedur \
        JOIN data_form ON prozedur.data_form_id = data_form.id \
        JOIN erkrankung_prozedur ON prozedur.id = erkrankung_prozedur.prozedur_id \
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

pub fn delete_procedure(db: &Database, id: u64) {

}