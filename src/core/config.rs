use std::fs::{create_dir_all, read, write};
use std::path::PathBuf;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use crate::core::BURST_CONFIG_ROOT;
#[derive(Deserialize, Serialize, Default)]
pub struct JavaTool {
    pub version: String,
    pub path: PathBuf,
}

impl JavaTool {
    pub fn run(&self, args: Vec<String>) -> anyhow::Result<()> {
        let mut command = std::process::Command::new("java");
        command.arg("-jar").arg(&self.path);
        command.args(args);

        let status = command.status().context("unable to run java command")?;
        if !status.success() {
            anyhow::bail!("java command failed with status: {}", status);
        }

        Ok(())
    }
}


#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    pub apktool: Option<JavaTool>,
    pub uber_apk_signer: Option<JavaTool>,
    pub dex2jar: Option<JavaTool>,
}

impl Config {
    fn load() -> anyhow::Result<Self> {
        let config_file = BURST_CONFIG_ROOT.join("config.bin");
        let bytes = read(&config_file).context("unable to read config")?;
        bincode::deserialize(&bytes).map_err(|e| e.into())
    }

    pub fn save(&self) -> anyhow::Result<()> {
        if !BURST_CONFIG_ROOT.exists() {
            create_dir_all(BURST_CONFIG_ROOT.as_path()).context("unable to create config root")?;
        }

        let config_file = BURST_CONFIG_ROOT.join("config.bin");
        let bytes = bincode::serialize(&self).context("unable to serialize config")?;
        write(&config_file, bytes).context("unable to write config")
    }

    pub fn load_or_create() -> anyhow::Result<Self> {
        if BURST_CONFIG_ROOT.exists() {
            Self::load()
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }
}


impl Drop for Config {
    fn drop(&mut self) {
        self.save().expect("unable to save config");
    }
}