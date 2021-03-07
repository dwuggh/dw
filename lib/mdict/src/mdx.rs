use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::PathBuf;

/// full information for a word in mdict
#[derive(Debug)]
struct Word {}

// TODO use serde
/// stylesheet attribute if present takes form of:
///   style_number # 1-255
///   style_begin  # or ''
///   style_end    # or ''
/// store stylesheet in dict in the form of
/// {'number' : ('style_begin', 'style_end')}
#[derive(Debug)]
enum StyleSheet {
    Number(u32),
    Begin,
    End,
    Dict(HashMap<String, Vec<String>>),
}

/// mdict's metainfo, extracted from header
#[derive(Debug)]
pub struct MetaInfo {
    /// mdict version, breaking changes occured in 2.0
    pub version: f32,

    /// encoding of this .mdd file, retrieved from header
    pub encoding: String,
    /// TODO passcode
    pub passcode: Option<String>,

    /// encryption flag
    /// - 0x00: no encryption
    /// - 0x01: encrypt record block
    /// - 0x02: encrypt key info block
    encrypt: u32,

    stylesheet: StyleSheet,

    number_width: u32,
    number_format: String,
}

#[derive(Debug)]
pub struct MDD {
    /// .mdd file path
    pub file_path: String,

    /// header info
    pub header: MetaInfo,

    // words
    dict: evmap::handles::ReadHandle<String, Word>,
}

impl MDD {
    pub fn new(mdd_file_path: &str) -> std::io::Result<MDD> {
        let mut f = std::io::BufReader::new(File::open(mdd_file_path)?);
        // let mut reader = std::io::BufReader::new(f);
        let header = MetaInfo::parse_header(&mut f)?;

        todo!()
    }
}

impl MetaInfo {
    fn parse_header<R: Read>(f: &mut R) -> std::io::Result<MetaInfo> {
        // let mut header_bytes_size = Cursor::new([0; 4]);
        let header_bytes_size = f.read_u32::<BigEndian>().unwrap() as usize;
        log::info!("header_byte_size: {}", header_bytes_size);
        // Dict info string, utf-16 encoding
        let mut header_bytes = vec![0; header_bytes_size];
        f.read_exact(&mut header_bytes)?;
        // an alternative way:
        // let adler32_checksum1 = adler32::adler32(Cursor::new(&header_bytes))?;
        let mut rolling_adler32 = adler32::RollingAdler32::new();
        rolling_adler32.update_buffer(&header_bytes);
        let adler32_checksum1 = rolling_adler32.hash();
        let adler32_checksum2 = f.read_u32::<LittleEndian>().unwrap();
        log::debug!(
            "adler32_checksum: {:#x}, {:#x}",
            adler32_checksum1,
            adler32_checksum2
        );
        assert!(
            adler32_checksum1 == adler32_checksum2 & 0xffffffff,
            "header checksum validation failed!"
        );

        // header text in utf-16 encoding ending with '\x00\x00', remove the last 4 bytes
        header_bytes.truncate(header_bytes.len() - 4);
        // 4 bytes: adler32 checksum of header, in little endian
        let dict_info = read_utf16_string(&header_bytes).unwrap();
        log::info!("{}", dict_info);

        let attributes = MetaInfo::read_header_attributes(&dict_info);

        let version: f32 = attributes
            .get("GeneratedByEngineVersion")
            .unwrap()
            .parse::<f32>()
            .unwrap();

        let encoding: String = attributes.get("Encoding").unwrap().to_string();

        let encrypt: u32 = match attributes.get("Encrypted") {
            Some(e) => {
                if e == "No" {
                    0
                } else if e == "Yes" {
                    1
                } else {
                    e.parse::<u32>().unwrap()
                }
            }
            None => 0,
        };

        // TODO use serde
        let stylesheet_raw = attributes.get("StyleSheet").unwrap();
        let stylesheet = if stylesheet_raw == "" {
            StyleSheet::Begin
        } else if let Ok(n) = stylesheet_raw.parse::<u32>() {
            StyleSheet::Number(n)
        } else {
            let map = HashMap::new();
            for line in stylesheet_raw.lines() {
                todo!()
            }
            StyleSheet::Dict(map)
        };

        let number_width: u32 = if version < 2.0 { 4 } else { 8 };

        let number_format: String = if version < 2.0 {
            String::from(">I")
        } else {
            String::from(">Q")
        };

        // TODO encoding rules, GBK GB2312 GB18030
        let metainfo = MetaInfo {
            version,
            encoding,
            passcode: None,
            encrypt,
            stylesheet,
            number_width,
            number_format,
        };

        Ok(metainfo)
    }

    /// extract all attributes from dict info string
    /// a typical dict info string would look like this:
    /// '''html
    /// <dictionary  GeneratedByEngineVersion="2.0" RequiredEngineVersion="2.0" ... />
    /// '''
    pub fn read_header_attributes(text: &str) -> HashMap<String, String> {
        let re = Regex::new(r#"(\w+)="(.*?)""#).unwrap();
        let mut map = HashMap::new();
        for caps in re.captures_iter(text) {
            map.insert(caps[1].to_string(), caps[2].to_string());
        }
        log::info!("{:?}", map);
        return map;
    }
}

impl MDD {}

pub fn read_utf16_string(slice: &[u8]) -> Option<String> {
    // assert!(2 * size <= slice.len());
    let size = slice.len() / 2;
    let iter = (0..size).map(|i| u16::from_le_bytes([slice[2 * i], slice[2 * i + 1]]));

    std::char::decode_utf16(iter)
        .collect::<Result<String, _>>()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_header_test() -> Result<(), std::io::Error> {
        let _ = env_logger::builder().is_test(true).try_init();
        let mdd_file_path = "/home/dwuggh/.dicts/OALDcn8/oald.mdd";
        let mdx_file_path = "/home/dwuggh/.dicts/OALDcn8/oald.mdx";
        let mut f = File::open(mdx_file_path)?;
        let header = MetaInfo::parse_header(&mut f)?;
        Err(std::io::Error::new(std::io::ErrorKind::Interrupted, "test"))
    }
}
