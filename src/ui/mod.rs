use console::{style, Term};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::io::Error;

pub mod datenkatalog;
pub mod merkmalskatalog;

pub struct CustomTheme;

impl CustomTheme {
    fn default() -> ColorfulTheme {
        ColorfulTheme {
            active_item_prefix: style(">".into()).for_stderr().green(),
            ..ColorfulTheme::default()
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
