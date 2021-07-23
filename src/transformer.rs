///! transformer.rs -- text transformer for various purposes.

pub trait Transformer {
    /// perform the transform
    fn act(&self, text: &str) -> String;
}

/// replace linebreaks with witespace
#[derive(Debug, Clone, Copy)]
pub struct Concat {
    /// Threshold number of consecutive linebreaks for which marking a real linebreak.
    /// Default value is 2.
    pub n: usize,
}

impl Concat {
    pub fn new(n: usize) -> Self {
        Self { n }
    }
}

impl Default for Concat {
    fn default() -> Self {
        Concat::new(2)
    }
}

// TODO treat \n\n as \n
impl Transformer for Concat {
    fn act(&self, text: &str) -> String {
        let mut c = 0;
        let mut result = String::new();
        for str in text.split(|c| c == '\n') {
            if str.is_empty() {
                c = c + 1;
            } else {
                if c + 1 >= self.n {
                    result.pop();
                    result.push('\n');
                }
                c = 0;
                result.push_str(str);
                result.push(' ')
            }
        }
        // the last char must be empty, unless `text` is empty already.
        result.pop();
        result
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

    #[test]
    fn can_concat_lines() {
        let text = "欲买桂花同载酒\n终不似 少年游";
        let transfromed = Concat::default().act(&text);
        assert_eq!(transfromed, "欲买桂花同载酒 终不似 少年游");
    }

    #[test]
    fn can_identify_real_linebreaks() {
        let text = "欲买桂花同载酒\n\n\n终不似\n少年游";
        let transfromed = Concat::default().act(&text);
        assert_eq!(transfromed, "欲买桂花同载酒\n终不似 少年游");
    }
}
