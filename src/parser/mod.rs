mod display;

use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

pub trait Parse {
    fn parse(er: &ElementRef) -> Self;
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Lang {
    Eu,
    Es,
    En,
    Fr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Translation {
    pub from: Lang,
    pub to: Lang,
    pub word: String,
    entry_list: Vec<Entry>,
}

impl Translation {
    pub fn new(from: Lang, to: Lang, word: String) -> Self {
        Translation {
            from,
            to,
            word,
            entry_list: vec![],
        }
    }

    pub fn parse(&mut self, doc: &Html) -> Result<(), ()> {
        let fmt_str = format!(
            "ul.hizkuntzaren_arabera.hizkuntza-{}_{}>li",
            self.from, self.to
        );
        let entry_selector = scraper::Selector::parse(&fmt_str).unwrap();

        let entry_list: Vec<Entry> = doc
            .select(&entry_selector)
            .map(|mut x| Entry::parse(&mut x))
            .collect();

        if entry_list.is_empty() {
            Err(())
        } else {
            self.entry_list = entry_list;
            Ok(())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    kind: String,
    word_list: Vec<String>,
    example_list: Vec<Example>,
}

impl Parse for Entry {
    fn parse(er: &ElementRef) -> Entry {
        let word_list = parse_word_list(&er);
        let example_list = parse_example_list(&er);

        Entry {
            kind: String::default(),
            word_list,
            example_list,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    sentence: String,
    translation: String,
}

impl Parse for Example {
    fn parse(er: &ElementRef) -> Example {
        let pattern = "</em>: ";
        let sentence_selector = Selector::parse("em").unwrap();
        let sentence: String = er
            .select(&sentence_selector)
            .map(|x| x.inner_html())
            .collect();
        let mut translation: String = er.inner_html();

        let translation_offs = translation
            .find(pattern)
            .unwrap_or(translation.len() - pattern.len());

        translation.drain(..translation_offs + pattern.len());

        Example {
            sentence,
            translation,
        }
    }
}

fn parse_word_list(er: &ElementRef) -> Vec<String> {
    let word_selector = Selector::parse("p.lehena>span.remark, a>*").unwrap();
    let word_list: Vec<String> = er.select(&word_selector).map(|x| x.inner_html()).collect();

    word_list
}

fn parse_example_list(er: &ElementRef) -> Vec<Example> {
    let example_selector = Selector::parse("div.padDefn>p.text-muted").unwrap();
    let example_list: Vec<Example> = er
        .select(&example_selector)
        .map(|mut x| Example::parse(&mut x))
        .collect();

    example_list
}
