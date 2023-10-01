use clap::Parser;

/// displays a calendar.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
}