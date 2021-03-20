///! transformer.rs -- text transformer for various purposes.

pub trait Transformer {
    /// perform the transform
    fn act<'a>(&self, text: &'a mut str) -> &'a str;
}

#[allow(dead_code)]
pub enum TF {
    Concat,
}

impl Transformer for TF {
    fn act<'a>(&self, _text: &'a mut str) -> &'a str {
        todo!()
    }
}

/// identify input languages.
/// If any of the input characters are utf-8 Chinese, set the source language as Chinese.
/// English otherwise.
/// - TODO perhaps whatlang-rs, or as a feature?
///   url: https://github.com/greyblake/whatlang-rs
pub fn identify_language(text: &str) -> &str {
    let mut is_chinese = false;
    for c in text.chars() {
        if 0x4e00 < c as i32 && (c as i32) < 0x9fa5 {
            is_chinese = true;
            break;
        }
    }

    if is_chinese {
        "zh"
    } else {
        "en"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_identify_chinese() {
        let text = "日月忽其不淹兮，春与秋其代序";
        let guess = identify_language(text);
        assert_eq!(guess, "zh")
    }

    #[test]
    fn can_identify_nonchinese_as_en() {
        let text = "Without delay the sun and moon sped fast, In swift succession spring and autumn passed";
        let guess = identify_language(text);
        assert_eq!(guess, "en")
    }
}
