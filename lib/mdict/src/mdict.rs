#![allow(dead_code)]

use crate::mdict_header::*;
use std::{collections::HashMap, fs::File};

pub type Datas<T> = HashMap<String, T>;

pub type Words = Datas<String>;
pub type Binarys = Datas<Vec<u8>>;

/// the first element is key_id, the second is the key itself, normally a word or phrase.
/// A typical KeyList item: (348951919, "zero tolerances")
/// TODO maybe using a hashmap here?
pub(crate) type KeyList = Vec<(u64, String)>;

/// MDict stores the dictionary definitions, i.e. (key word, explanation) in MDX file and
/// the dictionary reference data, e.g. images, pronunciations, stylesheets in MDD file.
#[derive(Clone, Debug)]
pub struct MDict {
    /// .mdx file path
    pub mdx_file_path: String,

    /// header info
    pub header: MDictHeader,

    pub mdd_asset: Option<MDictAsset>,

    pub words: Words,
}

/// Assets stored in .mdd file, usually are images and css tables.
#[derive(Clone, Debug)]
pub struct MDictAsset {
    pub mdd_file_path: String,

    /// header info
    pub header: MDictHeader,

    assets: Binarys,
}

impl MDict {
    pub fn new(mdx_file_path: &str, mdd_file_path: Option<String>) -> std::io::Result<MDict> {
        let mut f = std::io::BufReader::new(File::open(mdx_file_path)?);
        let header = MDictHeader::parse_header(&mut f, "UTF-16")?;

        let mut key_list = header.read_keys(&mut f)?;
        key_list.sort_by(|a, b| a.0.cmp(&b.0));
        let words = header.mdx_decode_record_block(&mut f, key_list)?;

        let mdd_asset = if let Some(mdd_file_path) = mdd_file_path {
            let mut f = std::io::BufReader::new(File::open(&mdd_file_path)?);
            let header = MDictHeader::parse_header(&mut f, "UTF-16")?;
            let mut key_list = header.read_keys(&mut f)?;
            key_list.sort_by(|a, b| a.0.cmp(&b.0));
            let assets = header.mdd_decode_record_block(&mut f, key_list)?;
            Some(MDictAsset {
                mdd_file_path,
                header,
                assets,
            })
        } else {
            None
        };

        let mdd = MDict {
            mdx_file_path: mdx_file_path.to_string(),
            header,
            mdd_asset,
            words,
        };

        return Ok(mdd);
    }

    pub fn lookup(&self, word: &str) -> Option<&String> {
        self.words.get(word)
    }

    pub fn lookup_assets(&self, word: &str) -> Option<&Vec<u8>> {
        self.mdd_asset.as_ref()?.assets.get(word)
    }
}
