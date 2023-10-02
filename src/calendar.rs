use chrono::{Datelike, Local, Month, NaiveDate, Weekday};

pub struct Calendar {
    pub year: i32,
    pub month: u32,
    pub today: Option<u32>,
    pub weekday: Option<Weekday>,
}

impl Calendar {
    pub fn now() -> Calendar {
        let now = Local::now();
        Calendar {
            year: now.year(),
            month: now.month(),
            today: Some(now.day()),
            weekday: Some(now.weekday()),
        }
    }

    pub fn from_month(month: Month) -> Calendar {
        let now = Local::now();
        Calendar {
            year: now.year(),
            month: month.number_from_month(),
            today: None,
            weekday: None,
        }
    }

    pub fn get_month_with_string(&self) -> &str {
        Month::try_from(self.month as u8).unwrap().name()
    }

    pub fn get_weekday_of_first_of_this_month(&self) -> Weekday {
        NaiveDate::from_ymd_opt(self.year, self.month, 1)
            .unwrap()
            .weekday()
    }
}
