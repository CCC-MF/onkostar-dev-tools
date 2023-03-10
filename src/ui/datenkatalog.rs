use crate::database::Database;
use crate::ui::page::Page;
use crate::ui::{CustomTheme, EntitySelect};
use crate::{database, green_headline, headline, warn};
use console::{style, Term};
use dialoguer::Select;
use std::process::exit;

pub fn show_query_result(db: &Database, query: &String) {
    let dks = database::datenkatalog::query(db, query);
    if dks.len() > 50 {
        warn!("Mehr als 50 Einträge, bitte Filter weiter einschränken");
        exit(1);
    } else if dks.is_empty() {
        warn!("Keine Einträge");
        println!();
        return;
    }

    let term = Term::stdout();

    headline!("Datenkatalog auswählen");

    if let Ok(Some(selection)) = EntitySelect::new().items(&dks).interact_on_opt(&term) {
        let _ = term.clear_last_lines(1);
        let value = dks.get(selection).unwrap();
        show(db, value.id)
    }
}

pub fn show(db: &Database, id: u64) {
    let term = Term::stdout();
    green_headline!("Datenkatalog");
    if let Some(dk) = database::datenkatalog::get_by_id(db, id) {
        println!("{}", dk);
        headline!("Nächste Aktion auswählen");

        if let Ok(Some(selection)) = Select::with_theme(&CustomTheme::default())
            .items(&["Ende", "Formulare anzeigen", "Prozeduren löschen"])
            .default(0)
            .interact_on_opt(&term)
        {
            let _ = term.clear_last_lines(1);
            match selection {
                1 => show_forms(db, dk.id),
                2 => show_clean_dialogue(db, dk.id),
                _ => {}
            }
        }
        return;
    }

    warn!("Nicht gefunden");
    println!();
}

pub fn show_forms(db: &Database, id: u64) {
    let forms = database::datenkatalog::forms(db, id);
    Page::with(&forms, 4).show("Formulare mit diesem Datenkatalog");
}

pub fn show_clean_dialogue(db: &Database, id: u64) {
    green_headline!("Prozeduren löschen");
    if let Ok(name) = database::datenkatalog::get_name(db, id) {
        let count = database::datenkatalog::clean(db, id);
        if count > 0 {
            println!(
                "Es wurden {} Einträge für '{}' entfernt!",
                style(count).green(),
                style(name).bold()
            );
            return;
        }
    }
    warn!("Keine Einträge");
    println!()
}
