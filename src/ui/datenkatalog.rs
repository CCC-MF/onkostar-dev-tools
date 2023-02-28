use crate::database;
use crate::database::Database;
use crate::ui::EntitySelect;
use console::{style, Term};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::process::exit;

pub fn show_query_result(db: &Database, query: &String) {
    let dks = database::datenkatalog::query(db, query);
    if dks.len() > 50 {
        println!("Mehr als 50 Einträge, bitte Filter weiter einschränken");
        exit(1);
    } else if dks.is_empty() {
        println!("{}", style("Keine Einträge").yellow());
        println!();
        return;
    }

    let term = Term::stdout();

    println!("Datenkatalog auswählen:");

    if let Ok(Some(selection)) = EntitySelect::new().items(&dks).interact_on_opt(&term) {
        let _ = term.clear_last_lines(1);
        let value = dks.get(selection).unwrap();
        println!("{}", style("Datenkatalog").green().bold());
        println!("{}", value);
        println!("Nächste Aktion auswählen:");

        if let Ok(Some(selection)) = Select::with_theme(&ColorfulTheme::default())
            .items(&["Ende", "Formulare anzeigen", "Prozeduren löschen"])
            .default(0)
            .interact_on_opt(&term)
        {
            match selection {
                1 => {
                    show_forms(db, value.id)
                }
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

pub fn show_forms(db: &Database, id: u64) {
    let term = Term::stdout();
    let _ = term.clear_last_lines(1);

    println!(
        "{}",
        style("Formulare mit diesem Datenkatalog").green().bold()
    );

    let forms = database::datenkatalog::forms(db, id);

    if forms.is_empty() {
        println!("{}", style("Keine Einträge").yellow());
        println!();
        return;
    }

    for form in forms {
        println!("{}", form);
    }
}

pub fn show_clean_dialogue(db: &Database, id: u64) {
    println!("{}", style("Prozeduren löschen").green().bold());
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
    println!("{}", style("Keine Einträge").yellow());
    println!()
}
