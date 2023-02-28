use crate::database::Database;
use crate::ui::{CustomTheme, EntitySelect};
use crate::{database, green_headline, headline, warn};
use console::{style, Term};
use dialoguer::Select;
use std::process::exit;

pub fn show_query_result(db: &Database, query: &String) {
    let forms = database::form::query(db, query);
    if forms.len() > 50 {
        warn!("Mehr als 50 Einträge, bitte Filter weiter einschränken");
        exit(1);
    } else if forms.is_empty() {
        warn!("Keine Einträge");
        println!();
        return;
    }

    let term = Term::stdout();

    headline!("Formular auswählen");

    if let Ok(Some(selection)) = EntitySelect::new().items(&forms).interact_on_opt(&term) {
        let _ = term.clear_last_lines(1);
        let value = forms.get(selection).unwrap();
        green_headline!("Formular");
        println!("{}", value);
        headline!("Nächste Aktion auswählen");

        if let Ok(Some(selection)) = Select::with_theme(&CustomTheme::default())
            .items(&[
                "Ende",
                "Verwendete Datenkataloge anzeigen",
                "Alle Prozeduren löschen",
            ])
            .default(0)
            .interact_on_opt(&term)
        {
            match selection {
                1 => show_data_catalogues(db, value.id),
                2 => {
                    let _ = term.clear_last_lines(1);
                    show_clean_dialogue(db, value.id)
                }
                _ => {
                    let _ = term.clear_last_lines(1);
                }
            }
        }
    }
}

pub fn show_data_catalogues(db: &Database, id: u64) {
    let term = Term::stdout();
    let _ = term.clear_last_lines(1);

    green_headline!("Datenkataloge dieses Formulars");

    let forms = database::form::data_catalogues(db, id);

    if forms.is_empty() {
        warn!("Keine Einträge");
        println!();
        return;
    }

    for form in forms {
        println!("{}", form);
    }
}

pub fn show_clean_dialogue(db: &Database, id: u64) {
    green_headline!("Prozeduren aus Datenkatalogen löschen");

    let dcs = database::form::data_catalogues(db, id);

    if dcs.is_empty() {
        warn!("Keine Einträge");
        println!();
        return;
    }

    for dc in dcs {
        if let Ok(name) = database::datenkatalog::get_name(db, dc.id) {
            let count = database::datenkatalog::clean(db, dc.id);
            if count > 0 {
                println!(
                    "Es wurden {} Einträge für Datenkatalog '{}' entfernt!",
                    style(count).green(),
                    style(name).bold()
                );
                println!();
                return;
            }
            warn!(format!(
                "Keine Einträge für Datenkatalog '{}'!",
                style(name).bold()
            ));
        }
        println!()
    }
}
