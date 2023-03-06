use crate::{blue_headline, green_headline, warn};
use console::Term;
use dialoguer::Confirm;
use std::fmt::Display;
use std::slice::Chunks;

pub struct Page<'a, T: Display> {
    current_page: usize,
    chunks: Chunks<'a, T>,
}

impl<'a, T> Page<'a, T>
where
    T: Display,
{
    pub fn with(items: &[T], item_size: u16) -> Page<T> {
        let (rows, _) = &Term::stdout().size();
        Page {
            current_page: 0,
            chunks: items.chunks((rows / item_size - 4) as usize),
        }
    }

    pub fn show(mut self, headline: &str) {
        let len = self.chunks.len();

        if len == 0 {
            warn!("Keine Einträge");
            println!();
            return;
        }

        for chunk in self.chunks {
            self.current_page += 1;

            green_headline!(headline);
            blue_headline!(format!("Seite {} von {}", self.current_page, len));
            println!();
            chunk.iter().for_each(|value| {
                println!("{}", value);
            });

            if self.current_page == len {
                return;
            }

            blue_headline!(format!("Ende Seite {}", self.current_page));

            if let Ok(false) = Confirm::new()
                .with_prompt("Nächste Seite anzeigen?")
                .default(true)
                .interact()
            {
                return;
            }

            let _ = &Term::stdout().clear_last_lines(2);
        }
    }
}
