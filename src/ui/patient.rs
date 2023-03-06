use crate::database::Database;
use crate::{database, green_headline, headline, warn};
use std::process::exit;
use console::Term;
use dialoguer::Select;
use indicatif::ProgressBar;
use crate::ui::{CustomTheme, EntitySelect};
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

    let term = Term::stdout();

    headline!("Patient auswählen");

    if let Ok(Some(selection)) = EntitySelect::new().items(&ps).interact_on_opt(&term) {
        let _ = term.clear_last_lines(1);
        let value = ps.get(selection).unwrap();
        show(db, value.id)
    }
}

pub fn show(db: &Database, id: u64) {
    let term = Term::stdout();
    green_headline!("Patient");
    if let Some(p) = database::patient::get_by_id(db, id) {
        println!("{}", p);
        headline!("Nächste Aktion auswählen");

        if let Ok(Some(selection)) = Select::with_theme(&CustomTheme::default())
            .items(&["Ende"])
            .default(0)
            .interact_on_opt(&term)
        {
            match selection {
                _ => {
                    let _ = term.clear_last_lines(1);
                }
            }
        }
        return
    }

    warn!("Nicht gefunden");
    println!();
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
