use crate::database::Database;
use crate::ui::page::Page;
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

    headline!("Formular auswählen");
    let term = Term::stdout();
    if let Ok(Some(selection)) = EntitySelect::new().items(&forms).interact_on_opt(&term) {
        let _ = term.clear_last_lines(1);
        let value = forms.get(selection).unwrap();
        show(db, value.id)
    }
}

pub fn show(db: &Database, id: u64) {
    let term = Term::stdout();
    green_headline!("Formular");

    if let Some(form) = database::form::get_by_id(db, id) {
        println!("{}", form);
        headline!("Nächste Aktion auswählen");

        let mut items = vec![
            "Ende",
            "Verwendete Datenkataloge anzeigen",
            "Alle Prozeduren löschen",
        ];

        if !database::form::subforms(db, id).is_empty() {
            items.push("Unterformulare anzeigen")
        }

        if let Ok(Some(selection)) = Select::with_theme(&CustomTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(&term)
        {
            let _ = term.clear_last_lines(1);
            match selection {
                1 => show_data_catalogues(db, id),
                2 => show_clean_dialogue(db, id),
                3 => show_subforms(db, id),
                _ => {}
            }
            return;
        }
    }

    warn!("Nicht gefunden");
    println!();
}

pub fn show_subforms(db: &Database, id: u64) {
    let forms = database::form::subforms(db, id);
    Page::with(&forms, 4).show("Unterformulare dieses Formulars");
}

pub fn show_data_catalogues(db: &Database, id: u64) {
    let dks = database::form::data_catalogues(db, id);
    Page::with(&dks, 4).show("Datenkataloge dieses Formulars");
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
