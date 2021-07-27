mod ansiterm;
mod md;

pub use ansiterm::format_ansi_term;
pub use md::format_markdown;

use serde::{Deserialize, Serialize};

use crate::RespData;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Format {
    AnsiTerm,
    Markdown,
}

impl Format {
    pub fn format(&self, resp: &RespData) -> String {
        match self {
            Format::AnsiTerm => format_ansi_term(resp),
            Format::Markdown => format_markdown(resp),
        }
    }
}

impl From<&str> for Format {
    fn from(s: &str) -> Self {
        match s {
            "md" => Format::Markdown,
            "ansi" => Format::AnsiTerm,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_str() {
        let md = "md";
        let ansi = "ansi";
        assert_eq!(Format::Markdown, md.into());
        assert_eq!(Format::AnsiTerm, ansi.into())
    }
}
