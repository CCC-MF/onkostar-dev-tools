use clap::Parser;
use console::style;
use indicatif::ProgressBar;

use crate::cli::{Cli, Commands, DkCommands, MkCommands, PatientCommands, UserCommands};
use crate::database::patient::Patient;
use crate::database::user::User;
use crate::database::Database;
use crate::ui::*;

mod cli;
mod database;
mod ui;

fn main() {
    let cli = Cli::parse();

    let db = &Database::new(cli.username, cli.password, cli.host, cli.port, cli.dbname);

    match &cli.command {
        Commands::Datenkatalog { command } => match command {
            DkCommands::Ls { query } => {
                datenkatalog::show_query_result(db, query);
            }
            DkCommands::Clean { id } => {
                if let Ok(name) = database::datenkatalog::Datenkatalog::get_name(db, *id) {
                    let count = database::datenkatalog::Datenkatalog::clean(db, *id);
                    if count > 0 {
                        println!(
                            "Es wurden {} Einträge für '{}' entfernt!",
                            style(count).green(),
                            style(name).bold()
                        );
                        return
                    }
                }
                println!("Es wurden keine Einträge entfernt!")
            }
        },
        Commands::Merkmalskatalog { command } => match command {
            MkCommands::Ls { query } => {
                merkmalskatalog::show_query_result(db, query);
            }
            MkCommands::Versions { id } => {
                merkmalskatalog::show_versions_result(db, *id);
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
