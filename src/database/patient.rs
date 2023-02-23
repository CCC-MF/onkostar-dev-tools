use crate::database::Database;
use mysql::params;
use mysql::prelude::Queryable;

pub struct Patient;

impl Patient {
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

    pub fn next(db: &Database) -> Result<u64, ()> {
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
}
