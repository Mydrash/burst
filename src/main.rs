use cli::{Action, Cli};

mod cli;
mod extract_pngs;
mod native;

fn main() -> anyhow::Result<()> {
    let cli: Cli = argh::from_env();
    env_logger::builder().filter_level(log::LevelFilter::Info).parse_default_env().init();

    match cli.nested {
        Action::ExtractPngs(info) => extract_pngs::extract_pngs(info),
    }
}
