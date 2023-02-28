#![feature(iter_intersperse)]
mod parser;
mod args;

use args::*;
use colored::Colorize;

fn main() {
    let mut translations = parse_args();
    for translation in translations.iter_mut() {
        let response = reqwest::blocking::get(
            format!("https://hiztegiak.elhuyar.eus/{}_{}/{}",
                    translation.from,
                    translation.to,
                    translation.word,
            ),
        )
            .unwrap()
            .text()
            .unwrap();
        
        let document = scraper::Html::parse_document(&response);
        match translation.parse(&document) {
            Ok(_) => print!("{}", translation),
            Err(_) => eprintln!("{}",
                                format!("No entries for `{}' found.",
                                        translation.word).red()),
        }
        
    }
}
