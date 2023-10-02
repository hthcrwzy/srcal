use clap::Parser;

/// displays a calendar.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// disable highlighting of today.
    #[arg(short = 'x')]
    pub hide_highlight: bool,
    /// specify a month.
    #[arg(short = 'm')]
    pub month: Option<u32>,
}
