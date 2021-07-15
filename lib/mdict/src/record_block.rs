#![allow(dead_code)]
#![allow(unused)]

use byteorder::ReadBytesExt;
use byteorder::{BigEndian, LittleEndian};
use flate2::read::ZlibDecoder;
use std::io::{Cursor, Read};

use crate::mdict::{Binarys, Datas, KeyList, Words};
use crate::mdict_header::MDictHeader;
use crate::utils::assert_adler32_checksum;

type RecordBlockInfoList = Vec<(usize, usize)>;

impl MDictHeader {
    pub fn mdd_decode_record_block(
        &self,
        f: &mut impl Read,
        key_list: KeyList,
    ) -> std::io::Result<Binarys> {
        self.decode_record_block(f, key_list, |a| a.clone().into())
    }

    pub fn mdx_decode_record_block(
        &self,
        f: &mut impl Read,
        key_list: KeyList,
    ) -> std::io::Result<Words> {
        self.decode_record_block(f, key_list, |buf| {
            // TODO should decode first, as record_text is encoded in self.encoding
            std::str::from_utf8(buf).unwrap().into()
        })
    }

    pub fn decode_record_block<T, F>(
        &self,
        f: &mut impl Read,
        key_list: KeyList,
        data_processor: F,
    ) -> std::io::Result<Datas<T>>
    where
        F: Fn(&[u8]) -> T,
    {
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
        log::info!(
            "size of record block: {} kilobytes",
            record_block_size / 1024
        );

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

        let mut i: usize = 0;
        let mut offset: usize = 0;
        let mut datas = Datas::new();

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
                    let mut record_block = Vec::with_capacity(decompressed_size);
                    // let mut record_block = Vec::new();
                    d.read_to_end(&mut record_block)?;
                    record_block
                }
                _ => {
                    // TODO raise an error
                    todo!()
                }
            };
            assert_eq!(decompressed_size, record_block.len());
            assert_adler32_checksum(&record_block, adler32_checksum);

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

                datas.insert(key_text.to_string(), data_processor(record_text_bytes));
            }
            offset = offset + record_block.len();
        }
        Ok(datas)
    }
}
