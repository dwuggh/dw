#![allow(dead_code)]

use crate::mdict_header::*;
use dashmap::DashMap;
use std::fs::File;

/// full information for a word in mdict
#[derive(Debug, PartialEq, Eq, Clone)]
struct Word {
    word: String,
}

impl std::hash::Hash for Word {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.word, state)
    }
}

/// the first element is key_id, the second is the key itself, normally a word or phrase.
/// A typical KeyList item: (348951919, "zero tolerances")
/// TODO maybe using a hashmap here?
type KeyList = Vec<(u64, String)>;

/// MDict stores the dictionary definitions, i.e. (key word, explanation) in MDX file and
/// the dictionary reference data, e.g. images, pronunciations, stylesheets in MDD file.
#[derive(Debug)]
pub struct MDict {
    /// .mdx file path
    pub mdx_file_path: String,
    pub mdd_file_path: Option<String>,

    /// header info
    pub header: MDictHeader,

    /// key list in key blocks
    key_list: KeyList,

    // words
    dict_map: DashMap<String, Word>,
}

impl MDict {
    pub fn new(mdx_file_path: &str, mdd_file_path: Option<String>) -> std::io::Result<MDict> {
        let mut f = std::io::BufReader::new(File::open(mdx_file_path)?);
        let header = MDictHeader::parse_header(&mut f)?;

        let key_list = header.read_keys(&mut f)?;

        let dict_map = DashMap::new();

        let mdd = MDict {
            mdx_file_path: mdx_file_path.to_string(),
            mdd_file_path,
            header,
            key_list,
            dict_map,
        };

        return Ok(mdd);
    }
}
