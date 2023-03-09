use crate::parser::{Lang, Translation};
use clap::{arg, command, ArgAction, Parser};
// use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Set the language with which to search for the following terms.
    /// Default is `eu'
    #[arg(short, long, value_parser = ["eu", "es", "en", "fr"])]
    from: Option<String>,

    /// Set JSON as the output format
    #[arg(short, long)]
    pub json: bool,

    /// Set the output file
    // #[arg(short, long, value_name = "FILE")]
    // pub output: Option<PathBuf>,

    /// Search for the following terms
    #[arg(required = true, action = ArgAction::Append)]
    term: Vec<String>,
}

pub fn parse_args() -> (Config, Vec<Translation>) {
    let cfg = Config::parse();

    let mut from = Lang::Eu;
    let mut to = Lang::Es;

    if let Some(lang) = cfg.from.as_deref() {
        match lang {
            "eu" => {
                from = Lang::Eu;
                to = Lang::Es;
            }
            "es" => {
                from = Lang::Es;
                to = Lang::Eu;
            }
            "en" => {
                from = Lang::En;
                to = Lang::Eu;
            }
            "fr" => {
                from = Lang::Fr;
                to = Lang::Eu;
            }
            _ => unreachable!(),
        }
    }

    let translations: Vec<Translation> = cfg
        .term
        .iter()
        .map(|t| Translation::new(from, to, t.to_owned()))
        .collect();

    (cfg, translations)
}
