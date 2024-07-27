use std::sync::LazyLock;
use directories::ProjectDirs;
use std::path::PathBuf;

pub mod config;
pub mod state;

pub static DIRS: LazyLock<ProjectDirs> = LazyLock::new(|| {
    ProjectDirs::from("com", "ryster", "burst").expect("unable to find project directories")
});
pub static BURST_CONFIG_ROOT: LazyLock<PathBuf> = LazyLock::new(|| {
    DIRS.config_dir().to_path_buf()
});
pub static BURST_STATE_ROOT: LazyLock<PathBuf> = LazyLock::new(|| {
    DIRS.data_dir().to_path_buf()
});