use crate::parser::{Translation, Lang};
use clap::{
    Parser, arg, command, ArgAction};

// enum ArgError {
//     UnknownArg(String),
//     WrongPosition(String),
// }

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Search for the next terms in this language
    #[arg(short, long, value_parser = ["eu", "es", "en", "fr"])]
    from: Option<String>,

    #[arg(required = true, action = ArgAction::Append)]
    term: Vec<String>,
}

pub fn parse_args() -> Vec<Translation> {
    let cli = Cli::parse();

    let mut from = Lang::Eu;
    let mut to = Lang::Es;

    if let Some(lang) = cli.from.as_deref() {
        match lang {
            "eu" => {
                from = Lang::Eu;
                to = Lang::Es;
            },
            "es" => {
                from = Lang::Es;
                to = Lang::Eu;
            },
            "en" => {
                from = Lang::En;
                to = Lang::Eu;
            },
            "fr" => {
                from = Lang::Fr;
                to = Lang::Eu;
            },
            _ => unreachable!(),
        }
    }

    let translations: Vec<Translation> = cli.term
        .iter()
        .map(|t| Translation::new(from, to, t.to_string()))
        .collect();

    translations
}


// fn parse_arg() -> Result<Vec<Translation>, ArgError> {
//     unimplemented!();
//     let args = env::args().skip(1);

//     let from = Lang::Eu;
//     let to = Lang::Es;

//     for arg in args {
//         match arg.as_str() {
//             "-f" |
//             "--from" => {},
//             _ => {},
//         }

//     }

//     unimplemented!();
// }

// pub fn parse_args() -> Result<Vec<Translation>, ()> {
//     let args = env::args();
//     if args.len() < 2 {
//         Err(())
//     } else {
//         Ok(args.skip(1)
//            .map(|s| Translation::new(Lang::Eu, Lang::Es, s))
//            .collect())
//     }
// }

// pub fn show_help() {
//     let name = env::args().next().unwrap();
//     println!("Usage: {name} [OPTION]... [TERM]...");
//     println!("Fetch translation from Elhuyar Hiztegia\n");
//     println!("\tflag1");
//     println!("\tflag2");
// }
