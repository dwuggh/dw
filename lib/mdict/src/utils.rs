pub fn assert_adler32_checksum(buffer: &[u8], checksum: u32) {
    // an alternative way:
    // let adler32_checksum1 = adler32::adler32(Cursor::new(&header_bytes))?;
    let mut rolling_adler32 = adler32::RollingAdler32::new();
    rolling_adler32.update_buffer(buffer);
    let adler32_checksum1 = rolling_adler32.hash();
    log::debug!(
        "adler32_checksum: {:#x}, {:#x}",
        adler32_checksum1,
        checksum
    );
    assert!(
        adler32_checksum1 == checksum & 0xffffffff,
        "header checksum validation failed!"
    );
}

pub fn read_utf16_string(slice: &[u8]) -> Option<String> {
    // assert!(2 * size <= slice.len());
    let size = slice.len() / 2;
    let iter = (0..size).map(|i| u16::from_le_bytes([slice[2 * i], slice[2 * i + 1]]));

    std::char::decode_utf16(iter)
        .collect::<Result<String, _>>()
        .ok()
}
