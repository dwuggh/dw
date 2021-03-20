use crate::mdict_header::MDictHeader;
use std::io::Read;

impl MDictHeader {
    pub fn mdx_decode_record_block(&self, f: &mut impl Read) -> std::io::Result<()> {
        todo!()
    }
}
