use ansi_term::{ANSIString, Colour, Style};

use crate::server::WordData;

/// format the string to ansi_term
pub fn format_ansi_term<'a>(data: &'a WordData) -> String {
    let mut strings: Vec<String> = Vec::new();
    strings.push(Colour::Red.paint(&data.backend).to_string());
    strings.push(Style::new().bold().paint(data.query.text).to_string());
    if let Some(ps) = &data.phonetic_symbol {
        strings.push(Colour::Red.paint(ps).to_string());
    };
    strings.push(Colour::Green.paint(&data.short_desc).to_string());
    if let Some(long_desc) = &data.long_desc {
        strings.push(long_desc.clone());
    }
    strings.join("\n")
}
