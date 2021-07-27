use crate::RespData;

pub fn format_markdown(resp: &RespData) -> String {
    let mut result = String::new();
    let backend = format!("# {}\n", resp.backend);
    let split1 = "---\n";
    let query = format!("## {}\n", resp.query.text);
    let from_to = format!(
        "- language: /{}/ to /{}/",
        resp.query.lang_from, resp.query.lang_to
    );
    result.push_str(&backend);
    result.push_str(split1);
    result.push_str(&query);
    result.push_str(&from_to);
    result.push_str("\n- phonetic symbol:\n");
    if let Some(pss) = &resp.phonetic_symbol {
        let mut ps_string = String::new();
        for (lang, ps) in pss {
            ps_string.push_str(&format!("  {}: {}\n", lang, ps));
            // ps_string.push('\t');
        }
        result.push_str(&ps_string);
    };
    result.push_str("\n## short desc\n");
    if resp.query.is_short_text {
        result.push_str(&resp.basic_desc);
    } else {
        result.push_str(&resp.basic_desc);
    }
    result.push_str("\n\n## long desc\n");
    if let Some(long_desc) = &resp.detail_desc {
        result.push_str(long_desc);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::Query;
    // TODO consider using mockall

    use super::*;

    fn mock_query(text: &str) -> Query {
        let query = Query::new(text, "en", "zh", false);
        query
    }

    fn mock_respdata() -> RespData {
        let query = mock_query("humuhumunukunukuapua'a");
        let phonetic_symbol: Vec<(String, String)> = vec![
            ("s1".to_string(), "s2".to_string()),
            ("s3".to_string(), "s4".to_string()),
        ];
        RespData {
            backend: "mock".to_string(),
            query,
            basic_desc: "夏威夷鳞鲀".to_string(),
            phonetic_symbol: Some(phonetic_symbol),
            detail_desc: None,
            audio: None,
        }
    }

    #[test]
    fn can_format_to_md_correctly() {
        let resp = mock_respdata();
        let s = format_markdown(&resp);
        println!("{}", s);
    }
}
