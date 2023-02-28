use crate::database::Database;
use crate::ui::EntitySelect;
use crate::{database, green_headline, headline, warn};
use console::Term;
use std::process::exit;

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
        green_headline!("Merkmalskatalog");
        println!("{}", value);
        show_versions_result(db, value.id);
    }
}

pub fn show_versions_result(db: &Database, id: u128) {
    let versions = database::merkmalskatalog::versions(db, id);

    let term = Term::stdout();

    headline!("Version auswählen");

    if let Ok(Some(selection)) = EntitySelect::new().items(&versions).interact_on_opt(&term) {
        let _ = term.clear_last_lines(1);
        let value = versions.get(selection).unwrap();

        green_headline!("Version des Merkmalskatalogs");
        println!("{}", value);

        green_headline!("Merkmale");
        let result = database::merkmalskatalog::values(db, value.id);
        if result.is_empty() {
            warn!("Keine Einträge");
            println!();
            return;
        }
        result.into_iter().for_each(|value| {
            println!("{}", value);
        })
    }
}
