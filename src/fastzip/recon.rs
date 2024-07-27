use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, UNIX_EPOCH};
use jwalk::WalkDir;

pub fn recognize_timestamps(directory: &Path) -> HashMap<PathBuf, Duration> {
    let entries = WalkDir::new(directory)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| {
            let path = entry.path();
            let metadata = entry.metadata().unwrap();
            let modified = metadata.modified().unwrap();
            let timestamp = modified.duration_since(UNIX_EPOCH).unwrap();

            (path, timestamp)
        });

    entries.collect()
}