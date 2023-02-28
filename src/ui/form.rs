use crate::database::Database;
use crate::ui::{CustomTheme, EntitySelect};
use crate::{database, green_headline, headline, warn};
use console::Term;
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
            .items(&["Ende", "Verwendete Datenkataloge anzeigen"])
            .default(0)
            .interact_on_opt(&term)
        {
            match selection {
                1 => show_data_catalogues(db, value.id),
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
