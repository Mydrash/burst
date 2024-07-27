use std::fs::{create_dir_all, File};
use std::io::copy;
use anyhow::Context;
use log::info;
use crate::core::BURST_STATE_ROOT;
use crate::core::config::{Config, JavaTool};

const APKTOOL_VERSION: &str = "2.9.3";
const APKTOOL_URL: &str = "https://github.com/iBotPeaches/Apktool/releases/download/v2.9.3/apktool_2.9.3.jar";
const APKTOOL_NAME: &str = "apktool.jar";

const UBER_APK_SIGNER_VERSION: &str = "1.5.0";
const UBER_APK_SIGNER_URL: &str = "https://github.com/patrickfav/uber-apk-signer/releases/download/v1.3.0/uber-apk-signer-1.3.0.jar";
const UBER_APK_SIGNER_NAME: &str = "uber-apk-signer.jar";

pub fn install_apktool(config: &mut Config) -> anyhow::Result<&JavaTool> {
    info!("Downloading apktool...");

    let mut reader = ureq::get(APKTOOL_URL)
        .call()?
        .into_reader();

    if !BURST_STATE_ROOT.exists() {
        create_dir_all(BURST_STATE_ROOT.as_path()).context("unable to create state root")?;
    }

    let apktool_path = BURST_STATE_ROOT.join(APKTOOL_NAME);
    let mut writer = File::create(&apktool_path).context("unable to create apktool file")?;
    copy(&mut reader, &mut writer).context("unable to copy apktool file")?;

    config.apktool = Some(JavaTool {
        version: APKTOOL_VERSION.to_string(),
        path: apktool_path,
    });

    info!("Installed apktool v{}!", APKTOOL_VERSION);

    Ok(config.apktool.as_ref().unwrap())
}

pub fn install_uber_apk_signer(config: &mut Config) -> anyhow::Result<&JavaTool> {
    info!("Downloading uber-apk-signer...");

    let mut reader = ureq::get(UBER_APK_SIGNER_URL)
        .call()?
        .into_reader();

    if !BURST_STATE_ROOT.exists() {
        create_dir_all(BURST_STATE_ROOT.as_path()).context("unable to create state root")?;
    }

    let uber_apk_signer_path = BURST_STATE_ROOT.join(UBER_APK_SIGNER_NAME);
    let mut writer = File::create(&uber_apk_signer_path).context("unable to create uber-apk-signer file")?;
    copy(&mut reader, &mut writer).context("unable to copy uber-apk-signer file")?;

    config.uber_apk_signer = Some(JavaTool {
        version: UBER_APK_SIGNER_VERSION.to_string(),
        path: uber_apk_signer_path,
    });

    info!("Installed uber-apk-signer v{}!", UBER_APK_SIGNER_VERSION);

    Ok(config.uber_apk_signer.as_ref().unwrap())
}

