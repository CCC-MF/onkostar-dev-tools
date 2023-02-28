use clap::Parser;
use dialoguer::Password;
use indicatif::ProgressBar;
use std::process::exit;

use crate::cli::{
    Cli, Commands, DkCommands, FormCommands, MkCommands, PatientCommands, UserCommands,
};
use crate::database::Database;
use crate::ui::*;

mod cli;
mod database;
mod ui;

fn main() {
    let cli = Cli::parse();

    let (db_username, db_password) = db_login(cli.username, cli.password);

    let db = &Database::new(db_username, db_password, cli.host, cli.port, cli.dbname);
    let db = match db {
        Ok(db) => db,
        Err(error_string) => {
            warn!(error_string);
            exit(1)
        }
    };

    match &cli.command {
        Commands::Datenkatalog { command } | Commands::DK { command } => match command {
            DkCommands::Ls { query } => {
                datenkatalog::show_query_result(db, query);
            }
            DkCommands::Forms { id } => {
                datenkatalog::show_forms(db, *id);
            }
            DkCommands::Clean { id } => datenkatalog::show_clean_dialogue(db, *id),
        },
        Commands::Form { command } => match command {
            FormCommands::Ls { query } => {
                form::show_query_result(db, query);
            }
        },
        Commands::Merkmalskatalog { command } | Commands::MK { command } => match command {
            MkCommands::Ls { query } => {
                merkmalskatalog::show_query_result(db, query);
            }
            MkCommands::Versions { id } => {
                merkmalskatalog::show_versions_result(db, *id);
            }
        },
        Commands::Patient { command } => match command {
            PatientCommands::Anonym => {
                let count = database::patient::count_non_anonym(db);

                println!("Anonymisiere {count} Patienten ...");

                let bar = ProgressBar::new(count);
                for _ in 0..count {
                    if let Ok(id) = database::patient::next(db) {
                        database::patient::anonymize(db, id);
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
            } => match new_password {
                Some(password) => database::user::update_password(db, login, password),
                None => {
                    green_headline!(match login {
                        Some(login) => format!("Neues Passwort für Benutzer '{}' setzen", login),
                        None => "Neues Passwort für alle Benutzer setzen".to_string(),
                    });
                    if let Ok(password) = Password::new()
                        .with_prompt("Neues Passwort")
                        .with_confirmation("Wiederholung", "Passwörter nicht identisch")
                        .interact()
                    {
                        database::user::update_password(db, login, &password)
                    }
                }
            },
        },
    }
}
