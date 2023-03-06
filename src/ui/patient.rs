use crate::database::Database;
use crate::{database, warn};
use std::process::exit;
use indicatif::ProgressBar;
use crate::ui::page::Page;

pub fn show_query_result(db: &Database, query: &String) {
    let ps = database::patient::query(db, query);
    if ps.len() > 50 {
        warn!("Mehr als 50 Einträge, bitte Filter weiter einschränken");
        exit(1);
    } else if ps.is_empty() {
        warn!("Keine Einträge");
        println!();
        return;
    }

    Page::with(&ps, 4).show("Patienten");
}

pub fn anonymize(db: &Database) {
    let count = database::patient::count_non_anonym(db);

    println!("Anonymisiere {count} Patienten ...");

    let bar = ProgressBar::new(count);
    for _ in 0..count {
        if let Ok(id) = database::patient::next_not_anon(db) {
            database::patient::anonymize(db, id);
            bar.inc(1);
        }
    }
    bar.finish();
    println!("... fertig!")
}
