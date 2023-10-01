use chrono::{Datelike, Local, Month};

pub struct Calendar {
    pub year: i32,
    pub month: u32,
    pub today: u32,
    pub weekday: chrono::Weekday,
}

impl Calendar {
    pub fn new() -> Calendar {
        let now = Local::now();
        Calendar {
            year: now.year(),
            month: now.month(),
            today: now.day(),
            weekday: now.weekday(),
        }
    }

    pub fn get_month_with_string(&self) -> &str {
        Month::try_from(self.month as u8).unwrap().name()
    }

    pub fn get_weekday_of_first_of_this_month(&self) -> chrono::Weekday {
        chrono::NaiveDate::from_ymd_opt(self.year, self.month, 1)
            .unwrap()
            .weekday()
    }
}
