use crate::database;
use crate::database::Database;
use console::{style, Term};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::process::exit;

pub fn show_query_result(db: &Database, query: &String) {
    let mks = database::merkmalskatalog::query(db, query);
    if mks.len() > 50 {
        println!("Mehr als 50 Einträge, bitte Filter weiter einschränken");
        exit(1);
    } else if mks.is_empty() {
        println!("{}", style("Keine Einträge").yellow());
        println!();
        return
    }
    let term = Term::stdout();

    println!("\nMerkmalskatalog auswählen:");

    if let Ok(Some(selection)) = Select::with_theme(&ColorfulTheme::default())
        .items(&mks)
        .default(0)
        .interact_on_opt(&term)
    {
        let _ = term.clear_last_lines(2);
        let value = mks.get(selection).unwrap();
        println!("{}", style("Merkmalskatalog").green().bold());
        println!("ID:           {}", value.id);
        println!("Name:         {}", value.name);
        show_versions_result(db, value.id);
    }
}

pub fn show_versions_result(db: &Database, id: u128) {
    let versions = database::merkmalskatalog::versions(db, id);

    let term = Term::stdout();

    println!("\nVersion auswählen:");

    if let Ok(Some(selection)) = Select::with_theme(&ColorfulTheme::default())
        .items(&versions)
        .default(0)
        .interact_on_opt(&term)
    {
        let _ = term.clear_last_lines(2);
        let value = versions.get(selection).unwrap();

        println!("\n{}", style("Version des Merkmalskatalogs").green().bold());
        println!("ID:           {}", value.id);
        println!("Beschreibung: {}", value.description);

        println!("\n{}", style("Merkmale").green().bold());
        let result = database::merkmalskatalog::values(db, value.id);
        if result.is_empty() {
            println!("{}", style("Keine Einträge").yellow());
            println!();
            return
        }
        result.into_iter()
            .for_each(|value| {
                println!("ID:           {}", value.id);
                println!("Name:         {}", value.name);
                println!("Beschreibung: {}", value.beschreibung);
                println!()
            })
    }
}
