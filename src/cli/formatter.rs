use ansi_term::{Colour, Style};

use crate::server::runner::Handler;
use crate::server::RespData;

pub struct AnsiTermHandler;

unsafe impl Send for AnsiTermHandler {}
unsafe impl Sync for AnsiTermHandler {}

impl Handler for AnsiTermHandler {
    type Result = ();

    fn handle(&self, resp: RespData) -> Self::Result {
        let output = format_ansi_term(&resp);
        println!("{}\n", output);
    }
}

/// format the string to ansi_term
fn format_ansi_term(data: &RespData) -> String {
    let mut strings: Vec<String> = Vec::new();
    strings.push(Colour::Red.paint(&data.backend).to_string());
    strings.push(Style::new().bold().paint(&data.query.text).to_string());
    let from_to = format!("{} to {}", data.query.lang_from, data.query.lang_to);
    strings.push(from_to);

    if let Some(ps) = &data.phonetic_symbol {
        strings.push(Colour::Red.paint(ps).to_string());
    };
    // if query words & phrases, use green colour
    if data.query.is_short_text {
        strings.push(Colour::Green.paint(&data.basic_desc).to_string());
    } else {
        // do not use special colour for long sentences' translation
        strings.push(data.basic_desc.clone());
    }
    if let Some(long_desc) = &data.detail_desc {
        strings.push(long_desc.clone());
    }
    strings.join("\n")
}
