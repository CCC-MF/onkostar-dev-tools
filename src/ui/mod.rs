use console::Term;
use dialoguer::theme::Theme;
use dialoguer::{Input, Password, Select};
use std::io::Error;

pub mod datenkatalog;
pub mod form;
pub mod merkmalskatalog;
pub mod page;
pub mod patient;
pub mod user;

pub struct CustomTheme;

impl CustomTheme {
    fn default() -> impl Theme {
        #[cfg(windows)]
        {
            use dialoguer::theme::SimpleTheme;
            SimpleTheme
        }
        #[cfg(unix)]
        {
            use dialoguer::theme::ColorfulTheme;
            ColorfulTheme::default()
        }
    }
}

pub trait SelectDisplay {
    fn to_string(&self) -> String;
}

pub struct EntitySelect {
    items: Vec<String>,
}

impl EntitySelect {
    fn new() -> EntitySelect {
        EntitySelect { items: vec![] }
    }

    pub fn items(&mut self, items: &[impl SelectDisplay]) -> &mut Self {
        for item in items {
            self.items.push(item.to_string());
        }
        self
    }

    fn interact_on_opt(&self, term: &Term) -> Result<Option<usize>, Error> {
        let items = &self.items;
        Select::with_theme(&CustomTheme::default())
            .items(items)
            .default(0)
            .interact_on_opt(term)
    }
}

pub fn db_login(username: Option<String>, password: Option<String>) -> (String, String) {
    let term = Term::stdout();

    let db_username = match username {
        Some(username) => username,
        None => {
            let value = match Input::<String>::new()
                .with_prompt("DB Benutzername")
                .interact_text()
            {
                Ok(username) => username,
                _ => String::new(),
            };
            let _ = term.clear_last_lines(1);
            value
        }
    };

    let db_password = match password {
        Some(password) => password,
        None => {
            let value = match Password::new().with_prompt("DB Passwort").interact() {
                Ok(password) => password,
                _ => String::new(),
            };
            let _ = term.clear_last_lines(1);
            value
        }
    };

    (db_username, db_password)
}

#[macro_export]
macro_rules! success {
    ($str:expr) => {{
        use console::style;
        println!("{}", style($str).green())
    }};
}

#[macro_export]
macro_rules! warn {
    ($str:expr) => {{
        use console::style;
        println!("{}", style($str).yellow())
    }};
}

#[macro_export]
macro_rules! blue_headline {
    ($str:expr) => {{
        use console::style;
        println!("{}", style($str).blue().bold())
    }};
}

#[macro_export]
macro_rules! green_headline {
    ($str:expr) => {{
        use console::style;
        println!("{}", style($str).green().bold())
    }};
}

#[macro_export]
macro_rules! headline {
    ($str:expr) => {{
        use console::style;
        println!("{}", style($str).bold())
    }};
}
