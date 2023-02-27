use std::io::Write;
use clap::Parser;
use console::Term;
use indicatif::ProgressBar;

use crate::cli::{Cli, Commands, DkCommands, MkCommands, PatientCommands, UserCommands};
use crate::database::Database;
use crate::ui::*;

mod cli;
mod database;
mod ui;

fn main() {
    let cli = Cli::parse();
    let mut term = &Term::stdout();

    let db_username = match cli.username {
        Some(username) => username,
        None => {
            let _ = term.write(b"Benutzername:  ");
            match term.read_line() {
                Ok(username) => username,
                _ => String::new()
            }
        }
    };

    let db_password = match cli.password {
        Some(password) => password,
        None => {
            let _ = term.write(b"Passwort:      ");
            match term.read_secure_line() {
                Ok(password) => password,
                _ => String::new()
            }
        }
    };

    let db = &Database::new(db_username, db_password, cli.host, cli.port, cli.dbname);

    match &cli.command {
        Commands::Datenkatalog { command } | Commands::DK {command} => match command {
            DkCommands::Ls { query } => {
                datenkatalog::show_query_result(db, query);
            }
            DkCommands::Forms { id } => {
                datenkatalog::show_forms(db, *id);
            }
            DkCommands::Clean { id } => {
                datenkatalog::show_clean_dialogue(db, *id)
            }
        },
        Commands::Merkmalskatalog { command } | Commands::MK { command }=> match command {
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
            } => {
                database::user::update_password(db, login, new_password);
            }
        },
    }
}
