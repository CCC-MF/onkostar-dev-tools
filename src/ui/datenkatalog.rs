use crate::database;
use crate::database::Database;
use console::{style, Term};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::process::exit;

pub fn show_query_result(db: &Database, query: &String) {
    let dks = database::datenkatalog::query(db, query);
    if dks.len() > 50 {
        println!("Mehr als 50 Einträge, bitte Filter weiter einschränken");
        exit(1);
    }
    let term = Term::stdout();

    println!("\nDatenkatalog auswählen:");

    if let Ok(Some(selection)) = Select::with_theme(&ColorfulTheme::default())
        .items(&dks)
        .default(0)
        .interact_on_opt(&term)
    {
        let _ = term.clear_last_lines(2);
        let value = dks.get(selection).unwrap();
        println!("{}", style("Datenkatalog").green().bold());
        println!("ID:           {}", value.id);
        println!("Name:         {}", value.name);
        println!("Beschreibung: {}", value.description);
        println!();
        println!("Nächste Aktion auswählen:");

        if let Ok(Some(selection)) = Select::with_theme(&ColorfulTheme::default())
            .items(&["Ende", "Formulare anzeigen", "Prozeduren löschen"])
            .default(0)
            .interact_on_opt(&term)
        {
            match selection {
                1 => {
                    println!();
                    show_forms(db, value.id)
                },
                2 => {
                    let _ = term.clear_last_lines(1);
                    show_clean_dialogue(db, value.id)
                },
                _ => {
                    let _ = term.clear_last_lines(1);
                },
            }
        }
    }
}

pub fn show_forms(db: &Database, id: u64) {
    let term = Term::stdout();
    let _ = term.clear_last_lines(2);

    println!("{}", style("Formulare mit diesem Datenkatalog").green().bold());

    for form in database::datenkatalog::forms(db, id) {
        println!("ID:           {}", form.id);
        println!("Name:         {}", form.name);
        println!("Beschreibung: {}", form.description);
        println!()
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
            return
        }
    }
    println!("Es wurden keine Einträge entfernt!");
    println!()
}
