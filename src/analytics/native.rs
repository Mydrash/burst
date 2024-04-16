use std::{
    io::{BufRead, Seek},
    ops::Range,
};

use log::info;

const PNG_SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";
const CHUNK_SIZE: usize = PNG_SIGNATURE.len();

/// scan for PNGs headers in a binary blob.
/// this fucks-out with the cursor position so make sure to reset it :)
pub fn scan_for_pngs<R: BufRead + Seek>(
    source: &mut R,
    buf_size: usize,
) -> anyhow::Result<Vec<Range<u64>>> {
    let mut found = vec![];
    let mut search_buffer = vec![0u8; buf_size];
    let mut bytes_read: usize;
    let mut cursor_pos = source.stream_position()?;
    let mut did_find = Option::<u64>::None;

    info!("Searching using a buffer with {buf_size} of size...");

    loop {
        bytes_read = source.read(&mut search_buffer[..])?;

        if bytes_read == 0 {
            break;
        }

        let contents = &search_buffer[0..bytes_read];

        for pos in 0..contents.len() {
            if pos + CHUNK_SIZE <= contents.len()
                && contents[pos..pos + CHUNK_SIZE] == *PNG_SIGNATURE
            {
                if let Some(start_pos) = did_find.take() {
                    let end_pos = cursor_pos;
                    info!("<- Found a PNG Image at {cursor_pos:x}");
                    found.push(start_pos..end_pos);
                } else {
                    info!("-> Found a PNG Signature at {cursor_pos:x}");
                    did_find = Some(cursor_pos);
                }
            }

            cursor_pos += 1;
        }
    }

    Ok(found)
}
