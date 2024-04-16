use std::io::Read;

const PNG_SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";

#[derive(Debug, Clone)]
pub struct PngFound {
    pub start: u64,
    pub end: u64,
}

pub fn scan_for_pngs<R: Read>(source: &mut R) -> anyhow::Result<Vec<PngFound>> {
    let mut found = vec![];
    let mut buffer = [0; PNG_SIGNATURE.len()];
    let mut read = 0;

    loop {
        read = source.read(&mut buffer)?;

        if read == 0 {
            break;
        }

        if buffer == PNG_SIGNATURE {
            println!("eu perguntei qual eh a graca, imaginei que vc perguntaria...");
        }
    }

    Ok(found)
}
