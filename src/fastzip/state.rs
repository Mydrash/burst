use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct State {
    // [FILENAME]: [Last modified Timestamp]
    pub files: HashMap<PathBuf, Duration>,
}

impl State {
    pub(crate) fn load(path: &Path) -> anyhow::Result<Self> {
        let bytes = fs::read(path).context("unable to read state file")?;
        bincode::deserialize(&bytes).context("unable to deserialize state file")
    }

    pub(crate) fn save(&self, path: &Path) -> anyhow::Result<()> {
        let bytes = bincode::serialize(&self).context("unable to serialize state file")?;
        fs::write(path, bytes).context("unable to write state file")
    }
}