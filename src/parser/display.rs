use crate::parser::{Entry, Example, Lang, Translation};
use colored::Colorize;
use std::fmt;

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

impl fmt::Display for Example {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sentence = &self.sentence.italic();
        let translation = &self.translation;
        writeln!(f, "   {sentence}: {translation}")
    }
}
