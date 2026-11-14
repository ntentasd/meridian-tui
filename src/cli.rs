use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// URL of the Meridian instance
    ///
    /// Example: https://meridian.company.org
    #[arg(short, long)]
    pub url: String,

    /// Print all routes and exit
    ///
    /// Routes are written to stdout, one per line, without starting the TUI.
    #[arg(short, long)]
    pub dump: bool,
}
