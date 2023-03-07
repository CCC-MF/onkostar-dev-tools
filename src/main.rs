extern crate core;

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use std::io;
use std::process::exit;

use crate::cli::{Cli, Commands};
use crate::commands::handle_command;
use crate::database::Database;
use crate::ui::*;

mod cli;
mod commands;
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
        return;
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

    handle_command(db, &cli.commands)
}
