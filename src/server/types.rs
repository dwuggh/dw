/// TODO audio type
#[derive(Debug)]
pub struct Audio;

#[derive(Debug)]
pub struct RespData {
    /// backend identifier
    pub backend: String,
    pub query: std::sync::Arc<Query>,
    /// basic description about words or sentences' translation.
    pub basic_desc: String,
    pub phonetic_symbol: Option<String>,
    /// detail description for the words.
    pub detail_desc: Option<String>,
    // TODO just placeholder for now
    pub audio: Option<Audio>,
}

/// query for text
#[derive(Debug)]
pub struct Query {
    /// the query text
    pub text: String,
    /// short or long(words&phrases or sentences)
    pub is_short_text: bool,
    /// which language does this literature originally belongs to
    pub lang_from: String,
    /// which language description belongs to
    pub lang_to: String,
    pub audio: bool,
}

impl Query {
    pub fn new<S: Into<String>>(text: S, lang_from: &str, lang_to: &str, audio: bool) -> Query {
        let text: String = text.into();
        let is_short_text = Query::is_short_text(&text);
        Query {
            text,
            is_short_text,
            lang_from: lang_from.into(),
            lang_to: lang_to.into(),
            audio,
        }
    }

    fn is_short_text(text: &str) -> bool {
        text.len() < 20
    }
}

/// Backend for searching words. Can be dictserver, mdd/mdx, or online searching.
pub trait Backend: Send + Sync {
    fn query(&self, query: std::sync::Arc<Query>) -> Result<RespData, String>;
}
