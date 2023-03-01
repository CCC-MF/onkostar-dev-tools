use crate::database::Database;
use crate::ui::EntitySelect;
use crate::{database, green_headline, headline, warn};
use console::Term;
use std::process::exit;
use dialoguer::Confirm;
use crate::ui::page::Page;

pub fn show_query_result(db: &Database, query: &String) {
    let mks = database::merkmalskatalog::query(db, query);
    if mks.len() > 50 {
        warn!("Mehr als 50 Einträge, bitte Filter weiter einschränken");
        exit(1);
    } else if mks.is_empty() {
        warn!("Keine Einträge");
        println!();
        return;
    }
    let term = Term::stdout();

    headline!("Merkmalskatalog auswählen");

    if let Ok(Some(selection)) = EntitySelect::new().items(&mks).interact_on_opt(&term) {
        let _ = term.clear_last_lines(1);
        let value = mks.get(selection).unwrap();
        show(db, value.id);
    }
}

pub fn show(db: &Database, id: u64) {
    green_headline!("Merkmalskatalog");
    if let Some(mk) = database::merkmalskatalog::get_by_id(db, id) {
        println!("{}", mk);
        show_versions_result(db, mk.id);
        return;
    }

    warn!("Nicht gefunden");
    println!();
}

pub fn show_versions_result(db: &Database, id: u64) {
    let term = Term::stdout();

    if let Ok(true) = Confirm::new().with_prompt("Version auswählen und anzeigen?").default(false).interact() {
        let _ = term.clear_last_lines(1);
        let versions = database::merkmalskatalog::versions(db, id);
        headline!("Version auswählen");
        if let Ok(Some(selection)) = EntitySelect::new().items(&versions).interact_on_opt(&term) {
            let _ = term.clear_last_lines(1);
            let value = versions.get(selection).unwrap();

            green_headline!("Version des Merkmalskatalogs");
            println!("{}", value);

            let result = database::merkmalskatalog::values(db, value.id);
            Page::with(&result, 4).show("Merkmale");
        }
    }
}
