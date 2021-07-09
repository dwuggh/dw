#![allow(dead_code)]

#![allow(unused)]

use byteorder::ReadBytesExt;
use byteorder::{BigEndian, LittleEndian};
use flate2::read::ZlibDecoder;
use std::io::{Cursor, Read};

use crate::mdict::KeyList;
use crate::mdict_header::MDictHeader;
use crate::mdx::Word;
use crate::utils::assert_adler32_checksum;

type RecordBlockInfoList = Vec<(usize, usize)>;

impl MDictHeader {
    pub fn mdd_decode_record_block(
        &self,
        f: &mut impl Read,
        key_list: KeyList,
    ) -> std::io::Result<()> {
        let num_record_blocks = self.read_number(f)?;
        // TODO this number should equal to num_entries in key blocks
        let _num_record_entries = self.read_number(f)?;
        let record_block_info_size = self.read_number(f)?;
        let record_block_size = self.read_number(f)?;

        log::info!("number of record blocks: {}", num_record_blocks);
        log::info!("number of entries: {}", _num_record_entries);
        log::info!(
            "number of bytes of record block info: {}",
            record_block_info_size
        );
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

        todo!()
    }
}
