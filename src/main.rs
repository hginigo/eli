#![feature(iter_intersperse)]
mod args;
mod parser;

use args::*;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let mut translations = parse_args();
    let bar = ProgressBar::new(translations.len() as u64);
    bar.set_style(ProgressStyle::with_template("[{pos:.cyan}/{len:.cyan}] {msg}").unwrap());

    for translation in translations.iter_mut() {
        bar.inc(1);
        bar.set_message(format!(
            "{} `{}'",
            "fetching".yellow(),
            translation.word.white().bold()
        ));

        let response = reqwest::blocking::get(format!(
            "https://hiztegiak.elhuyar.eus/{}_{}/{}",
            translation.from, translation.to, translation.word,
        ))
        .unwrap()
        .text()
        .unwrap();

        let document = scraper::Html::parse_document(&response);
        // Set the cursor properly to hide progress bar
        eprint!("\x1b[1G\x1b[0K");
        match translation.parse(&document) {
            Ok(_) => print!("{}", translation),
            Err(_) => eprintln!(
                "{}",
                format!("No entries for `{}' found.", translation.word).red()
            ),
        }
    }
    bar.finish_and_clear();
}
