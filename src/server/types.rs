/// TODO audio type
#[derive(Debug)]
pub struct Audio;

#[derive(Debug)]
pub struct WordData {
    /// backend identifier
    pub backend: String,
    pub query: std::sync::Arc<Query>,
    pub short_desc: String,
    pub phonetic_symbol: Option<String>,
    // TODO find better type for long_desc, should not be string
    /// long description for the words. Could involve media
    pub long_desc: Option<String>,
    // TODO just placeholder for now
    pub audio: Option<Audio>,
}

/// query for text
#[derive(Debug)]
pub struct Query {
    /// the query text
    pub text: String,
    /// which language does this literature originally belongs to
    pub lang_from: String,
    /// which language description belongs to
    pub lang_to: String,
    pub audio: bool,
}

impl Query {
    pub fn new<S: Into<String>>(text: S, lang_from: &str, lang_to: &str, audio: bool) -> Query {
        Query {
            text: text.into(),
            lang_to: lang_to.into(),
            lang_from: lang_from.into(),
            audio: audio,
        }
    }

    fn short_or_long(text: &str) -> bool {
        text.len() < 20
    }
}

/// Backend for searching words. Can be dictserver, mdd/mdx, or online searching.
pub trait Backend {
    fn query(&self, query: std::sync::Arc<Query>) -> Result<WordData, String>;
}
