use crate::{calendar::Calendar, weekday};
use chrono::{Datelike, NaiveDate};

impl Calendar {
    pub fn print_calendar(&self, hide_highlight: bool) {
        self.print_month_and_year();
        println!();

        Self::print_weekday();
        println!();

        Self::print_skip(
            self.get_weekday_of_first_of_this_month()
                .num_days_from_sunday() as usize,
            "    ",
        );
        self.print_month(hide_highlight);
        println!();
    }

    fn print_month_and_year(&self) {
        print!("{} {}", self.get_month_with_string(), self.year);
    }

    fn print_weekday() {
        let mut weekday = weekday::Weekday::new();
        for _ in 0..7 {
            print!("{} ", weekday.weekday);
            weekday.next();
        }
    }

    fn print_skip(times: usize, space: &str) {
        print!("{}", space.repeat(times));
    }

    fn print_month(&self, hide_highlight: bool) {
        let mut counter = self
            .get_weekday_of_first_of_this_month()
            .num_days_from_sunday();

        let mut current_date = NaiveDate::from_ymd_opt(self.year, self.month, 1).unwrap();

        while current_date.month() == self.month {
            counter += 1;
            //print!("\x1b[{}m\x1b[47m{:>3}\x1b[m ", 30, current_date.day());
            let day = current_date.day();
            let highlight = if let Some(today) = self.today {
                if day == today && !hide_highlight {
                    true
                } else {
                    false
                }
            } else {
                false
            };
            Self::print_cell(highlight, day);
            print!(" ");
            current_date = current_date.succ_opt().unwrap();

            if counter % 7 == 0 {
                println!();
            }
        }
    }

    fn print_cell(highlight: bool, day: u32) {
        if highlight {
            print!("\x1b[{}m\x1b[47m{:>3}\x1b[m", 30, day);
        } else {
            print!("{:>3}", day);
        }
    }
}
