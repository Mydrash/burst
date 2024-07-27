use argh::FromArgs;

mod extract_pngs;
mod fastzip;
mod x;


pub use x::execute;
pub use fastzip::fastzip;
pub use extract_pngs::extract_pngs;
use extract_pngs::ExtractPngs;

#[derive(FromArgs, Clone, Debug)]
/// the ultimate modification tool
pub(crate) struct Cli {
    #[argh(subcommand)]
    pub nested: Action,
}

#[derive(FromArgs, Clone, Debug)]
#[argh(subcommand)]
pub enum Action {
    ExtractPngs(ExtractPngs),
    FastZip(fastzip::FastZip),
    X(x::Execute),
}
