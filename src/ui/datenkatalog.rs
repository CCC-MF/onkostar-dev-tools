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

    println!("\nSelect Entry:");

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
    }
}
