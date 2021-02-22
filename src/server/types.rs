/// TODO audio type
#[derive(Debug)]
pub struct Audio;

#[derive(Debug)]
pub struct WordData<'a> {
    /// backend identifier
    pub backend: String,
    pub query: std::sync::Arc<Query<'a>>,
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
pub struct Query<'a> {
    /// the query text
    pub text: &'a str,
    /// which language does this literature originally belongs to
    pub lang_from: String,
    /// which language description belongs to
    pub lang_to: String,
    pub audio: bool,
}

impl<'a> Query<'a> {
    pub fn new<S: Into<String>>(text: &'a str, lang_from: S, lang_to: S, audio: bool) -> Query<'a> {
        Query {
            text,
            lang_to: lang_to.into(),
            lang_from: lang_from.into(),
            audio: audio,
        }
    }
}

/// Backend for searching words. Can be dictserver, mdd/mdx, or online searching.
pub trait Backend {
    fn query<'a>(&self, query: std::sync::Arc<Query<'a>>) -> Result<WordData<'a>, String>;
}
