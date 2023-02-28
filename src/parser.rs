use colored::Colorize;
use scraper::{ElementRef, Html, Selector};
use std::fmt;

pub trait Parse {
    fn parse(er: &ElementRef) -> Self;
}

#[derive(Clone, Copy)]
pub enum Lang {
    Eu,
    Es,
    En,
    Fr,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Lang::Eu => "eu",
                Lang::Es => "es",
                Lang::En => "en",
                Lang::Fr => "fr",
            }
        )
    }
}

impl fmt::Debug for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
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

impl fmt::Display for Translation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}\n{} > {}",
            self.word.bold(),
            format!("{}", self.from).blue(),
            format!("{}", self.to).blue(),
        )?;

        for (mut num, entry) in self.entry_list.iter().enumerate() {
            num += 1;
            write!(f, "{num}. {entry}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    kind: String,
    word_list: Vec<String>,
    example_list: Vec<Example>,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)?;
        self.word_list
            .iter()
            .map(|w| w.blue())
            .intersperse(", ".clear())
            .for_each(|word| write!(f, "{word}").unwrap());

        writeln!(f)?;
        for example in self.example_list.iter() {
            write!(f, "{example}")?;
        }
        Ok(())
    }
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

#[derive(Debug, Clone)]
pub struct Example {
    sentence: String,
    translation: String,
}

impl fmt::Display for Example {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sentence = &self.sentence.italic();
        let translation = &self.translation;
        writeln!(f, "   {sentence}: {translation}")
    }
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
