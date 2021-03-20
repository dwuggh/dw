use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use flate2::read::ZlibDecoder;
use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::Cursor;

use crate::utils::*;

// TODO use serde
/// stylesheet attribute if present takes form of:
///   style_number # 1-255
///   style_begin  # or ''
///   style_end    # or ''
/// store stylesheet in dict in the form of
/// {'number' : ('style_begin', 'style_end')}
#[derive(Debug)]
pub(crate) enum StyleSheet {
    Number(u32),
    Begin,
    End,
    Dict(HashMap<String, Vec<String>>),
}

/// mdict's metainfo, extracted from header
#[derive(Debug)]
pub struct MDictHeader {
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
    pub(crate) encrypt: u32,

    pub(crate) stylesheet: StyleSheet,

    pub(crate) number_width: u32,
    pub(crate) number_format: String,
}

impl MDictHeader {
    /// reading key blocks in mdict
    /// TODO encryption
    pub fn read_keys<R: Read>(&self, f: &mut R) -> std::io::Result<Vec<(u64, String)>> {
        let block_bytes_size: usize = if self.version >= 2.0 { 8 * 5 } else { 4 * 4 };

        let mut block_bytes = vec![0; block_bytes_size];
        f.read_exact(&mut block_bytes)?;

        if self.version >= 2.0 {
            let checksum = f.read_u32::<BigEndian>().unwrap();
            assert_adler32_checksum(&block_bytes, checksum);
        }

        // TODO encryption
        if self.encrypt != 0 {}

        let mut block_bytes_reader = Cursor::new(block_bytes);
        // number of key blocks
        let num_key_blocks = self.read_number(&mut block_bytes_reader)?;
        // number of entries
        let num_entries = self.read_number(&mut block_bytes_reader)?;

        // number of bytes of key block info after decompression
        let _key_block_info_decomp_size = if self.version >= 2.0 {
            Some(self.read_number(&mut block_bytes_reader)?)
        } else {
            None
        };

        // number of bytes of key block info
        let key_block_info_size = self.read_number(&mut block_bytes_reader)?;
        // number of bytes of key block
        let key_block_size = self.read_number(&mut block_bytes_reader)?;

        log::info!("number of key blocks: {}", num_key_blocks);
        log::info!("number of entries: {}", num_entries);
        log::info!("number of bytes of key block info: {}", key_block_info_size);
        log::info!("number of bytes of key block: {}", key_block_size);

        // read key block info, which indicates key block's compressed and decompressed size
        let key_block_info_list = self.decode_key_block_info(f, key_block_info_size as usize)?;
        self.decode_key_block(f, key_block_size as usize, key_block_info_list)
    }

    fn decode_key_block_info(
        &self,
        f: &mut impl Read,
        key_block_info_size: usize,
    ) -> std::io::Result<Vec<(usize, usize)>> {
        let mut key_block_info_compressed = vec![0; key_block_info_size];
        let mut key_block_info = Vec::new();
        f.read_exact(&mut key_block_info_compressed)?;
        let mut key_block_info_reader = Cursor::new(key_block_info_compressed);

        if self.version >= 2.0 {
            // TODO encryption
            let _zlib_mark = key_block_info_reader.read_u32::<LittleEndian>()?;
            log::debug!("zlib mark should be 0x02: {:#x}", _zlib_mark);
            let checksum = key_block_info_reader.read_u32::<BigEndian>()?;
            let mut d = ZlibDecoder::new(key_block_info_reader);
            let _size = d.read_to_end(&mut key_block_info)?;
            assert_adler32_checksum(&key_block_info, checksum);
        } else {
            key_block_info_reader.read_to_end(&mut key_block_info)?;
        };
        let mut key_block_info_reader = Cursor::new(key_block_info);
        self.decode_key_block_info_list(&mut key_block_info_reader)
    }

    /// return a list of tuples, the first element is key_block_compressed_size,
    /// the second element is key_block_decompressed_size.
    fn decode_key_block_info_list(
        &self,
        f: &mut impl Read,
    ) -> std::io::Result<Vec<(usize, usize)>> {
        let mut key_block_info_list = Vec::<(usize, usize)>::new();
        loop {
            match self.read_number(f) {
                Ok(_num_entries) => {
                    let text_head_size = if self.version >= 2.0 {
                        f.read_u16::<BigEndian>()?
                    } else {
                        f.read_u8()? as u16
                    };

                    let text_term = if self.version >= 2.0 { 1 } else { 0 };

                    let tex_head_term_size = if self.encoding == "UTF-16" {
                        2 * (text_head_size + text_term)
                    } else {
                        text_head_size + text_term
                    };

                    log::info!("tex_head_term_size: {}", tex_head_term_size);

                    // don't know what this buffer is
                    let mut _buf = vec![0; tex_head_term_size as usize];
                    f.read_exact(&mut _buf)?;

                    let _str = std::str::from_utf8(&_buf).unwrap();
                    log::info!("{}", _str);

                    let text_tail_size = if self.version >= 2.0 {
                        f.read_u16::<BigEndian>()?
                    } else {
                        f.read_u8()? as u16
                    };

                    let tex_tail_term_size = if self.encoding == "UTF-16" {
                        2 * (text_tail_size + text_term)
                    } else {
                        text_tail_size + text_term
                    };

                    let mut _buf = vec![0; tex_tail_term_size as usize];
                    f.read_exact(&mut _buf)?;
                    log::info!("tex_tail_term_size: {}", tex_tail_term_size);

                    let _str = std::str::from_utf8(&_buf).unwrap();
                    log::info!("{}", _str);

                    let key_block_compressed_size = self.read_number(f)? as usize;
                    let key_block_decompressed_size = self.read_number(f)? as usize;

                    key_block_info_list
                        .push((key_block_compressed_size, key_block_decompressed_size));
                }
                Err(_) => break,
            }
        }

        Ok(key_block_info_list)
    }

    ///
    fn decode_key_block(
        &self,
        f: &mut impl Read,
        key_block_size: usize,
        key_block_info_list: Vec<(usize, usize)>,
    ) -> std::io::Result<Vec<(u64, String)>> {
        let mut key_block_compressed = vec![0; key_block_size as usize];
        f.read_exact(&mut key_block_compressed)?;

        let mut reader = Cursor::new(key_block_compressed);

        let mut key_list = Vec::<(u64, String)>::new();

        for (compressed_size, _decompressed_size) in key_block_info_list {
            // 0x00000002
            let key_block_type = reader.read_u32::<LittleEndian>()?;
            let adler32_checksum = reader.read_u32::<BigEndian>()?;

            let key_block = match key_block_type {
                0 => {
                    let mut key_block = vec![0; compressed_size - 8];
                    reader.read_exact(&mut key_block)?;
                    key_block
                }
                1 => {
                    // TODO lzo compress
                    todo!("lzo compress")
                }
                2 => {
                    // zlib compress
                    let mut key_block_compressed = vec![0; compressed_size - 8];
                    reader.read_exact(&mut key_block_compressed)?;
                    let mut d = ZlibDecoder::new(Cursor::new(key_block_compressed));
                    let mut key_block = Vec::new();
                    d.read_to_end(&mut key_block)?;
                    key_block
                }
                _ => {
                    // TODO raise an error
                    todo!()
                }
            };
            // let key_list = self.split_key_block(&key_block)?;
            key_list.append(&mut self.split_key_block(&key_block)?);
            assert_adler32_checksum(&key_block, adler32_checksum);
        }

        log::debug!("successful decode key_list with {} items", key_list.len());

        Ok(key_list)
    }

    fn split_key_block(&self, key_block: &[u8]) -> std::io::Result<Vec<(u64, String)>> {
        let mut reader = Cursor::new(key_block);

        let mut key_list = Vec::<(u64, String)>::new();

        loop {
            match self.read_number(&mut reader) {
                Ok(key_id) => {
                    let mut key_text_bytes = Vec::<u8>::new();
                    // read all bytes until EOF is met. 0x00 for u8, 0x0000 for u16.
                    loop {
                        match self.read_u8_or_u16(&mut reader) {
                            Ok(mut i) => {
                                if i.len() == 1 && i[0] == 0 {
                                    break;
                                } else if i.len() == 2 && i[0] == 0 && i[1] == 0 {
                                    break;
                                } else {
                                    key_text_bytes.append(&mut i);
                                }
                            }
                            Err(_) => {
                                // TODO
                                break;
                            }
                        }
                    }
                    // transfrom key_text_bytes to utf-8 string
                    // TODO should decode first, as key_text is encoded in self.encoding
                    let key_text = std::str::from_utf8(&key_text_bytes).unwrap();
                    key_list.push((key_id, key_text.to_string()))
                }
                Err(_) => break,
            }
        }
        Ok(key_list)
    }

    pub(crate) fn read_u8_or_u16(&self, f: &mut impl Read) -> std::io::Result<Vec<u8>> {
        if self.encoding == "UTF-16" {
            f.read_u16::<BigEndian>()
                .map(|x| u16::to_le_bytes(x).to_vec())
        } else {
            f.read_u8().map(|x| vec![x])
        }
    }

    pub(crate) fn read_number<R: Read>(&self, f: &mut R) -> std::io::Result<u64> {
        if self.version < 2.0 {
            f.read_u32::<BigEndian>().map(|x| x as u64)
        } else {
            f.read_u64::<BigEndian>()
        }
    }
}

impl MDictHeader {
    pub fn parse_header<R: Read>(f: &mut R) -> std::io::Result<MDictHeader> {
        // let mut header_bytes_size = Cursor::new([0; 4]);
        let header_bytes_size = f.read_u32::<BigEndian>().unwrap() as usize;
        log::info!("header_byte_size: {}", header_bytes_size);
        // Dict info string, utf-16 encoding
        let mut header_bytes = vec![0; header_bytes_size];
        f.read_exact(&mut header_bytes)?;

        /*
         hash checksum validation
        */
        let adler32_checksum2 = f.read_u32::<LittleEndian>().unwrap();
        assert_adler32_checksum(&header_bytes, adler32_checksum2);

        // header text in utf-16 encoding ending with '\x00\x00', remove the last 4 bytes
        header_bytes.truncate(header_bytes.len() - 4);
        // 4 bytes: adler32 checksum of header, in little endian
        let dict_info = read_utf16_string(&header_bytes).unwrap();
        log::info!("{}", dict_info);

        let attributes = MDictHeader::read_header_attributes(&dict_info);

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
            for _line in stylesheet_raw.lines() {
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
        let metainfo = MDictHeader {
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
    fn read_header_attributes(text: &str) -> HashMap<String, String> {
        let re = Regex::new(r#"(\w+)="(.*?)""#).unwrap();
        let mut map = HashMap::new();
        for caps in re.captures_iter(text) {
            map.insert(caps[1].to_string(), caps[2].to_string());
        }
        log::info!("{:?}", map);
        return map;
    }
}

