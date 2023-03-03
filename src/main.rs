use std::io;
use clap::{CommandFactory, Parser};
use dialoguer::Password;
use indicatif::ProgressBar;
use std::process::exit;
use clap_complete::generate;

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

    if let Commands::Completions { shell } = cli.commands {
        let exe_name = std::env::current_exe()
            .ok()
            .and_then(|exe| exe.file_name().map(|s| s.to_os_string()))
            .and_then(|s| s.into_string().ok());

        let command_name = if let Some(command_name) = exe_name {
            command_name
        } else {
            "onkostar_dev_tools".to_string()
        };
        generate(shell, &mut Cli::command(), command_name, &mut io::stdout());
        return
    }

    let (db_username, db_password) = db_login(cli.username, cli.password);

    let db = &Database::new(db_username, db_password, cli.host, cli.port, cli.dbname);
    let db = match db {
        Ok(db) => db,
        Err(error_string) => {
            warn!(error_string);
            exit(1)
        }
    };

    match &cli.commands {
        Commands::Completions { .. } => { /* Command handled before */ },
        Commands::Datenkatalog { command } | Commands::DK { command } => match command {
            DkCommands::Ls { query } => {
                datenkatalog::show_query_result(db, query);
            }
            DkCommands::Show { id } => {
                datenkatalog::show(db, *id);
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
            FormCommands::Show { id } => {
                form::show(db, *id);
            }
            FormCommands::UF { id } => form::show_subforms(db, *id),
            FormCommands::DK { id } => form::show_data_catalogues(db, *id),
            FormCommands::Clean { id } => form::show_clean_dialogue(db, *id),
        },
        Commands::Merkmalskatalog { command } | Commands::MK { command } => match command {
            MkCommands::Ls { query } => {
                merkmalskatalog::show_query_result(db, query);
            }
            MkCommands::Show { id } => {
                merkmalskatalog::show(db, *id);
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
