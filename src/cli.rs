use clap::Parser;

/// Game Mode
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Wether to disable execute custom command or not
    #[clap(short, long, display_order = 1)]
    pub cmd: bool,

    /// Wether to disable fan speed or not
    #[clap(short, long,display_order = 2)]
    pub nbfc: bool,
}
