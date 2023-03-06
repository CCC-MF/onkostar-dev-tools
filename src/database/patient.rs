use crate::database::Database;
use mysql::params;
use mysql::prelude::Queryable;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct PatientEntity {
    pub id: u64,
    pub vorname: String,
    pub nachname: String,
}

impl Display for PatientEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "ID:           {}\nVorname:      {}\nNachname:     {}\n",
            self.id, self.vorname, self.nachname
        )
    }
}

pub fn query(db: &Database, query: &String) -> Vec<PatientEntity> {
    let sql = "SELECT id, vorname, nachname FROM patient \
            WHERE LOWER(vorname) LIKE :name OR LOWER(nachname) LIKE :name ORDER BY id";

    if let Ok(result) = db.connection().exec_map(
        sql,
        params! {"name" => format!("%{query}%")},
        |(id, vorname, nachname)| PatientEntity {
            id,
            vorname,
            nachname,
        },
    ) {
        return result;
    };

    vec![]
}

pub fn count_non_anonym(db: &Database) -> u64 {
    let sql = "SELECT COUNT(*) FROM patient WHERE vorname NOT LIKE 'Vorname%'";
    if let Ok(count) = db.connection().query_first::<u64, &str>(sql) {
        return match count {
            Some(count) => return count,
            _ => 0,
        };
    }
    0
}

pub fn next_not_anon(db: &Database) -> Result<u64, ()> {
    let sql = "SELECT id FROM patient WHERE vorname NOT LIKE 'Vorname%' LIMIT 1";

    if let Ok(id) = db.connection().query_first::<u64, &str>(sql) {
        return match id {
            Some(id) => return Ok(id),
            _ => Err(()),
        };
    }
    Err(())
}

pub fn anonymize(db: &Database, id: u64) {
    let sql = "UPDATE patient SET \
            geburtsort = 'Musterstadt', \
            krankenkassennummer = 1, \
            namenzusatz = '', \
            vorname = CONCAT('Vorname', id), \
            nachname = CONCAT('Nachname', id), \
            geburtsname = '', \
            fruehere_namen = '', \
            telefon = '', \
            telefon2 = '', \
            email = '', \
            staatsangehoerigkeit = NULL, \
            staat_id = 'DE', \
            ort = 'Musterhausen', \
            postleitzahl = '12345', \
            strasse = 'Musterweg', \
            adresszusatz = '', \
            nachsorgepassnr = '', \
            ahvnummer = '', \
            namensvorsatz = ''\
            WHERE id = :id";

    let _ = db.connection().exec_drop(sql, params! {"id" => id});
}
