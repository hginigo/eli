#![feature(iter_intersperse)]
mod parser;
mod args;

use args::*;

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
        translation.parse(&document);
        
        print!("{}", translation);
    }
}
