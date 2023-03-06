use crate::database::Database;
use mysql::params;
use mysql::prelude::{BinQuery, Queryable, WithParams};

#[derive(Debug)]
pub struct DiseaseEntity {
    pub id: u64,
    pub patient_id: u64,
    pub icd10_code: Option<String>,
    pub icd10_version: Option<u64>,
}

pub fn find_by_patient_id(db: &Database, patient_id: u64) -> Vec<DiseaseEntity> {
    let sql = "SELECT id, patient_id, icd10_code, icd10_version FROM erkrankung \
        WHERE patient_id = :patient_id";

    if let Ok(result) = db.connection().exec_map(
        sql,
        params! {"patient_id" => patient_id},
        |(id, patient_id, icd10_code, icd10_version)| DiseaseEntity {
            id,
            patient_id,
            icd10_code,
            icd10_version,
        },
    ) {
        return result;
    }

    vec![]
}

pub fn delete(db: &Database, id: u64) -> bool {
    "DELETE FROM erkrankung WHERE id = :id"
        .with(params! {"id" => id})
        .run(db.connection())
        .is_ok()
}
