use ansi_term::{Colour, Style};

/// format the string to ansi_term
pub fn format_ansi_term(data: &crate::types::RespData) -> String {
    let mut strings: Vec<String> = Vec::new();
    strings.push(Colour::Red.paint(&data.backend).to_string());
    strings.push(Style::new().bold().paint(&data.query.text).to_string());
    let from_to = format!("{} to {}", data.query.lang_from, data.query.lang_to);
    strings.push(from_to);

    if let Some(pss) = &data.phonetic_symbol {
        let mut ps_string = String::new();
        for (lang, ps) in pss {
            ps_string.push_str(&format!(
                "{}: {}",
                Style::new().italic().paint(lang).to_string(),
                Colour::Blue.paint(ps).to_string()
            ));
            ps_string.push('\t');
        }
        strings.push(ps_string);
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
