mod ansiterm;
mod md;

pub use ansiterm::format_ansi_term;
pub use md::format_markdown;

use super::RespData;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Formatter {
    AnsiTerm,
    Markdown
}

impl Formatter {
    pub fn format(&self, resp: &RespData) -> String {
        match self {
            Formatter::AnsiTerm => {
                format_ansi_term(resp)
            }
            Formatter::Markdown => {
                format_markdown(resp)
            }
        }
    }
}
