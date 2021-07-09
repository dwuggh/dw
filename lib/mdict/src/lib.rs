mod mdd;
mod mdict;
mod mdict_header;
mod mdx;
mod utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_parse_header_and_key_blocks() -> Result<(), std::io::Error> {
        let _ = env_logger::builder().is_test(true).try_init();
        // let mdd_file_path = "/home/dwuggh/.dicts/OALDcn8/oald.mdd";
        let mdx_file_path = "/home/dwuggh/.dicts/OALDcn8/oald.mdx";
        let mut f = std::fs::File::open(mdx_file_path)?;
        let header = mdict_header::MDictHeader::parse_header(&mut f)?;
        let keys = header.read_keys(&mut f)?;
        header.mdx_decode_record_block(&mut f, keys)?;
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "asdfsdf"))
        // Ok(())
    }
}
