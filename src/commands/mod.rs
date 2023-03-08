use crate::cli::{Commands, DkCommands, FormCommands, MkCommands, PatientCommands, UserCommands};
use crate::database::Database;
use crate::ui::user::change_password;
use crate::ui::*;

pub fn handle_command(db: &Database, command: &Commands) {
    match command {
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
            PatientCommands::Anonym => patient::anonymize(db),
            PatientCommands::Ls { query } => {
                patient::show_query_result(db, query);
            }
        },
        Commands::User { command } => match command {
            UserCommands::Password {
                login,
                new_password,
            } => change_password(db, login, new_password),
        },
        _ => { /* Do not handle command or command handled before */ }
    }
}
