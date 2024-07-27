use argh::FromArgs;

use crate::core::config::Config;
use crate::core::state::{install_apktool, install_uber_apk_signer};

#[derive(FromArgs, Clone, Debug)]
#[argh(subcommand, name = "x")]
/// executes a tool 
pub(crate) struct Execute {
    #[argh(subcommand)]
    pub nested: Tool,
}

#[derive(FromArgs, Clone, Debug)]
#[argh(subcommand)]
pub enum Tool {
    ApkTool(ApkTool),
    UberApkSigner(UberApkSigner),
}

#[derive(FromArgs, Clone, Debug)]
#[argh(subcommand, name = "apktool")]
/// Runs apktool
pub struct ApkTool {
    #[argh(positional, greedy)]
    pub args: Vec<String>,
}

#[derive(FromArgs, Clone, Debug)]
#[argh(subcommand, name = "uber-apk-signer")]
/// Runs uber-apk-signer
pub struct UberApkSigner {
    #[argh(positional, greedy)]
    pub args: Vec<String>,
}

pub fn execute(info: Execute) -> anyhow::Result<()> {
    let mut config = Config::load_or_create()?;

    match info.nested {
        Tool::ApkTool(info) => {
            let apktool = if config.apktool.is_none() {
                install_apktool(&mut config)?
            } else {
                config.apktool.as_ref().expect("BUG: apktool is not in state.")
            };

            apktool.run(info.args)?;
        }
        Tool::UberApkSigner(info) => {
            let uber_apk_signer = if config.uber_apk_signer.is_none() {
                install_uber_apk_signer(&mut config)?
            } else {
                config.uber_apk_signer.as_ref().expect("BUG: uber-apk-signer is not in state.")
            };

            uber_apk_signer.run(info.args)?;
        }
    }

    Ok(())
}

