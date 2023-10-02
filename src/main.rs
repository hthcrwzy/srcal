mod args;
mod calendar;
mod display;
mod weekday;
use std::process::exit;

use chrono::Month;
use clap::Parser;

fn main() {
    let args = args::Args::parse();

    let calendar = if args.month.is_none() {
        calendar::Calendar::now()
    } else {
        match Month::try_from(args.month.unwrap() as u8) {
            Ok(month) => calendar::Calendar::from_month(month),
            Err(_) => {
                eprintln!("Invalid month number specified.");
                exit(1);
            }
        }
    };
    calendar.print_calendar(args.hide_highlight);
}
