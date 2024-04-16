use super::extract_pngs::ExtractPngs;
use argh::FromArgs;

#[derive(FromArgs, Clone, Debug)]
/// the ultimate minecraft modification tool
pub(crate) struct Cli {
    #[argh(subcommand)]
    pub nested: Action,
}

#[derive(FromArgs, Clone, Debug)]
#[argh(subcommand)]
pub enum Action {
    ExtractPngs(ExtractPngs),
}
