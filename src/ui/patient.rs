use crate::database::disease::delete;
use crate::database::Database;
use crate::ui::page::Page;
use crate::ui::{CustomTheme, EntitySelect};
use crate::{database, green_headline, headline, warn};
use console::Term;
use dialoguer::Select;
use indicatif::{MultiProgress, ProgressBar};
use std::process::exit;

pub fn show_query_result(db: &Database, query: &String) {
    let ps = database::patient::query(db, query);
    if ps.len() > 50 {
        warn!("Mehr als 50 Einträge, bitte Filter weiter einschränken");
        exit(1);
    } else if ps.is_empty() {
        warn!("Keine Einträge");
        println!();
        return;
    }

    let term = Term::stdout();

    headline!("Patient auswählen");

    if let Ok(Some(selection)) = EntitySelect::new().items(&ps).interact_on_opt(&term) {
        let _ = term.clear_last_lines(1);
        let value = ps.get(selection).unwrap();
        show(db, value.id)
    }
}

pub fn show(db: &Database, id: u64) {
    let term = Term::stdout();
    green_headline!("Patient");
    if let Some(p) = database::patient::get_by_id(db, id) {
        println!("{}", p);
        headline!("Nächste Aktion auswählen");

        if let Ok(Some(selection)) = Select::with_theme(&CustomTheme::default())
            .items(&[
                "Ende",
                "Prozeduren anzeigen",
                "Erkrankungen und Prozeduren löschen",
            ])
            .default(0)
            .interact_on_opt(&term)
        {
            match selection {
                1 => show_procedures(db, p.id),
                2 => delete_disease_and_procedures(db, p.id),
                _ => {
                    let _ = term.clear_last_lines(1);
                }
            }
        }
        return;
    }

    warn!("Nicht gefunden");
    println!();
}

pub fn show_procedures(db: &Database, id: u64) {
    let term = Term::stdout();
    let _ = term.clear_last_lines(1);

    let procedures = database::patient::procedures(db, id);
    Page::with(&procedures, 4).show("Prozeduren dieses Patienten");
}

pub fn delete_disease_and_procedures(db: &Database, id: u64) {
    let term = Term::stdout();
    let _ = term.clear_last_lines(1);

    let procedures = database::patient::procedures(db, id);
    let diseases = database::disease::find_by_patient_id(db, id);

    let multiprogress = MultiProgress::new();

    headline!("Lösche Prozeduren");
    if procedures.is_empty() {
        warn!("Keine Einträge");
    } else {
        let procedure_bar = ProgressBar::new(procedures.len() as u64);
        procedures.iter().for_each(|procedure| {
            let removed = database::prozedur::delete(db, procedure.procedure_id);
            if removed > 0 {
                procedure_bar.inc(1);
            };
        });
        procedure_bar.finish();
        multiprogress.add(procedure_bar);
    }

    headline!("Lösche Erkrankungen");
    if diseases.is_empty() {
        warn!("Keine Einträge");
    } else {
        let disease_bar = ProgressBar::new(diseases.len() as u64);
        diseases.iter().for_each(|disease| {
            if delete(db, disease.id) {
                disease_bar.inc(1)
            }
        });
        disease_bar.finish();
        multiprogress.add(disease_bar);
    }

    println!();
}

pub fn anonymize(db: &Database) {
    let count = database::patient::count_non_anonym(db);

    println!("Anonymisiere {count} Patienten ...");

    let bar = ProgressBar::new(count);
    for _ in 0..count {
        if let Ok(id) = database::patient::next_not_anon(db) {
            database::patient::anonymize(db, id);
            bar.inc(1);
        }
    }
    bar.finish();
    println!("... fertig!")
}
