use crate::database::Database;
use crate::{database, green_headline};
use dialoguer::Password;

pub fn change_password(db: &Database, login: &Option<String>, new_password: &Option<String>) {
    match new_password {
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
    }
}
