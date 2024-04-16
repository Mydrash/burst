use anyhow::{bail, Context};
use argh::FromArgs;
use log::info;
use std::fs::{metadata, File};

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
    let mut input = File::open(info.native_lib).context("unable to open source file")?;

    scan_for_pngs(&mut input)?;
    Ok(())
}
