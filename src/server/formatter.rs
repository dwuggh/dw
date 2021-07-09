mod ANSITermFormatter;
mod MDFormatter;

pub use ANSITermFormatter::format_ansi_term;
pub use MDFormatter::format_markdown;

use super::RespData;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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
