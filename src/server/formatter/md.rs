use crate::server::RespData;

pub fn format_markdown(resp: &RespData) -> String {
    let mut result = String::new();
    let backend = format!("* {}\n", resp.backend);
    let split1 = "---\n";
    let query = format!("{}\n", resp.query.text);
    let from_to = format!("/{}/ to /{}/", resp.query.lang_from, resp.query.lang_to);
    result.push_str(&backend);
    result.push_str(split1);
    result.push_str(&query);
    result.push_str(&from_to);
    if let Some(pss) = &resp.phonetic_symbol {
        let mut ps_string = String::new();
        for (lang, ps) in pss {
            ps_string.push_str(&format!("{}: /{}/", lang, ps));
            ps_string.push('\t');
        }
        result.push_str(&ps_string);
    };
    if resp.query.is_short_text {
        result.push_str(&resp.basic_desc);
    } else {
        // do not use special colour for long sentences' translation
        result.push_str(&resp.basic_desc);
    }
    if let Some(long_desc) = &resp.detail_desc {
        result.push_str(long_desc);
    }
    result
}
