use cli::{Action, Cli};

mod analytics;
mod cli;

fn main() -> anyhow::Result<()> {
    let cli: Cli = argh::from_env();
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();

    match cli.nested {
        Action::ExtractPngs(info) => cli::extract_pngs(info),
    }
}
