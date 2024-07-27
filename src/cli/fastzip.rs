use crate::fastzip::recon::recognize_timestamps;
use crate::fastzip::state::State;
use anyhow::Context;
use argh::FromArgs;
use log::{info, warn};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::copy;
use std::path::{Path, PathBuf};
use std::time::Duration;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

#[derive(FromArgs, Clone, Debug)]
#[argh(subcommand, name = "fastzip")]
/// Fast compresses files into a zip file
pub struct FastZip {
    #[argh(option, short = 'i')]
    /// the directory to compress
    pub input_directory_path: String,

    #[argh(option, short = 'o')]
    /// the output zip file
    pub output: String,
}

fn only_diffs(
    pre: &HashMap<PathBuf, Duration>,
    post: &HashMap<PathBuf, Duration>,
) -> HashMap<PathBuf, Duration> {
    let mut diffs = HashMap::new();

    for (file, post_timestamp) in post {
        let pre_timestamp = pre.get(file);

        if pre_timestamp.is_none() || pre_timestamp.unwrap() != post_timestamp {
            diffs.insert(file.clone(), *post_timestamp);
        }
    }

    diffs
}

pub fn fastzip(info: FastZip) -> anyhow::Result<()> {
    let input_path = Path::new(&info.input_directory_path);
    let output_path = Path::new(&info.output);
    let state_path = output_path.with_extension("zip-state");

    let zip_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_path)
        .context("unable to create the output zip file")?;

    let mut state: State;

    if state_path.exists() {
        state = State::load(&state_path)?;
    } else {
        state = State::default();
        state.save(&state_path)?;
    }

    let post = recognize_timestamps(&input_path);
    let diffs = only_diffs(&state.files, &post);

    if diffs.is_empty() {
        warn!("No changes detected. no work needed!");
        return Ok(());
    }

    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    let mut zip = ZipWriter::new(zip_file);

    for (file, _) in diffs {
        info!("Adding {:?} to the zip file", file);
        zip.start_file(file.to_string_lossy(), options)?;
        let mut file = File::open(file).context("unable to read file")?;
        copy(&mut file, &mut zip)?;
    }

    zip.finish()?;

    state.files = post;
    state.save(&state_path)?;

    Ok(())
}
