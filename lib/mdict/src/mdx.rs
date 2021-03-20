use byteorder::ReadBytesExt;
use byteorder::{BigEndian, LittleEndian};
use flate2::read::ZlibDecoder;
use std::io::{Cursor, Read};

use crate::mdict::KeyList;
use crate::mdict_header::MDictHeader;
use crate::utils::assert_adler32_checksum;

type RecordBlockInfoList = Vec<(usize, usize)>;

#[derive(Debug)]
pub struct Word {
    key_text: String,
    record_text: String,
}

impl Word {
    fn new(key_text: String, record_text: String) -> Self {
        Self {
            key_text,
            record_text,
        }
    }
}

type Words = Vec<Word>;

impl MDictHeader {
    pub fn mdx_decode_record_block(
        &self,
        f: &mut impl Read,
        key_list: KeyList,
    ) -> std::io::Result<Words> {
        let num_record_blocks = self.read_number(f)?;
        // TODO this number should equal to num_entries in key blocks
        let _num_record_entries = self.read_number(f)?;
        let record_block_info_size = self.read_number(f)?;
        let record_block_size = self.read_number(f)?;

        log::info!("number of record blocks: {}", num_record_blocks);
        log::info!("number of entries: {}", _num_record_entries);
        log::info!("number of bytes of record block info: {}", record_block_info_size);
        log::info!("number of bytes of record block: {}", record_block_size);

        // info section
        let mut buf = vec![0; record_block_info_size as usize];
        f.read_exact(&mut buf)?;
        let mut buf_reader = Cursor::new(buf);
        let mut record_block_info_list = RecordBlockInfoList::new();
        loop {
            match self.read_number(&mut buf_reader) {
                Ok(compressed_size) => {
                    let decompressed_size = self.read_number(&mut buf_reader)? as usize;
                    record_block_info_list.push((compressed_size as usize, decompressed_size));
                }
                Err(_) => break,
            }
        }

        // actual record block data
        let mut record_list = Words::new();
        let mut i: usize = 0;
        let mut offset: usize = 0;

        for (compressed_size, decompressed_size) in record_block_info_list {
            let record_block_type = f.read_u32::<LittleEndian>()?;
            let adler32_checksum = f.read_u32::<BigEndian>()?;
            let record_block = match record_block_type {
                0 => {
                    let mut record_block = vec![0; compressed_size - 8];
                    f.read_exact(&mut record_block)?;
                    record_block
                }
                1 => {
                    // TODO lzo compress
                    todo!("lzo compress")
                }
                2 => {
                    // zlib compress
                    let mut record_block_compressed = vec![0; compressed_size - 8];
                    f.read_exact(&mut record_block_compressed)?;
                    let mut d = ZlibDecoder::new(Cursor::new(record_block_compressed));
                    let mut record_block = Vec::new();
                    d.read_to_end(&mut record_block)?;
                    record_block
                }
                _ => {
                    // TODO raise an error
                    todo!()
                }
            };

            assert_adler32_checksum(&record_block, adler32_checksum);

            // TODO this part seems really inefficient
            while i < key_list.len() {
                let (record_start, key_text) = &key_list[i];
                let record_start = *record_start as usize;
                if record_start - offset >= record_block.len() {
                    break;
                }

                let record_end = if i < key_list.len() - 1 {
                    key_list[i + 1].0 as usize
                } else {
                    record_block.len() + offset
                };

                i = i + 1;

                let record_text_bytes = &record_block[record_start - offset..record_end - offset];

                // TODO should decode first, as record_text is encoded in self.encoding
                let record_text = std::str::from_utf8(&record_text_bytes).unwrap();
                log::debug!("{}", record_text);
                println!("{}", record_text);

                record_list.push(Word::new(key_text.to_string(), record_text.to_string()));
            }

            offset = offset + record_block.len();
        }

        Ok(record_list)
    }
}
