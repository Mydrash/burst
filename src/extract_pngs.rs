use anyhow::{bail, Context};
use argh::FromArgs;
use log::info;
use std::{
    fs::{metadata, File, OpenOptions},
    io::{copy, BufReader, Read, Seek, SeekFrom},
    path::PathBuf,
};

use crate::native::scan_for_pngs;

#[derive(FromArgs, Clone, Debug)]
#[argh(subcommand, name = "extract-pngs")]
/// extract the pngs from the game
pub struct ExtractPngs {
    #[argh(option, short = 'N')]
    /// the shared library file
    pub native_lib: String,

    #[argh(option, short = 'd')]
    /// the destnation directory
    pub destination: String,

    #[argh(option, short = 'B', default = "65536")]
    /// the buffer size
    pub buffer_size: usize,
}

pub fn extract_pngs(info: ExtractPngs) -> anyhow::Result<()> {
    if !metadata(&info.native_lib)
        .context("unable to open the source file")?
        .is_file()
    {
        bail!("the source isn't a file.");
    }

    if !metadata(&info.destination)
        .context("unable to open target directory")?
        .is_dir()
    {
        bail!("the target directory isn't a directory.");
    }

    info!("analyzing {}...", info.native_lib);
    let mut image_count = 1;

    let file = File::open(info.native_lib).context("unable to open source file")?;
    let mut input = BufReader::new(file);

    let ranges = scan_for_pngs(&mut input, info.buffer_size)?;

    input.seek(SeekFrom::Start(0))?;

    for pos in ranges {
        // Ain miga esse target tá mto divaaaa
        let mut path = PathBuf::from(&info.destination);
        path.push(format!("Image {image_count}.png"));

        info!("Copying {pos:?} into {path:?}");

        let mut output = OpenOptions::new().write(true).read(true).open(path)?;

        input.seek(SeekFrom::Start(pos.start as _))?;

        copy(
            &mut input.by_ref().take((pos.end - pos.start) as _),
            &mut output,
        )?;

        image_count += 1;
    }

    Ok(())
}
