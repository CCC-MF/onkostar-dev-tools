use clap::Parser;
use console::style;
use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use indicatif::ProgressBar;
use std::process::exit;

use crate::cli::{Cli, Commands, MkCommands, PatientCommands, UserCommands};
use crate::database::markmalskatalog::Merkmalskatalog;
use crate::database::patient::Patient;
use crate::database::user::User;
use crate::database::Database;

mod cli;
mod database;

fn show_query_result(db: &Database, query: &String) {
    let mks = Merkmalskatalog::query(db, query);
    if mks.len() > 25 {
        println!("Mehr als 25 Einträge, bitte Filter weiter einschränken");
        exit(1);
    }
    if let Ok(Some(selection)) = Select::with_theme(&ColorfulTheme::default())
        .items(&mks)
        .default(0)
        .interact_on_opt(&Term::stderr())
    {
        let value = mks.get(selection).unwrap();
        println!(
            "{}: \t{}\n\nSelect Version:\n",
            style(value.id).bold(),
            style(value.name.clone()).bold()
        );
        show_versions_result(db, value.id);
    }
}

fn show_versions_result(db: &Database, id: u128) {
    let versions = Merkmalskatalog::versions(db, id);
    if let Ok(Some(selection)) = Select::with_theme(&ColorfulTheme::default())
        .items(&versions)
        .default(0)
        .interact_on_opt(&Term::stderr())
    {
        let value = versions.get(selection).unwrap();
        println!("{}: \t{}\n", style(value.id).bold(), value.description);
        Merkmalskatalog::values(db, value.id)
            .into_iter()
            .for_each(|value| {
                println!(
                    "{}: \t{}\n\t{}",
                    style(value.id).bold(),
                    style(value.name).bold(),
                    value.beschreibung
                );
            })
    }
}

fn main() {
    let cli = Cli::parse();

    let db = &Database::new(cli.username, cli.password, cli.host, cli.port, cli.dbname);

    match &cli.command {
        Commands::Merkmalskatalog { command } => match command {
            MkCommands::Ls { query } => {
                println!("Select Entry:\n");
                show_query_result(db, query);
            }
            MkCommands::Versions { id } => {
                show_versions_result(db, *id);
            }
        },
        Commands::Patient { command } => match command {
            PatientCommands::Anonym => {
                let count = Patient::count_non_anonym(db);

                println!("Anonymisiere {count} Patienten ...");

                let bar = ProgressBar::new(count);
                for _ in 0..count {
                    if let Ok(id) = Patient::next(db) {
                        Patient::anonymize(db, id);
                        bar.inc(1);
                    }
                }
                bar.finish();
                println!("... fertig!")
            }
        },
        Commands::User { command } => match command {
            UserCommands::Password {
                login,
                new_password,
            } => {
                User::update_password(db, login, new_password);
            }
        },
    }
}
