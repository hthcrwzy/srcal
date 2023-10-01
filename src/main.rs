mod args;
mod calendar;
mod weekday;
use chrono::{Datelike, NaiveDate};
use clap::Parser;

fn main() {
    let _ = args::Args::parse();

    let calendar = calendar::Calendar::new();
    println!("{} {}", calendar.get_month_with_string(), calendar.year);

    let mut weekday = weekday::Weekday::new();
    for _ in 0..7 {
        print!("{} ", weekday.weekday);
        weekday.next();
    }
    println!();

    let skip = calendar
        .get_weekday_of_first_of_this_month()
        .num_days_from_sunday();
    print!("{}", "    ".repeat(skip as usize));
    let mut counter = skip;

    let mut current_date =
        NaiveDate::from_ymd_opt(calendar.year, calendar.month, calendar.today).unwrap();

    while current_date.month() == calendar.month {
        counter += 1;
        print!("{:>3} ", current_date.day0() + 1);
        current_date = current_date.succ_opt().unwrap();

        if counter % 7 == 0 {
            println!()
        }
    }
    println!();
}
