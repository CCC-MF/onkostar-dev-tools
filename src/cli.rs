use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true, arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(short = 'U', long = "user", help = "Benutzername für Datenbankzugriff")]
    pub username: String,
    #[arg(long = "password", help = "Passwort für Datenbankzugriff")]
    pub password: String,
    #[arg(
        short = 'H',
        long = "host",
        default_value = "localhost",
        help = "Datenbankhost"
    )]
    pub host: String,
    #[arg(
        short = 'P',
        long = "port",
        default_value = "3306",
        help = "Datenbankport"
    )]
    pub port: String,
    #[arg(
        short = 'D',
        long = "database",
        default_value = "onkostar",
        help = "Name der Datenbank"
    )]
    pub dbname: String,
}

#[derive(Clone, Subcommand)]
pub enum Commands {
    #[command(about = "Befehle für Merkmalskataloge")]
    Merkmalskatalog {
        #[command(subcommand)]
        command: MkCommands,
    },
    #[command(about = "Befehle für Patienten")]
    Patient {
        #[command(subcommand)]
        command: PatientCommands,
    },
    #[command(about = "Befehle für Benutzer")]
    User {
        #[command(subcommand)]
        command: UserCommands,
    },
}

#[derive(Clone, Subcommand)]
pub enum MkCommands {
    #[command(about = "Zeigt eine Liste von gefilterten Merkmalskatalogen an")]
    Ls {
        #[arg(short = 'q', long = "query", help = "Suchbegriff")]
        query: String,
    },
    #[command(about = "Zeigt eine Versionen eines Merkmalskataloges an")]
    Versions {
        #[arg(help = "ID des Merkmalskatalogs")]
        id: u128,
    },
}

#[derive(Clone, Subcommand)]
pub enum PatientCommands {
    #[command(about = "Anonymisiert Patientendaten")]
    Anonym,
}

#[derive(Clone, Subcommand)]
pub enum UserCommands {
    #[command(about = "Neues Passwort für Onkostar festlegen")]
    Password {
        #[arg(long = "login", help = "Loginname des Benutzers. Ändert alle Passwörter, wenn nicht angegeben.")]
        login: Option<String>,
        #[arg(help = "Neues Passwort")]
        new_password: String,
    }
}
